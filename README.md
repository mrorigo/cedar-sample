# Cedar

Cedar is a small Markdown-to-HTML command-line tool. It is the sample
project used to exercise `rlm-harness` as a supervised repository maintainer.

## Verify the project

```sh
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo audit
```

Run Cedar against a Markdown file:

```sh
cargo run -- README.md
```

The first version intentionally has a small surface. Future issue reports can
add lists, links, code blocks, front matter, and better diagnostics.
