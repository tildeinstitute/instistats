use chrono::prelude::*;
use std::env;
use std::fs;
use std::process;

use serde::{Deserialize, Serialize};
use serde_yaml;
use walkdir::WalkDir;

const VERS: &str = "v0.1-only_user_count";
const CONF_PATH: &str = "instistats.yml";

#[derive(Debug, Deserialize, Serialize)]
struct Server {
    name: String,
    url: String,
    signup_url: String,
    user_count: usize,
    want_users: bool,
    admin_email: String,
    description: String,
    last_generated: String,
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

    let home_dir = WalkDir::new("/home").follow_links(true).max_depth(1);
    let user_count = fs::read_dir("/home")
        .unwrap()
        .map(|d| d.unwrap())
        .collect::<Vec<fs::DirEntry>>()
        .len();
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

    let conf = fs::read_to_string(CONF_PATH).expect("Could not read config file");
    let conf_yaml: serde_yaml::Value =
        serde_yaml::from_str(&conf).expect("Could not parse config data as YAML");

    let last_generated = Utc::now().to_rfc2822();

    let mut conf_yaml = Server {
        name: conf_yaml["name"].as_str().unwrap().to_string(),
        url: conf_yaml["url"].as_str().unwrap().to_string(),
        signup_url: conf_yaml["signup_url"].as_str().unwrap().to_string(),
        user_count: 0,
        want_users: conf_yaml["want_users"].as_bool().unwrap(),
        admin_email: conf_yaml["admin_email"].as_str().unwrap().to_string(),
        description: conf_yaml["description"].as_str().unwrap().to_string(),
        last_generated,
    };

    conf_yaml.user_count = user_count;

    let json = serde_json::to_string(&conf_yaml).unwrap();
    fs::write(out_path, &json).unwrap();

    println!("File written successfully! Please inspect it: {}", out_path);
}
