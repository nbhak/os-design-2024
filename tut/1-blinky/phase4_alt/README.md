# Phase 4: Rusting Away

## Prerequisites

Ensure LLVM is installed and added to your path. On MacOS:

```
$ brew install llvm
$ echo 'export PATH="/usr/local/opt/llvm/bin:$PATH"' >> ~/.bash_profile
$ source ~/.bash_profile
```

Add necessary dependencies:

```
$ rustup install nightly
$ rustup component add rust-src llvm-tools-preview
$ rustup target add aarch64-unknown-none
```

## Build

Once the code has been modified correctly, you can run the following commands to build the kernel image (in the `phase4_alt` directory).

```
$ cargo clean
$ cargo build --release
$ llvm-objcopy -O binary target/aarch64-unknown-none/release/blinky blinky.bin
$ mv blinky.bin kernel8.img
```
