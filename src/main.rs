
#[link(name = "foo")]
extern { fn c_debug(val: i32); }

fn main() {
    println!("Hello, world!");
    unsafe {c_debug(8);}
}
