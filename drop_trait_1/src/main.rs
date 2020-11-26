struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn main() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let data: String = String::from("other stuff");
    let _d = CustomSmartPointer {
        data,
    };
    println!("CustomSmartPointer created.");
}
