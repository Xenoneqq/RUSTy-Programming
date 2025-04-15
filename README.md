# 🦀 RUSTy Programming 🦀
This repository contains solutions to college Rust programming exercises as well as some personal projects. The main goal is to learn Rust and build interesting projects along the way.

## 🛠️ How to Compile and Run

To run the programs, you need to have the Rust compiler installed. On Linux, you can install it using the following command:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, you can compile any ``.rs`` file using:

```sh
rustc program_name.rs
```

This will produce a binary you can run with:

```sh
./program_name
```

Some projects use cargo (rust build system). To run them you will need to enter their directory and run the following commands:

```sh
# build and compile the project
cargo build
# run the compiled project
cargo run
```

## 📝 Class Exercises

All class-related exercises can be found in the ``classes`` directory.
Each subfolder corresponds to a different topic covered during the course, and includes its own ``lab`` folder with practical assignments.
