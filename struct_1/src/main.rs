use std::fmt;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(username: {}, email: {}, sign_in_count: {}, active: {})", self.username, self.email, self.sign_in_count, self.active)
    }
}

fn main() {
    let user1 = User {
        username: String::from("someone@email.com"),
        email: String::from("someone@email.com"),
        active: true,
        sign_in_count: 1
    };
    // user1.email = String::from("john.doe@email.com");
    // ^^^^^^^^^^^ cannot assign
    
    println!("user1: {}", format!("{}", user1));

    let mut user2 = User {
        username: String::from("john.doe@email.com"),
        email: String::from("john.doe@email.com"),
        active: true,
        sign_in_count: 10
    };
    println!("user2: {}", format!("{}", user2));
    user2.email = String::from("john.doe@gmail.com");
    println!("user2: {}", format!("{}", user2));

}
