# 🛠️ Contributing

The usual process to make a contribution is to:

1. Check for existing related [issues on GitHub](https://github.com/biopragmatics/curies.rs/issues)
2. [Fork](https://github.com/biopragmatics/curies.rs/fork) the repository and create a new branch
3. Make your changes
4. Make sure formatting, linting and tests passes.
5. Add tests if possible to cover the lines you added.
6. Commit, and send a Pull Request.

## ️🗺️ Architecture details

### 🗃️ Folder structure

```
curies.rs/
├── lib/
│   ├── src/
│   │   └── 🦀 Source code for the core Rust crate.
│   ├── tests/
│   │   └── 🧪 Tests for the core Rust crate.
│   └── docs/
│       └── 📖 Markdown and HTML files for the documentation website.
├── python/
│   └── 🐍 Python bindings for interacting with the Rust crate.
├── js/
│   └── 🌐 JavaScript bindings for integrating into JS environments.
├── scripts/
│   └── 🛠️ Development scripts (build docs, testing).
└── .github/
    └── workflows/
        └── ⚙️ Automated CI/CD workflows.
```

## 🧑‍💻 Development workflow

[![Build](https://github.com/biopragmatics/curies.rs/actions/workflows/build.yml/badge.svg)](https://github.com/biopragmatics/curies.rs/actions/workflows/build.yml) [![Lint and Test](https://github.com/biopragmatics/curies.rs/actions/workflows/test.yml/badge.svg)](https://github.com/biopragmatics/curies.rs/actions/workflows/test.yml) [![codecov](https://codecov.io/gh/biopragmatics/curies.rs/graph/badge.svg?token=BF15PSO6GN)](https://codecov.io/gh/biopragmatics/curies.rs) [![dependency status](https://deps.rs/repo/github/biopragmatics/curies.rs/status.svg)](https://deps.rs/repo/github/biopragmatics/curies.rs)

[Rust](https://www.rust-lang.org/tools/install), python, and NodeJS are required for development.

Install development dependencies:

```bash
# Activate python virtual env
python3 -m venv .venv
source .venv/bin/activate
# Install python dependencies
pip install maturin
# Install rust dev tools
rustup update
rustup component add rustfmt clippy
cargo install wasm-pack cargo-tarpaulin mdbook mdbook-admonish cargo-make
```

### 📥️ Clone the repository

Clone the `curies.rs` repository, `cd` into it, and create a new branch for your contribution:

```bash
cd curies.rs
git checkout -b add-my-contribution
```

###  🧪 Run tests

Run tests for all packages:

```bash
cargo test
```

Display prints:

```bash
cargo test -- --nocapture
```

Run a specific test:

```bash
cargo test new_empty_converter -- --nocapture
```

If tests panic without telling on which test it failed:

```bash
cargo test -- --test-threads=1
```

Test the `curies` crate with code coverage:

```bash
cargo tarpaulin -p curies --out html
```

### 🐍 Run python

Build the pip package, and run the `python/try.py` script:

```bash
./scripts/build-python.sh
```

Or just run the script:

```bash
source .venv/bin/activate
python python/try.py
```

### 🟨 Run JavaScript

Build the npm package:

```bash
./scripts/build-js.py
```

Start a web server:

```bash
python -m http.server 3000 --directory ./js
```

Open [localhost:3000](http://localhost:3000) in your web browser.

### ✨ Format

```bash
cargo fmt
```

### 🧹 Lint

```bash
cargo clippy --all --all-targets --all-features
```

### 📖 Generate docs locally

Install dependencies:

```bash
./scripts/docs-install.sh
```

Build and serve:

```bash
./scripts/docs-serve.sh
```

### 🏷️ New release

Publishing artifacts will be done by the `build.yml` workflow, make sure you have set the following tokens as secrets for this repository: `PYPI_TOKEN`, `NPM_TOKEN`, `CRATES_IO_TOKEN`, `CODECOV_TOKEN`

Install dependency:

```bash
cargo install cargo-outdated
```

1. Make sure dependencies have been updated:

   ```bash
   cargo update
   cargo outdated
   ```

2. Bump the version in the `Cargo.toml` file in folders `lib/`, `python`, `js`

   ```bash
   ./scripts/bump.sh 0.0.2
   ```

3. Commit, push, and create a new release on GitHub

4. The `build.yml` workflow will automatically build artifacts (binary, pip wheel, npm package), and add them to the new release.