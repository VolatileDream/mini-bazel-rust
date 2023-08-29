load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")
load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_vendor")

# https://github.com/bazelbuild/rules_rust/blob/a86313282f2874319b48d3d2efd9a44f9ba46249/examples/crate_universe/vendor_local_pkgs/BUILD.bazel

crates_vendor(
    name = "crates_vendor",
    cargo_lockfile = "Cargo.Bazel.lock",
    annotations = {
    },
    mode = "local",
    packages = {
        "once_cell": crate.spec(
            version = "1.17.1",
        ),
    },
)

