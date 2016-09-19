# nginx configuration manager

ncm is a command line tool for nginx configuration management (built in Rust).


## Installation

### From Source

Dependencies:

- **Rust 1.11.0**

To install `ncm` from source, just run the following commands:

```bash
# download the project
git clone https://github.com/github/hub.git && cd hub

# assuming `~/bin` is in your PATH:
cargo build --release
cp target/release/ncm ~/bin/ncm
```

Note: There is a Vagrant environment available with all of the necessary build tools (just run `vagrant up`).


## Usage

For usage instructions, just call `ncm --help` once you have the binary in your PATH. It will present you with a list of options and subcommands available.


## Meta

- Home: <https://github.com/drm2/ncm>
- Bugs: <https://github.com/drm2/ncm/issues>
- Authors: <https://github.com/drm2/ncm/graphs/contributors>

When filing a new bug, please make sure one doesn't already exist for your problem by [searching the issues](https://github.com/drm2/ncm/issues). Make sure you include any useful details such as error messages, OS details, ncm version, Nginx version, etc.
