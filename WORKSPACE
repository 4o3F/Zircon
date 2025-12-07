workspace(name = "Zircon")

# ----------------------------
# Rust support
# ----------------------------
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# rules_rust
http_archive(
    name = "rules_rust",
    url = "https://github.com/bazelbuild/rules_rust/releases/download/0.26.0/rules_rust-0.26.0.tar.gz",
    sha256 = "c997c6e72c0e9843f79e57f55b6f87d3e3d97c9a3d0f7c8271b63f5fbb6e8bfa",
)
load("@rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()

# ----------------------------
# Python support
# ----------------------------
http_archive(
    name = "rules_python",
    url = "https://github.com/bazelbuild/rules_python/releases/download/0.26.0/rules_python-0.26.0.tar.gz",
    sha256 = "4e33d1df8f1030e1d7fc1020d7f0cd3de290e97cb518330fa9cc63da3084bdfc",
)
load("@rules_python//python:repositories.bzl", "py_repositories")
py_repositories()

# ----------------------------
# Node.js support (frontend)
# ----------------------------
http_archive(
    name = "rules_nodejs",
    url = "https://github.com/bazelbuild/rules_nodejs/releases/download/5.1.0/rules_nodejs-5.1.0.tar.gz",
    sha256 = "86e4f7031a6fcfdfdfc2da155ea1c5bda1b1f8453d1a8f8c04a5a8fda9986b7b",
)
load("@rules_nodejs//nodejs:repositories.bzl", "nodejs_repositories")
nodejs_repositories()

# Initialize Yarn/npm for frontend (optional)
load("@rules_nodejs//nodejs:defs.bzl", "yarn_install")
yarn_install(
    name = "npm_deps",
    package_json = "//frontend:package.json",
    yarn_lock = "//frontend:yarn.lock",
)
