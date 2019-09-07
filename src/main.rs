use std::env;
use std::fs;
use std::path::Path;
use std::process;

use serde::{Deserialize, Serialize};
use serde_yaml;
use walkdir::WalkDir;

const VERS: &str = "v0.1";
const CONF_PATH: &str = "instistats.yml";

#[derive(Debug, Deserialize, Serialize)]
struct Server {
    name: String,
    url: String,
    signup_url: String,
    user_count: Option<u32>,
    want_users: bool,
    admin_email: String,
    description: String,
    users: Option<Vec<User>>,
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
    let args = env::args().collect::<Vec<String>>();
    let out_path = match args[1].trim() {
        "-h" | "--help" => {
            println!("The only argument should be the path to save the tilde.json file.\nEx: /var/www/htdocs/tilde.json");
            process::exit(0);
        }
        out_path => {
            println!("Output Location: {}", out_path);
            out_path
        }
    };
    println!();

    let conf = fs::read_to_string(CONF_PATH).expect("Could not read config file");
    let conf_yaml: Server =
        serde_yaml::from_str(&conf).expect("Could not parse config data as YAML");

    eprintln!("{:#?}", conf_yaml);

    let home_dir = WalkDir::new("/home").follow_links(true).max_depth(1);
    let mut users_list = Vec::new();
    home_dir.into_iter().for_each(|d| {
        if let Ok(p) = d {
            let p = p.path().strip_prefix("/home").unwrap();
            let p = p.to_str().unwrap();
            if p.len() > 1 {
                let user = p
                    .chars()
                    .map(|c| {
                        if c != '"' {
                            c.to_string()
                        } else {
                            "".to_string()
                        }
                    })
                    .collect::<String>();
                users_list.push(user);
            }
        }
    });

    eprintln!("{:?}", users_list);
}
