load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test")

rust_library(
    name = "bitpacking",
    srcs = ["bitpacking.rs"],
    visibility = ["//bits:__subpackages__"],
    deps = [
        "//bits",
        "//num",
    ],
)

rust_test(
    name = "bitpacking_unit_test",
    crate = ":bitpacking",
)

rust_doc_test(
    name = "bitpacking_doc_test",
    crate = ":bitpacking",
)
