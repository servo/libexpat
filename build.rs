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

    let mut dst = cmake::Config::new("expat")
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
