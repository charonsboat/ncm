#[macro_use] // implement macros from clap to allow loading from YAML
extern crate clap;

use clap::App;
use std::io::prelude::*;
use std::fs::{File};
use std::path::{Path, PathBuf};
use std::io;

fn generate_file(file_path: &PathBuf, file_contents: &str) {
    let mut file = File::create(file_path).unwrap();

    file.write_all(file_contents.as_bytes());
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

            println!("Filename: {}, Document Root: {}, Hostname: {}, Conf Dir: {}", filename, document_root, hostname, conf_dir);

            let conf_path = Path::new(conf_dir);
            let file_path = conf_path.join(filename);

            if conf_path.exists() {
                // conf dir is a directory
                if file_path.exists() {
                    // file exists already
                    // should we overwrite it?
                } else {
                    // no file yet
                    // we're good to go

                    let file_contents = "This is some text";

                    generate_file(&file_path, &file_contents);
                }
            } else {
                // directory doesn't exist
                // should we create it or should there be a 'force create' flag?
                // maybe prompt the user to create it?
            }
        },
        _ => {
            // no command was passed (or an unrecognized command)
        }
    }
}
