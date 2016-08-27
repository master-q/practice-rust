use std::ptr;

struct Foo {
    x: i32,
    p: *mut Bar
}
struct Bar {
    x: i32,
    p: *mut Foo
}

fn main () {
    // Init
    let mut foo = Foo { x: 10, p: ptr::null_mut() };
    let mut bar = Bar { x: 20, p: &mut foo};
    foo.p = &mut bar;

    // Print x
    println!("foo.x = {}", foo.x);
    println!("bar.x = {}", bar.x);

    // Deref
    unsafe {
        let foopx = (*foo.p).x;
        let barpx = (*bar.p).x;
        println!("(foo.p).x = {}", foopx);
        println!("(bar.p).x = {}", barpx);
    }
}
