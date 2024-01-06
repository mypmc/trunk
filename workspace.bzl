load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive", "http_file")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")

_RUST_EDITION = "2021"

_RUST_STABLE_VERSION = "1.75.0"

# https://github.com/oxalica/rust-overlay/tree/master/manifests/nightly
_RUST_NIGHTLY_VERSION = "nightly/2024-01-06"

_GO_VERSION = "1.21.1"

versions = struct(
    rust = struct(
        edition = _RUST_EDITION,
        versions = [
            _RUST_STABLE_VERSION,
            _RUST_NIGHTLY_VERSION,
        ],
    ),
    go = struct(
        version = _GO_VERSION,
    ),
)

_http_archives = {
    "platforms": {
        "sha256": "3a561c99e7bdbe9173aa653fd579fe849f1d8d67395780ab4770b1f381431d51",
        "urls": [
            "https://mirror.bazel.build/github.com/bazelbuild/platforms/releases/download/0.0.7/platforms-0.0.7.tar.gz",
            "https://github.com/bazelbuild/platforms/releases/download/0.0.7/platforms-0.0.7.tar.gz",
        ],
    },
    "aspect_bazel_lib": {
        "sha256": "09b51a9957adc56c905a2c980d6eb06f04beb1d85c665b467f659871403cf423",
        "strip_prefix": "bazel-lib-1.34.5",
        "url": "https://github.com/aspect-build/bazel-lib/releases/download/v1.34.5/bazel-lib-v1.34.5.tar.gz",
    },
    "bazel_skylib": {
        "sha256": "66ffd9315665bfaafc96b52278f57c7e2dd09f5ede279ea6d39b2be471e7e3aa",
        "urls": [
            "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.4.2/bazel-skylib-1.4.2.tar.gz",
            "https://github.com/bazelbuild/bazel-skylib/releases/download/1.4.2/bazel-skylib-1.4.2.tar.gz",
        ],
    },
    "rules_cc": {
        "sha256": "2037875b9a4456dce4a79d112a8ae885bbc4aad968e6587dca6e64f3a0900cdf",
        "strip_prefix": "rules_cc-0.0.9",
        "urls": ["https://github.com/bazelbuild/rules_cc/releases/download/0.0.9/rules_cc-0.0.9.tar.gz"],
    },
    "rules_foreign_cc": {
        "sha256": "2a4d07cd64b0719b39a7c12218a3e507672b82a97b98c6a89d38565894cf7c51",
        "strip_prefix": "rules_foreign_cc-0.9.0",
        "url": "https://github.com/bazelbuild/rules_foreign_cc/archive/refs/tags/0.9.0.tar.gz",
    },
    "zlib": {
        "build_file": "//x/zlib:BUILD.zlib.bazel",
        "sha256": "b3a24de97a8fdbc835b9833169501030b8977031bcb54b3b3ac13740f846ab30",
        "strip_prefix": "zlib-1.2.13",
        "urls": [
            "https://zlib.net/zlib-1.2.13.tar.gz",
            "https://storage.googleapis.com/mirror.tensorflow.org/zlib.net/zlib-1.2.13.tar.gz",
        ],
    },
    "openssl": {
        "build_file": "//x/openssl:BUILD.openssl.bazel",
        "sha256": "cf3098950cb4d853ad95c0841f1f9c6d3dc102dccfcacd521d93925208b76ac8",
        "strip_prefix": "openssl-1.1.1w",
        "urls": [
            "https://mirror.bazel.build/www.openssl.org/source/openssl-1.1.1w.tar.gz",
            "https://www.openssl.org/source/openssl-1.1.1w.tar.gz",
            "https://github.com/openssl/openssl/archive/OpenSSL_1_1_1w.tar.gz",
        ],
    },
    "nasm": {
        "build_file": "//x/openssl:BUILD.nasm.bazel",
        "sha256": "f5c93c146f52b4f1664fa3ce6579f961a910e869ab0dae431bd871bdd2584ef2",
        "strip_prefix": "nasm-2.15.05",
        "urls": [
            "https://mirror.bazel.build/www.nasm.us/pub/nasm/releasebuilds/2.15.05/win64/nasm-2.15.05-win64.zip",
            "https://www.nasm.us/pub/nasm/releasebuilds/2.15.05/win64/nasm-2.15.05-win64.zip",
        ],
    },
    "rules_perl": {
        "sha256": "391edb08802860ba733d402c6376cfe1002b598b90d2240d9d302ecce2289a64",
        "strip_prefix": "rules_perl-7f10dada09fcba1dc79a6a91da2facc25e72bd7d",
        "urls": [
            "https://github.com/bazelbuild/rules_perl/archive/7f10dada09fcba1dc79a6a91da2facc25e72bd7d.tar.gz",
        ],
    },
    "rules_rust": {
        "sha256": "a761d54e49db06f863468e6bba4a13252b1bd499e8f706da65e279b3bcbc5c52",
        "urls": ["https://github.com/bazelbuild/rules_rust/releases/download/0.36.2/rules_rust-v0.36.2.tar.gz"],
    },
    "io_bazel_rules_go": {
        "sha256": "91585017debb61982f7054c9688857a2ad1fd823fc3f9cb05048b0025c47d023",
        "urls": [
            "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.42.0/rules_go-v0.42.0.zip",
            "https://github.com/bazelbuild/rules_go/releases/download/v0.42.0/rules_go-v0.42.0.zip",
        ],
    },
    "rules_proto": {
        "sha256": "dc3fb206a2cb3441b485eb1e423165b231235a1ea9b031b4433cf7bc1fa460dd",
        "strip_prefix": "rules_proto-5.3.0-21.7",
        "urls": [
            "https://github.com/bazelbuild/rules_proto/archive/refs/tags/5.3.0-21.7.tar.gz",
        ],
    },
    "rules_proto_grpc": {
        "sha256": "928e4205f701b7798ce32f3d2171c1918b363e9a600390a25c876f075f1efc0a",
        "strip_prefix": "rules_proto_grpc-4.4.0",
        "urls": ["https://github.com/rules-proto-grpc/rules_proto_grpc/releases/download/4.4.0/rules_proto_grpc-4.4.0.tar.gz"],
    },
    "bazel_gazelle": {
        "sha256": "d3fa66a39028e97d76f9e2db8f1b0c11c099e8e01bf363a923074784e451f809",
        "urls": [
            "https://mirror.bazel.build/github.com/bazelbuild/bazel-gazelle/releases/download/v0.33.0/bazel-gazelle-v0.33.0.tar.gz",
            "https://github.com/bazelbuild/bazel-gazelle/releases/download/v0.33.0/bazel-gazelle-v0.33.0.tar.gz",
        ],
    },
    "com_github_bazelbuild_buildtools": {
        "sha256": "42968f9134ba2c75c03bb271bd7bb062afb7da449f9b913c96e5be4ce890030a",
        "strip_prefix": "buildtools-6.3.3",
        "urls": ["https://github.com/bazelbuild/buildtools/archive/refs/tags/v6.3.3.tar.gz"],
    },
    "rules_pkg": {
        "urls": [
            "https://mirror.bazel.build/github.com/bazelbuild/rules_pkg/releases/download/0.9.1/rules_pkg-0.9.1.tar.gz",
            "https://github.com/bazelbuild/rules_pkg/releases/download/0.9.1/rules_pkg-0.9.1.tar.gz",
        ],
        "sha256": "8f9ee2dc10c1ae514ee599a8b42ed99fa262b757058f65ad3c384289ff70c4b8",
    },
    "rules_oci": {
        "sha256": "a3b6f4c0051938940ccf251a7bdcdf7ac5a93ae00e63ad107c9c6d3bfe20885b",
        "strip_prefix": "rules_oci-1.3.1",
        "url": "https://github.com/bazel-contrib/rules_oci/releases/download/v1.3.1/rules_oci-v1.3.1.tar.gz",
    },
    "container_structure_test": {
        "sha256": "2da13da4c4fec9d4627d4084b122be0f4d118bd02dfa52857ff118fde88e4faa",
        "strip_prefix": "container-structure-test-1.16.0",
        "urls": ["https://github.com/GoogleContainerTools/container-structure-test/archive/v1.16.0.zip"],
    },
}

_http_files = {
    "target-determinator_linux_x86_64": {
        "executable": True,
        "sha256": "5200dbca0dd4980690d5060cf8e04abac927efaca143567c51fe24cf973364d2",
        "url": "https://github.com/bazel-contrib/target-determinator/releases/download/v0.23.0/target-determinator.linux.amd64",
    },
    "target-determinator_macos_x86_64": {
        "executable": True,
        "sha256": "aba6dce8a978d2174b37dd1355eecba86db93be1ff77742d0753d8efd6a8a316",
        "url": "https://github.com/bazel-contrib/target-determinator/releases/download/v0.23.0/target-determinator.darwin.amd64",
    },
    "target-determinator_macos_arm64": {
        "executable": True,
        "sha256": "6c3c308dcfc651408ed5490245ea3e0180fc49d4cc9b762ab84a4b979bcb07b8",
        "url": "https://github.com/bazel-contrib/target-determinator/releases/download/v0.23.0/target-determinator.darwin.arm64",
    },
    "target-determinator-driver_linux_x86_64": {
        "executable": True,
        "sha256": "28b6570c637a99c78ee53524967bc7e0fd7b16a29fbfb9d081fbc214a56ea0f6",
        "url": "https://github.com/bazel-contrib/target-determinator/releases/download/v0.23.0/driver.linux.amd64",
    },
    "target-determinator-driver_macos_x86_64": {
        "executable": True,
        "sha256": "6c9e30f3207f592de588c8cdaa90b53577ec38d3ff48c9a00c8d8b0217ecc990",
        "url": "https://github.com/bazel-contrib/target-determinator/releases/download/v0.23.0/driver.darwin.amd64",
    },
    "target-determinator-driver_macos_arm64": {
        "executable": True,
        "sha256": "41b957c671c4a3cdef83a9cda66eb1042fa1e7b00cc0e5eec9640579ad841f9f",
        "url": "https://github.com/bazel-contrib/target-determinator/releases/download/v0.23.0/driver.darwin.arm64",
    },
}

_toolchains = [
    "//toolchains:prost_toolchain",
]

def workspace_dependencies():
    for name in _http_archives:
        maybe(http_archive, name, **_http_archives[name])

    for name in _http_files:
        maybe(http_file, name, **_http_files[name])

# buildifier: disable=unnamed-macro
def workspace_toolchains():
    for tc in _toolchains:
        native.register_toolchains(tc)
