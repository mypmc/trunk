load("@rules_rust//rust:defs.bzl", "rust_library")

package(default_visibility = ["//process:__subpackages__"])

rust_library(
    name = "fsutil",
    srcs = ["fsutil.rs"],
    deps = [
        "@crates//:futures",
        "@crates//:tokio",
    ],
)
