This pulls pinned cargo sources locally to build.

Run this to update:
> bazel run //third-party/rust:crates_vendor

This will create `//third-party/rust/creates` which will contain all the transitive dependencies listed in the BUILD.bazel file.

If you need to update the version of dependencies pulled from Cargo (ie: regenerate `Cargo.Bazel.lock`):
> bazel run //third-party/rust:crates_vendor -- --repin

`--repin` can take arguments about the packages to repin.

> bazel run //third-party/rust:crates_vendor -- --repin once_cell@1.18.0
