extern crate cheddar;

fn main() {
    println!("cargo:rustc-flags=-l dylib=ssl");
    println!("cargo:rustc-flags=-l dylib=stdc++");
    println!("cargo:rustc-flags=-l dylib=uv");

    println!("cargo:rustc-flags=-l dylib=crypto");
    println!("cargo:rustc-flags=-l dylib=cassandra");

    println!("cargo:rustc-link-search={}", "/usr/lib/");

    // the debian binary packages of the cpp driver install to /usr/lib/x86_64-linux-gnu/
    println!("cargo:rustc-link-search={}", "/usr/lib/x86_64-linux-gnu/");

    // on osx, libuv lives in /usr/local/lib
    println!("cargo:rustc-link-search={}", "/usr/local/lib/");

    // on osx, cpp driver is installed to /usr/local/lib64
    println!("cargo:rustc-link-search={}", "/usr/local/lib64/");
    //println!("cargo:rustc-link-lib=static=cassandra_static");
    cheddar::Cheddar::new().unwrap()
        .run_build("target/include/cassandra_rs.h");
}
