//
// Copyright (c) 2019 Reyk Floeter <contact@reykfloeter.com>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

extern crate glob;
extern crate cc;

use glob::glob;

fn main() {
    let mut files = Vec::new();

    for file in glob("src/*.c").expect("mg") {
        if let Ok(file) = file {
            files.push(file.to_string_lossy().to_string());
        }
    }

    cc::Build::new()
        .files(&files)
        .define("REGEX", Some("1"))
        .flag("-Wno-unused-parameter")
        .flag("-Wno-missing-field-initializers")
        .flag("-Wno-sign-compare")
        .compile("mg");

    println!("cargo:rustc-link-lib=util");
    println!("cargo:rustc-link-lib=curses");
}
