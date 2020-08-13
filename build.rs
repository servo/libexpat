/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate cmake;
extern crate pkg_config;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if !target.contains("android")
        && pkg_config::Config::new()
            .atleast_version("2.1.0")
            .find("expat")
            .is_ok()
    {
        return;
    }

    let mut config = cmake::Config::new("expat");

    let cfg = if target.contains("android") {
        let ndk_root = env::var("ANDROID_NDK_ROOT").or(env::var("ANDROID_NDK_HOME")).expect("`$ANDROID_NDK_ROOT` or `$ANDROID_NDK_ROOT` is not set.");
        let config = config
            .define("CMAKE_TOOLCHAIN_FILE", format!("{}/build/cmake/android.toolchain.cmake", ndk_root));
        if target.starts_with("aarch64") {
            config.define("ANDROID_ABI", "arm64-v8a")
        } else if target.starts_with("armv7") {
            config.define("ANDROID_ABI", "armeabi-v7a")
        } else if target.starts_with("i686") {
            config.define("ANDROID_ABI", "x86")
        } else if target.starts_with("x86_64") {
            config.define("ANDROID_ABI", "x86_64")
        } else {
            config
        }
    } else {
        &mut config
    };

    let mut dst = cfg
        .define("BUILD_shared", "OFF")
        .define("BUILD_tools", "OFF")
        .define("BUILD_examples", "OFF")
        .define("BUILD_tests", "OFF")
        .build();
    dst.push("lib");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=expat");
    println!("cargo:outdir={}", env::var("OUT_DIR").unwrap());
}
