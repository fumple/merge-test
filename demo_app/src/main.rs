mod models {
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
}

fn main() {
    println!("Hello, world!");
}
