load("@rules_rust//rust:defs.bzl", "rust_library")

rust_library(
    name = "protoc_plugin",
    srcs = glob(["src/*.rs"]),
    visibility = ["//protobuf/protoc_plugin:__subpackages__"],
    deps = [
        "@crates//:anyhow",
        "@crates//:prost",
        "@crates//:prost-reflect",
        "@crates//:prost-types",
    ],
)
