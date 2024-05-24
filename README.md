# Operating Systems Design (2024 Version)

This is an ongoing project to update the labs from Georgia Tech's CS3210, so that they are compatible with later versions of Rust (and an Apple Silicon development machine).

I'm using the following versions of programs for Rust development:

```
rustc 1.80.0-nightly (78a775127 2024-05-11)
rustup 1.27.1 (54dd3d00f 2024-04-24)
cargo-xbuild 0.6.6
cargo-objcopy 0.3.6
```

# Original README

## CS3210 Lab assignments

This repository contains lab assignments for Georgia Tech CS3210 "Design of Operating Systems".
The latest course material is available [here](https://tc.gts3.org/cs3210/2020/spring/index.html).

### Who should take CS3210?

- Anyone wants to work on challenges in operating systems
- Anyone cares about what's going on under the hood
- Anyone has to build high-performance systems (e.g., Cloud, Trading)
- Anyone wants to build embedded/IoT firmware (e.g., Robot)
- Anyone needs to diagnose bugs or security problems

### Why Rust?

Historically, C has been mainly used for OS development because of its portability,
minimal runtime, direct hardware/memory access, and (decent) usability.
Rust provides all of these features with addition of memory safety guarantee,
strong type system, and modern language abstractions
which help programmers to make less mistakes when writing code.

### Acknowledgement

We built our labs based on the materials originally developed for
[CS140e: An Experimental Course on Operating Systems](https://cs140e.sergio.bz/)
by [Sergio Benitez](https://sergio.bz/).
We have ported it to use newer toolchains such as Rust 2018 edition,
`cargo-xbuild` (instead of `xargo`), and `no_std` Rust with a minimal shim library
(instead of custom built std).
We’ve also developed it further to include topics such as virtual memory management, multicore scheduling, mutex designing, and implementing a networking stack.
