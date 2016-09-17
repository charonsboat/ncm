#[macro_use] // implement macros from clap to allow loading from YAML
extern crate clap;

use clap::App;

fn main() {
    let config  = load_yaml!("nxconf.yml");
    let matches = App::from_yaml(config).get_matches();
}
