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
    user_count: usize,
    want_users: bool,
    admin_email: String,
    description: String,
    users: Vec<User>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

    let mut users_struct = Vec::new();
    users_list.iter().for_each(|user| {
        let path = format!("/home/{}/public_html/index.html", user);
        let path = Path::new(&path);
        let index_file = if let Ok(file) = fs::read_to_string(path) {
            file
        } else {
            return;
        };
        let mut title = String::new();
        index_file.split("\n").for_each(|line| {
            if line.contains("<title>") {
                let title_line = line
                    .split("<title>")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let title_line = title_line
                    .iter()
                    .map(|s| {
                        s.split("</title>")
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>()
                    })
                    .flatten()
                    .collect::<Vec<String>>();
                title_line.iter().for_each(|e| {
                    if !e.contains("<title>") && !e.contains("</title>") {
                        title.push_str(e);
                    }
                })
            }
        });

        let meta = fs::metadata(path);
        let mtime = format!(
            "{}",
            meta.unwrap()
                .modified()
                .unwrap()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        users_struct.push(User {
            name: user.to_string(),
            title,
            mtime,
        });
    });

    let conf = fs::read_to_string(CONF_PATH).expect("Could not read config file");
    let conf_yaml: serde_yaml::Value =
        serde_yaml::from_str(&conf).expect("Could not parse config data as YAML");

    let mut conf_yaml = Server {
        name: conf_yaml["name"].as_str().unwrap().to_string(),
        url: conf_yaml["url"].as_str().unwrap().to_string(),
        signup_url: conf_yaml["signup_url"].as_str().unwrap().to_string(),
        user_count: 0,
        want_users: conf_yaml["want_users"].as_bool().unwrap(),
        admin_email: conf_yaml["admin_email"].as_str().unwrap().to_string(),
        description: conf_yaml["description"].as_str().unwrap().to_string(),
        users: Vec::new(),
    };

    users_struct.iter().for_each(|user| {
        conf_yaml.users.push(user.clone());
    });

    conf_yaml.user_count = users_struct.len();

    let json = serde_json::to_string(&conf_yaml).unwrap();
    fs::write(out_path, &json).unwrap();

    println!("File written successfully! Please inspect it: {}", out_path);
}
