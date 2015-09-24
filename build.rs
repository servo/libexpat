/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate pkg_config;

use std::process::Command;
use std::env;


fn main() {
    if pkg_config::Config::new().atleast_version("2.1.0").find("expat").is_ok()
    {
        return;
    }

    assert!(Command::new("make")
        .args(&["-f", "makefile.cargo"])
        .status()
        .unwrap()
        .success());
    let out_dir = env::var("OUT_DIR").unwrap();
    if env::var("TARGET").unwrap().contains("eabi") {
        println!("cargo:rustc-link-search=native={}", out_dir);
        println!("cargo:rustc-link-lib=static=expat");
    } else {
        println!("cargo:rustc-link-lib=expat");
    }
    println!("cargo:outdir={}", out_dir);
}
