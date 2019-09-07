use serde::{Deserialize, Serialize};

const VERS: &str = "v0.1";
const OUT_PATH: &str = "/var/www/htdocs/tilde.json";

#[derive(Debug, Deserialize, Serialize)]
struct Server {
    name: String,
    url: String,
    signup_url: String,
    user_count: u32,
    want_users: bool,
    admin_email: String,
    description: String,
    users: Vec<User>,
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    title: String,
    mtime: String,
}

fn main() {
    println!("instistats {}", VERS);
    println!("(c) 2019 Ben Morrison - ISC License");
    println!();
    println!("Path: {}", OUT_PATH);
    println!();
}
