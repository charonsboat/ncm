# nxconf

A full-featured Nginx configuration management tool (built in Rust).


## Installation

### From Source

Dependencies:

- **Rust 1.11.0**

To install `nxconf` from source, just run the following commands:

```bash
# download the project
git clone https://github.com/github/hub.git && cd hub

# assuming `~/bin` is in your PATH:
cargo build --release
cp target/release/nxconf ~/bin/nxconf
```

Note: There is a Vagrant environment available with all of the necessary build tools (just run `vagrant up`).


## Meta

- Home: <https://github.com/drm2/nxconf>
- Bugs: <https://github.com/drm2/nxconf/issues>
- Authors: <https://github.com/drm2/nxconf/graphs/contributors>

When filing a new bug, please make sure one doesn't already exist for your problem by [searching the issues](https://github.com/drm2/nxconf/issues). Make sure you include any useful details such as error messages, OS details, binary version, Nginx version, etc.
