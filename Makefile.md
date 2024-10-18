# all

* clippy
* test
* build
* doc

# check

* outdated
* audit

# update

* update-toml
* update-lock

# run

* `target/release/{dirname}`

```
target/release/{dirname}
```

# clippy

* `Cargo.lock`
* `Cargo.toml`
* `src/**/*.rs`

```
cargo clippy -- -D clippy::all
```

# test

* `Cargo.lock`
* `Cargo.toml`
* `src/**/*.rs`

```
cargo test --release
```

# build

* `target/release/{dirname}`

# `target/release/{dirname}`

* `Cargo.lock`
* `Cargo.toml`
* `src/**/*.rs`
* `README.md`

```
cargo build --release
```

# doc

```
cargo doc
```

# outdated

```
cargo outdated --exit-code=1
```

# audit

```
cargo audit
```

# update-toml

```
cargo upgrade -i
```

# update-lock

```
cargo update
```

# install-deps

```
cargo install cargo-audit cargo-edit cargo-outdated cocomo dtg kapow tokei toml-cli
```

# clean

```
cargo clean
```

# cocomo

```bash -eo pipefail
tokei; echo
cocomo -o sloccount
cocomo
```

# publish

```
cargo publish
git push
git push --tags
```

# full

* update
* check
* all
* install

