load("@rules_rust//rust:defs.bzl", "rust_doc", "rust_doc_test", "rust_library", "rust_test", "rust_test_suite")

rust_library(
    name = "fenwicktree",
    srcs = [
        "fenwicktree.rs",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "//num",
    ],
)

rust_test_suite(
    name = "tests",
    srcs = glob(["*_test.rs"]),
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    rustc_flags = [
        "-Coverflow-checks=off",
    ],
    deps = [
        ":fenwicktree",
        "//num",
        "@crates//:quickcheck",
    ],
)

rust_test(
    name = "cfg_test",
    crate = ":fenwicktree",
)

rust_doc_test(
    name = "doc_test",
    crate = ":fenwicktree",
)

rust_doc(
    name = "doc",
    crate = ":fenwicktree",
)
