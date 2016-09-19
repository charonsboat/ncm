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

# if you want to build a specific version, checkout that tag
git checkout tags/<version> # e.g. tags/v0.0.1

# assuming `~/bin` is in your PATH:
cargo build --release
cp target/release/ncm ~/bin/ncm
```

Note: There is a Vagrant environment available with all of the necessary build tools (just run `vagrant up`). The Vagrant environment also comes packaged with an Nginx installation for you to start testing `ncm` without affecting your local environment!


## Usage

For usage instructions, just call `ncm --help` once you have the binary in your PATH. It will present you with a list of options and subcommands available.

```
ncm 0.0.1
ncm is a command line tool for nginx configuration management (built in Rust)
David R. Myers II <davidrmyersii@gmail.com>

USAGE:
    ncm [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    create     Creates a new server block configuration file
    disable    Disables an existing configuration file
    enable     Enables an existing configuration file
    help       Prints this message or the help of the given subcommand(s)
    test       Controls testing features
```

You will probably want to call `--help` on each subcommand to see usage options for them as well. Here is an example of the output for `ncm create --help`:

```
ncm-create 0.0.1
Creates a new server block configuration file
David R. Myers II <davidrmyersii@gmail.com>

USAGE:
    ncm create [OPTIONS] <FILE> --root <ROOT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --conf-dir <PATH>    Sets the directory to store the new .conf file [default: /etc/nginx/sites-available]
        --root <ROOT>        Sets the DocumentRoot for the Server block (e.g. /var/www/website/public)
        --hostname <HOST>    Sets the ServerName (hostname) for the Server block [default: _]

ARGS:
    <FILE>    Name of Nginx conf file to create (e.g. website.conf)
```


## Meta

- Home: <https://github.com/drm2/ncm>
- Bugs: <https://github.com/drm2/ncm/issues>
- Authors: <https://github.com/drm2/ncm/graphs/contributors>

When filing a new bug, please make sure one doesn't already exist for your problem by [searching the issues](https://github.com/drm2/ncm/issues). Make sure you include any useful details such as error messages, OS details, ncm version, Nginx version, etc.
