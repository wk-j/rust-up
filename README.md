## Rust Up

```
rustup self update
rustup update nightly

rustup component add rls-preview --toolchain nightly
rustup component add rust-analysis --toolchain nightly
rustup component add rust-src --toolchain nightly

cargo install cargo-script
```


```
rustup override set nightly-2017-10-24
rustup component add rls-preview --toolchain nightly-2017-10-24
rustup component add rust-analysis --toolchain nightly-2017-10-24
rustup component add rust-src --toolchain nightly-2017-10-24
```