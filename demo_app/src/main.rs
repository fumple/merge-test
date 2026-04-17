mod models {
    use std::fmt::Display;

    pub struct User {
        username: String,
        password: String,
    }

    impl User {
        pub fn new(username: String, password: String) -> Self {
            User { username, password }
        }
        pub fn username(&self) -> &String {
            &self.username
        }
        pub fn password(&self) -> &String {
            &self.password
        }
    }

    impl Display for User {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("User {}", self.username))
        }
    }
}

fn main() {
    println!("Hello, world!");
}
