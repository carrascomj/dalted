![alt text](./static/img/logo.svg "Dalted logo black")  
![build](https://github.com/carrascomj/dalted/workflows/build/badge.svg?branch=master)

# What is Dalted?

Web-app hosted at https://dalted.tech to simulate different types of color blindness from an user-provided input image.

## Resources

1. The color blindness simulation implementation was inspired by [Vienot et al., 1999](http://vision.psychol.cam.ac.uk/jdmollon/papers/colourmaps.pdf).
2. This simulation is explained in this [awesome post](https://ixora.io/projects/colorblindness/color-blindness-simulation-research/).
3. Back-end written in [actix-web](https://actix.rs/).
4. Backbone and guide for building a [Rust-Web-App](https://github.com/steadylearner/Rust-Web-App/).

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Run locally

The first step is to [install Rust](https://www.rust-lang.org/tools/install):

```bash
# Unix-like OS
curl https://sh.rustup.rs -sSf | sh
```

After cloning this repository, it can be run in the browser with [cargo](https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html):

```bash
git clone https://github.com/carrascomj/dalted.git
# enable --release for optimized compilation (but slower build process)
cargo run #--release
```
