# To start

```bash
rustup install nightly
```


# To tag a new version

```bash
cargo +nightly -Zscript ./tag.rs
```

# To generate the documentation

```bash
cargo +nightly -Zscript ./pre-publish.rs
cargo doc
```
