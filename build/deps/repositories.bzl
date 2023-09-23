load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies")
load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")
load("@rules_foreign_cc//foreign_cc:repositories.bzl", "rules_foreign_cc_dependencies")
load("@rules_perl//perl:deps.bzl", "perl_register_toolchains", "perl_rules_dependencies")
load("@rules_rust//crate_universe:defs.bzl", "splicing_config")
load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")
load("@rules_rust//proto/prost:repositories.bzl", "rust_prost_dependencies")
load("@rules_rust//proto/prost:transitive_repositories.bzl", "rust_prost_transitive_repositories")
load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")
load("@rules_rust//tools/rust_analyzer:deps.bzl", "rust_analyzer_dependencies")
load("//build/deps:versions.bzl", "GO_VERSION", "RUST_EDITION", "RUST_STABLE_VERSION", "RUST_VERSIONS")
load("//build/deps/crates:defs.bzl", "bin_crates", "lib_crates")
load("@rules_cc//cc:repositories.bzl", "rules_cc_dependencies", "rules_cc_toolchains")

def build_dependencies():
    # Auto-detect toolchain that expects to find tools installed on the host machine.
    # This is non-hermetic, and may have varying behaviors depending on the versions of tools found.
    rules_cc_dependencies()
    rules_cc_toolchains()

    # This sets up some common toolchains for building targets. For more details, please see
    # https://bazelbuild.github.io/rules_foreign_cc/0.9.0/flatten.html#rules_foreign_cc_dependencies
    rules_foreign_cc_dependencies()

    perl_rules_dependencies()
    perl_register_toolchains()

    rules_rust_dependencies()

    rust_register_toolchains(
        edition = RUST_EDITION,
        versions = RUST_VERSIONS,
    )

    # Load the dependencies for the rust-project.json generator tool.
    # n.b., rust_register_toolchains in WORKSPACE ensure a rust_analyzer_toolchain is registered.
    #
    # To regenerate the rust-project.json file:
    #   bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
    rust_analyzer_dependencies()

    # For prost and tonic.
    rust_prost_dependencies()
    rust_prost_transitive_repositories()

    # If the current version of rules_rust is not a release artifact,
    # you may need to set additional flags such as bootstrap = True.
    crate_universe_dependencies()

    bin_crates.dependencies()

    # CARGO_BAZEL_REPIN=1 CARGO_BAZEL_REPIN_ONLY=bin_crates bazel sync --only=bin_crates
    bin_crates.repository()

    # CARGO_BAZEL_REPIN=1 CARGO_BAZEL_REPIN_ONLY=lib_crates bazel sync --only=lib_crates
    lib_crates.repository(
        # The version of Rust the currently registered toolchain is using.
        rust_version = RUST_STABLE_VERSION,
        splicing_config = splicing_config(
            resolver_version = "2",
        ),
    )

    go_rules_dependencies()
    go_register_toolchains(version = GO_VERSION)
    gazelle_dependencies(go_repository_default_config = "//:WORKSPACE.bazel")
