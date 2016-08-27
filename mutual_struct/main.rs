use std::ptr;

struct Bar;
struct Foo {
    x: i32,
    p: *mut Bar
}
struct Bar {
    x: i32,
    p: *mut Foo
}

fn main () {
    let mut foo = Foo { x: 0, p: ptr::null_mut() };
    println!("foo = x:{}", foo.x)
}
