//#![feature(exclusive_range_pattern)]
fn main() {
    let optional:Option<i32> = Some(7);
    match optional {
        Some(i) => {
            println!("This is a really log string and `{:?}`", i);
        },
        _ => {},
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let q = Foo::Qux(100);
    let q2 = Foo::Qux(199);

    if let Foo::Bar = a {
        println!("a is foobar");
    }
    if let Foo::Bar = b {
        println!("b is foobar");
    }
    // if let Foo::Qux(value @ 0..200) = q {
    //     println!("q is {}", value)
    // }
    // error[E0658]: exclusive range pattern syntax is experimental
    // binding with guard
    if let Foo::Qux(value @ 199) = q {
        println!("q is {}", value)
    }
    // binding
    if let Foo::Qux(value) = q2 {
        println!("q2 is {}", value)
    }
}

enum Foo {
    Bar,
    Baz,
    Qux(u32)
}
