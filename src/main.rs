#[macro_use] // implement macros from clap to allow loading from YAML
extern crate clap;

use clap::App;
use std::io::prelude::*;
use std::fs::{File};
use std::path::{Path, PathBuf};

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

fn main() {
    let config  = load_yaml!("nxconf.yml");
    let matches = App::from_yaml(config).get_matches();

    match matches.subcommand() {
        ("create", Some(subc)) => {
            // the 'create' subcommand was used
            let filename = subc.value_of("FILE").unwrap();
            let document_root = subc.value_of("document_root").unwrap();
            let hostname = subc.value_of("hostname").unwrap();
            let conf_dir = subc.value_of("conf_dir").unwrap();

            // println!("Filename: {}, Document Root: {}, Hostname: {}, Conf Dir: {}", filename, document_root, hostname, conf_dir);

            let file_path = Path::new(conf_dir).join(filename);
            let file_contents = format!("\
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

            generate_file(&file_path, &file_contents);
        },
        _ => {
            // no command was passed (or an unrecognized command)
        }
    }
}
