#[macro_use] // implement macros from clap to allow loading from YAML
extern crate clap;
extern crate hyper;

mod interpolater;

use clap::App;
use hyper::client::Client;
use hyper::Url;
use std::io::prelude::*;
use std::fs::{File, remove_file};
use std::path::{Path, PathBuf};
use std::os::unix::fs;

fn main() {
    let config  = load_yaml!("ncm.yml");
    let matches = App::from_yaml(config).get_matches();

    match matches.subcommand() {
        ("create", Some(subc)) => {
            // the 'create' subcommand was used
            let filename = subc.value_of("FILE").unwrap();
            let document_root = subc.value_of("document_root").unwrap();
            let hostname = subc.value_of("hostname").unwrap();
            let conf_dir = subc.value_of("conf_dir").unwrap();

            let file_path = Path::new(conf_dir).join(filename);

            let file_contents = match subc.is_present("template") {
                true => {
                    let template = subc.value_of("template").unwrap();

                    generate_conf_from_template(document_root, hostname, template)
                },
                false => {
                    generate_conf(document_root, hostname)
                }
            };

            generate_file(&file_path, &file_contents);
        },
        ("enable", Some(subc)) => {
            // the 'enable' subcommand was used
            let filename = subc.value_of("FILE").unwrap();
            let source_dir = subc.value_of("available_dir").unwrap();
            let dest_dir = subc.value_of("enabled_dir").unwrap();

            let source_path = Path::new(source_dir).join(filename);
            let dest_path = Path::new(dest_dir).join(filename);

            generate_link(&source_path, &dest_path);
        },
        ("disable", Some(subc)) => {
            // the 'disable' subcommand was used
            let filename = subc.value_of("FILE").unwrap();
            let file_dir = subc.value_of("enabled_dir").unwrap();

            let file_path = Path::new(file_dir).join(filename);

            remove_link(&file_path);
        },
        _ => {
            // no command was passed (or an unrecognized command)
        }
    }
}

fn generate_file(file_path: &PathBuf, file_contents: &str) {

    match File::create(file_path) {
        Ok(mut result) => {
            match result.write_all(file_contents.as_bytes()) {
                Ok(_) => {
                    println!("Success!");
                },
                Err(error) => {
                    println!("Error (write_all): {}", error);
                }
            }
        },
        Err(error) => {
            println!("Error (create): {}", error);
        }
    }
}

fn generate_link(source_path: &PathBuf, dest_path: &PathBuf) {
    match fs::symlink(source_path, dest_path) {
        Ok(_) => {
            // link created successfully
            println!("Successfully enabled conf file.");
        },
        Err(error) => {
            println!("Error (enable): {}", error);
        }
    }
}

fn remove_link(file_path: &PathBuf) {
    match remove_file(file_path) {
        Ok(_) => {
            // successfully removed link
            println!("Link removed successfully.");
        },
        Err(error) => {
            println!("Error (disable): {}", error);
        }
    }
}

fn generate_conf_from_template(document_root: &str, hostname: &str, template_url: &str) -> String {

    let mut conf = String::new();

    match Url::parse(template_url) {
        Ok(url) => {
            // is a Url
            let client = Client::new();

            let mut response = client.get(url).send().unwrap();

            match response.read_to_string(&mut conf) {
                Ok(_) => {
                    // success
                    println!("Success! We read the template via a URL!");
                },
                Err(error) => {
                    // failure
                    println!("Error (conf from template): {}", error);
                }
            }
        },
        Err(error) => {
            match File::open(template_url) {
                Ok(mut file) => {
                    match file.read_to_string(&mut conf) {
                        Ok(_) => {
                            // success
                            println!("Success! We read the template from the filesystem!");
                        },
                        Err(error) => {
                            // error
                        }
                    }
                },
                Err(error) => {
                    // error
                }
            }
        }
    }

    conf = interpolater::interpolate(conf, &[ ("{document_root}", document_root), ("{hostname}", hostname) ]);

    return conf;
}

fn generate_conf(document_root: &str, hostname: &str) -> String {
    let conf = format!("\
server {{
    listen 80;
    listen [::]:80;

    root {document_root};
    index index.html index.htm index.php;

    server_name {hostname};

    location / {{
            try_files $uri $uri/ /index.php?$query_string;
    }}

    # load the index page on 404
    error_page 404 /index.php;

    # don't log requests to favicon.ico
    location = /favicon.ico {{
        log_not_found off;
        access_log    off;
    }}

    # don't log requests to robots.txt
    location = /robots.txt {{
        log_not_found off;
        access_log    off;
    }}

    # pass the PHP scripts to FastCGI server listening on the php-fpm socket
    location ~ .php$ {{
        try_files $uri =404;
        fastcgi_pass unix:/run/php/php-fpm.sock;
        fastcgi_index index.php;
        include fastcgi_params;
        fastcgi_param SCRIPT_FILENAME $document_root$fastcgi_script_name;
        fastcgi_param HTTPS off;
    }}

    # disable access to .htaccess files
    location ~ /.ht {{
        deny all;
    }}

    sendfile off;
}}
", hostname = hostname, document_root = document_root);

    return conf;
}
