# rclrust

<img src="https://user-images.githubusercontent.com/25898373/131146249-36f349ba-ce33-462d-89f8-40bfa1a9899f.png" width="200px" alt="rclrust's logo"/>

[![test](https://github.com/rclrust/rclrust/actions/workflows/test.yaml/badge.svg?branch=main)](https://github.com/rclrust/rclrust/actions/workflows/test.yaml)
[![doc](https://github.com/rclrust/rclrust/actions/workflows/doc.yaml/badge.svg?branch=main)](https://github.com/rclrust/rclrust/actions/workflows/doc.yaml)
[![rclrust](https://img.shields.io/crates/v/rclrust.svg)](https://crates.io/crates/rclrust)
[![codecov](https://codecov.io/gh/rclrust/rclrust/branch/main/graph/badge.svg)](https://codecov.io/gh/rclrust/rclrust)
[![dependency status](https://deps.rs/repo/github/rclrust/rclrust/status.svg)](https://deps.rs/repo/github/rclrust/rclrust)
[![Rust 1.53](https://img.shields.io/badge/rust-1.53+-blue.svg)](https://blog.rust-lang.org/2021/06/17/Rust-1.53.0.html)
[![Apache-2.0](https://img.shields.io/github/license/rclrust/rclrust)](https://github.com/rclrust/rclrust/blob/main/LICENSE)

This is yet another ROS2 client library written in Rust.  
I have implemented it independent of the ament or colcon.
By using proc-macro to generate message-type and service-type code, crate dependency resolution can now be completed in `cargo`. This was inspired by [rosrust](https://github.com/adnanademovic/rosrust)

## Supporting Environments

- Rust: 1.56+
- ROS2:
  - Foxy ([doc](https://rclrust.github.io/rclrust/foxy/main/rclrust/index.html))
  - Galctic ([doc](https://rclrust.github.io/rclrust/galactic/main/rclrust/index.html))
  - Rolling ([doc](https://rclrust.github.io/rclrust/rolling/main/rclrust/index.html))
- OS:
  - Ubuntu
- DDS:
  - Fast DDS (Default for Foxy)
  - Cyclone DDS (Default for Galactic+)

## Supporting features

- Code generation from `.msg`, `.srv`, `.action`
- Loggers
- Publishers/Subscriptions
- Services/Clients
- Timers
- Parameters (without services)

## TODO

- Parameter services/clients
- Actions
- Lifecycles
- More
  - Unit test
  - Documentation
  - Examples (especially with ament)
- etc...

## Distribution

RclRust is supporting multiple distributions by using [cargo features](https://doc.rust-lang.org/cargo/reference/features.html).
If you use a fixed distribution, set `features` in `Cargo.toml` as follows.

```toml
rclrust = { git = "https://github.com/rclrust/rclrust.git", features = ["foxy"] }
```

Otherwise, do not set `features` in `Cargo.toml` and pass target features like `--featuers rclrust/${ROS_DISTRO}` on build.

```toml
rclrust = { git = "https://github.com/rclrust/rclrust.git" }
```

## Examples

### Prepare

```sh-session
$ git clone git@github.com:rclrust/rclrust.git
$ cd rclrust
$ cargo build
```

### Pub/Sub

Publisher:

```sh-session
$ cargo run --features <distro> --examples publisher
```

Subscription

```sh-session
$ cargo run --features <distro> --examples subscription
```

![out](https://user-images.githubusercontent.com/25898373/128894819-f925b31f-d814-4046-a328-68bfe854d03b.gif)

Other examples are [here](https://github.com/rclrust/rclrust/tree/main/rclrust/examples), and examples with colcon are [here](https://github.com/rclrust/rclrust-examples).

## Notice

The icon of RclRust was created by combinating and modifing the following materials.

- [ros.svg](https://github.com/ros-infrastructure/artwork/blob/master/orgunits/ros.svg) © ROS (Licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/))
- [Gear-icon.png](https://commons.wikimedia.org/wiki/File:Gear-icon.png) (Licensed under [CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/deed.en))

## The other ROS2 clients written in Rust

- [ros2_rust](https://github.com/ros2-rust/ros2_rust)
- [r2r](https://github.com/sequenceplanner/r2r)
