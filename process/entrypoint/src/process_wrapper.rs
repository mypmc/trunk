use std::env;
use std::io;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::PathBuf;
use std::process::{Command as StdCommand, ExitStatus, Stdio};
use std::time::Duration;

use clap::Parser;
use futures::prelude::*;
use tokio::process::{Child, Command};
use tokio::signal::unix::{signal, SignalKind};
use tokio::time;

use crate::fsutil;
use crate::Error::*;
use crate::{Error, Result};

#[derive(Debug, Clone, Parser)]
pub struct ProcessWrapper {
    /// Redirect the child process stdout.
    #[arg(long, value_name = "PATH")]
    stdout: Option<PathBuf>,

    /// Redirect the child process stderr.
    #[arg(long, value_name = "PATH")]
    stderr: Option<PathBuf>,

    /// Environment variables visible to the spawned process.
    #[arg(long = "env", value_name = "KEY")]
    envs: Vec<String>,

    /// Kill the spawned child process after the specified duration.
    #[arg(long, value_name = "DURATION")]
    timeout: Option<humantime::Duration>,

    /// The entrypoint of the child process.
    #[arg(last = true)]
    command: Vec<String>,
}

#[derive(Debug)]
pub struct Process {
    child: Child,
    id: u32,
}

impl ProcessWrapper {
    #[tracing::instrument(
        skip(self),
        fields(
            command = %self.command[0],
        )
    )]
    pub async fn run(self) -> Result {
        let mut interrupt = signal(SignalKind::interrupt()).map_err(Error::Io)?;
        let mut terminate = signal(SignalKind::terminate()).map_err(Error::Io)?;
        let mut process = self.spawn().await?;

        // The procedures below should mostly work, but not perfect because:
        // - it is easy to "escape" from the group
        // - the PID is potentially reused at some point
        //
        // Note that `wait_sync` must be invoked even if the child process exits successfully
        // in order to 1) wait all descendant processes, 2) ensure there are no children left behind.
        tokio::select! {
            biased;
            _ = interrupt.recv() => {},
            _ = terminate.recv() => {},
            _ = process.wait(self.timeout.map(|d| d.into())) => {},
        };
        process.stop(true).await
    }

    async fn spawn(&self) -> Result<Process> {
        // #[cfg(target_os = "linux")]
        // unsafe {
        //     libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
        // }

        let mut cmd = StdCommand::new(self.command[0].as_str());

        cmd.args(&self.command[1..]);

        cmd.stdout(if let Some(path) = self.stdout.as_ref() {
            fsutil::stdio_from(path, false).map_err(Error::Io).await?
        } else {
            Stdio::inherit()
        });

        cmd.stderr(if let Some(path) = self.stderr.as_ref() {
            fsutil::stdio_from(path, false).map_err(Error::Io).await?
        } else {
            Stdio::inherit()
        });

        cmd.env_clear().envs(env::vars().filter(|(key, _)| self.envs.contains(key)));

        // Put the child into a new process group.
        cmd.process_group(0);

        let child = Command::from(cmd).spawn().map_err(NotSpawned)?;
        let id = child.id().expect("fetching the OS-assigned process id");

        Ok(Process { child, id })
    }
}

mod process_impl {
    use super::*;

    impl Drop for Process {
        #[tracing::instrument(skip(self))]
        fn drop(&mut self) {
            let _ = self.kill();
        }
    }

    impl Process {
        pub async fn wait(&mut self, timeout: Option<Duration>) -> Option<Result> {
            match timeout {
                None => Some(self.wait_child().await),
                Some(dur) => time::timeout(dur, self.wait_child()).await.ok(),
            }
        }

        async fn wait_child(&mut self) -> Result {
            match self.child.wait().await {
                Ok(status) => into_process_result(status),
                Err(err) => Err(WaitFailed(err)),
            }
        }

        /// Same as [crate::Process::kill], but in a slightly more graceful way if gracefully is true.
        #[tracing::instrument(skip(self))]
        pub async fn stop(&mut self, gracefully: bool) -> Result {
            if gracefully {
                self.killpg(libc::SIGTERM);
                time::sleep(Duration::from_millis(50)).await;
            }
            self.kill()
        }

        /// Kill the whole process group, and wait the child exits.
        ///
        /// TODO: Wait all descendant processes to ensure there are no children left behind.
        /// Currently, the direct child is the only process to be waited before exiting.
        pub fn kill(&mut self) -> Result {
            self.killpg(libc::SIGKILL);
            self.wait_sync()
        }

        fn wait_sync(&mut self) -> Result {
            loop {
                match self.child.try_wait() {
                    // The exit status is not available at this time.
                    // The child process(es) may still be running.
                    Ok(None) => {
                        continue;
                    }

                    // It is possible for the child process to complete and exceed the timeout
                    // without returning an error.
                    Ok(Some(status)) => break into_process_result(status),

                    // Some error happens on collecting the child status.
                    Err(err) => break Err(WaitFailed(err)),
                }
            }
        }

        fn killpg(&self, signal: libc::c_int) {
            let id = self.id as libc::c_int;
            unsafe {
                let killed = libc::killpg(id, signal);
                tracing::trace!(
                    signal,
                    killed,
                    errno = io::Error::last_os_error().raw_os_error().unwrap_or(0)
                );
            }
        }
    }

    fn into_process_result(status: ExitStatus) -> Result {
        if status.success() {
            Ok(())
        } else if let Some(code) = status.code() {
            Err(ExitedUnsuccessfully(code))
        } else {
            // because `status.code()` returns `None`
            Err(KilledBySignal(status.signal().expect("WIFSIGNALED is true")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process_wrapper<S: Into<String>>(command: Vec<S>) -> ProcessWrapper {
        ProcessWrapper {
            stdout: None,
            stderr: None,
            envs: vec![],
            timeout: None,
            command: command.into_iter().map(|s| s.into()).collect(),
        }
    }

    fn sleep<S: Into<String>>(duration: S) -> ProcessWrapper {
        process_wrapper(vec!["sleep".to_owned(), duration.into()])
    }

    #[tokio::test]
    async fn run_process() {
        assert!(process_wrapper(vec!["date"]).run().await.is_ok());
        assert!(process_wrapper(vec!["unknown_command"]).run().await.is_err());
        assert!(sleep("0.1").run().await.is_ok());
    }
}