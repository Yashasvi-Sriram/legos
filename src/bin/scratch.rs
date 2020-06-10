#[derive(Debug, Copy, Clone)]
struct Foo {
    a: i32,
}

#[derive(Debug, Copy, Clone)]
struct Bar {
    foo: Foo,
}

fn main() {
    // primitives implement clone and copy traits
    let a = 100i32;
    let mut foo = Foo { a: a };
    foo.a = 200i32;
    let mut bar = Bar { foo: foo };
    bar.foo.a = 300i32;
    println!("{:?}", a);
    println!("{:?}", foo);
    println!("{:?}", bar);
}
