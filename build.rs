fn main() {
    println!("cargo:rustc-flags=-l dylib=ssl");    
    println!("cargo:rustc-flags=-l dylib=stdc++");
    println!("cargo:rustc-flags=-l dylib=uv");    
    println!("cargo:rustc-flags=-l dylib=crypto");    
    println!("cargo:rustc-flags=-l dylib=cassandra");    
    println!("cargo:rustc-link-search={}", "/usr/lib/");
//    println!("cargo:rustc-link-lib=static=cassandra_static");
}