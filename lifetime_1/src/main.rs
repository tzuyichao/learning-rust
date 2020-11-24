fn main() {
    println!("Hello, world!");
    let r: u32 = 10;

    {
        let x = 5;
        // r = &x;
        // 7  |         r = &x;
        //    |             ^^ borrowed value does not live long enough
        println!("x: {}", x);
    }

    println!("r: {}", r);

    let str1 = "abcd";
    let str2 = String::from("xyz");
    let result = longest(str1, str2.as_str());
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}