// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use cmake::Config;

fn main() {
    let dst = Config::new("cmake")
        .generator("Unix Makefiles")
        .define("CMAKE_BUILD_TYPE", "Release")
        .no_build_target(true)
        .build();

    println!("cargo:rustc-link-search=native={}", dst.join("build").display());
    println!("cargo:rustc-link-lib=dylib=iwasm");
    println!("cargo:rustc-link-lib=static=vmlib");
    println!("cargo:rustc-link-lib=dylib=dl");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=m");
}
