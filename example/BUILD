load("@rules_rust//rust:defs.bzl", "rust_library", "rust_binary")

rust_binary(
    name = "example",
    srcs = ["main.rs"],
    visibility = ["//visibility:public"],
    deps = [":lib-hello-world"],
)

rust_library(
    name = "lib-hello-world",
    srcs = ["lib.rs"],
)

