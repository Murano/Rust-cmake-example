extern crate  cmake;

fn main(){
    let dst = cmake::build("libfoo");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=foo");
}