use clap::Parser;

mod cmd;

#[derive(Clone, Debug, clap::Parser)]
pub struct Fuzz {
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, clap::Subcommand)]
enum Command {
    Test(cmd::Test),
    Corpus(cmd::Corpus),
}

impl Fuzz {
    async fn run(self) -> anyhow::Result<()> {
        match self.command {
            Command::Test(c) => c.run().await,
            Command::Corpus(c) => c.run().await,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_ansi(false)
        .compact()
        .init();

    Fuzz::parse().run().await
}
