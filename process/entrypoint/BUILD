load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_doc_test", "rust_library", "rust_test")

rust_library(
    name = "entrypoint",
    srcs = glob(
        ["src/**/*.rs"],
        exclude = [
            "src/main.rs",
            "src/bin/**/*",
        ],
    ),
    visibility = ["//visibility:public"],
    deps = [
        "//process",
        "@crates//:thiserror",
        "@crates//:tokio",
        "@crates//:tracing",
    ],
)

rust_test(
    name = "entrypoint_unit_test",
    size = "small",
    crate = ":entrypoint",
    deps = [
        "//testing",
    ],
)

rust_doc_test(
    name = "entrypoint_doc_test",
    size = "small",
    crate = ":entrypoint",
)

rust_binary(
    name = "coordinator",
    srcs = ["src/bin/coordinator.rs"],
    deps = [
        ":entrypoint",
        "//process",
        "//process/fsutil",
        "@crates//:anyhow",
        "@crates//:clap",
        "@crates//:futures",
        "@crates//:tokio",
        "@crates//:tracing",
        "@crates//:tracing-subscriber",
    ],
)