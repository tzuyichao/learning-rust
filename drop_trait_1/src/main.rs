struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let data: String = String::from("other stuff");
    let _d = CustomSmartPointer {
        data,
    };
    {
        let _e = CustomSmartPointer {
            data: String::from("this stuff"),
        };
    }
    println!("CustomSmartPointer created.");
    // c.drop();
    // 25 |     c.drop();
    // |       ^^^^
    // |       |
    // |       explicit destructor calls not allowed
    // |       help: consider using `drop` function: `drop(c)`
    // std::mem::drop
    drop(c);
    println!("after std::mem::drop c");
}
