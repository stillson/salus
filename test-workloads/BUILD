# SPDX-FileCopyrightText: 2023 Rivos Inc.
#
# SPDX-License-Identifier: Apache-2.0

package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_clippy", "rust_library", "rustfmt_test")
load("//:objcopy.bzl", "objcopy_to_bin", "objcopy_to_object")
load("//:lds.bzl", "lds_rule")

objcopy_to_bin(
    name = "tellus_raw_create",
    src = ":tellus",
    out = "tellus_guestvm",
)

objcopy_to_bin(
    name = "guestvm_raw",
    src = ":guestvm",
    out = "guestvm_raw.out",
)

objcopy_to_object(
    name = "guestvm_to_object",
    src = "guestvm_raw.out",
    out = "guestvm.o",
)

lds_rule(
    name = "lds_rule",
    template = "tellus_lds.tmpl",
)

rust_binary(
    name = "tellus",
    srcs = [
        "src/bin/tellus.rs",
        "src/consts.rs",
    ],
    crate_root = "src/bin/tellus.rs",
    compile_data = [":guestvm_to_object", ":lds_rule"],
    data = glob([
        "src/**/*.S",
    ]),
    proc_macro_deps = ["@salus-index//:hex-literal"],
    rustc_flags = [
        "-Ctarget-feature=+v",
        "--codegen=link-arg=-nostartfiles",
        "-Clink-arg=-T$(location :lds_rule)",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "//attestation",
        "//device-tree",
        "//rice",
        "//riscv-regs",
        "//s-mode-utils",
        "//sbi-rs",
        "//sync",
        "//test-system",
        "//test-workloads",
        "@rice-index//:der",
        "@salus-index//:arrayvec",
    ],
)

rust_binary(
    name = "guestvm",
    srcs = [
        "src/bin/guestvm.rs",
        "src/consts.rs",
    ],
    crate_root = "src/bin/guestvm.rs",
    data = glob([
        "src/**/*.S",
        "src/**/*.der",
    ]),
    linker_script = "guest.lds",
    proc_macro_deps = ["@salus-index//:hex-literal"],
    rustc_flags = [
        "-Ctarget-feature=+v",
        "--codegen=link-arg=-nostartfiles",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "//attestation",
        "//device-tree",
        "//rice",
        "//riscv-regs",
        "//s-mode-utils",
        "//sbi-rs",
        "//test-system",
        "//test-workloads",
        "@rice-index//:der",
        "@salus-index//:arrayvec",
    ],
)

rust_library(
    name = "test-workloads",
    srcs = glob(["src/*.rs"]),
    data = glob([
        "src/**/*.S",
        "src/**/*.der",
    ]),
    proc_macro_deps = ["@salus-index//:hex-literal"],
    deps = [
        "//attestation",
        "//device-tree",
        "//rice",
        "//riscv-regs",
        "//s-mode-utils",
        "//sbi-rs",
        "@rice-index//:der",
        "@salus-index//:arrayvec",
    ],
)

rust_clippy(
    name = "clippy",
    deps = [
        "guestvm",
        "tellus",
        "test-workloads",
    ],
)

rustfmt_test(
    name = "rustfmt",
    targets = [
        "test-workloads",
        "tellus",
        "guestvm",
    ],
)
