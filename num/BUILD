load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test", "rust_test_suite")

rust_library(
    name = "num",
    srcs = glob(["src/**/*.rs"]),
    visibility = ["//visibility:public"],
    deps = [
        "//bits",
    ],
)

rust_test(
    name = "num_unit_test",
    crate = ":num",
)

rust_doc_test(
    name = "num_doc_test",
    crate = ":num",
)

rust_test_suite(
    name = "integration",
    srcs = glob(["tests/**/*.rs"]),
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    deps = [
        ":num",
        "//bits",
        "@crates//:quickcheck",
    ],
)
