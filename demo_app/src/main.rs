use crate::controllers::user::sign_up;

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

mod controllers {
    pub mod user {
        use crate::models::User;

        pub fn sign_up(username: String, password: String) -> String {
            let user = User::new(username, password);
            format!("Successfully created user account {}", user).to_string()
        }
    }
}

fn main() {
    println!("Hello, world!");
    println!(
        "Result of sign_up call: {}",
        sign_up("John".to_string(), "React".to_string())
    )
}
