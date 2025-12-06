# 環境構築

rustupをインストール:
https://rust-lang.org/ja/tools/install/

もしくはnixを使う

```bash
# このテンプレートを再現したい場合
cargo init
cargo add clap --features derive
cargo add ratatui crossterm
cargo install --locked bacon
```

# 開発用コマンド

```bash
# build
cargo build
cargo build --release

# watch mode
bacon

# execute
cargo run
./target/debug/cli-rust-template
./target/release/cli-rust-template
```

## 参考

- https://doc.rust-jp.rs/book-ja/
- https://ratatui.rs/
- https://docs.rs/clap/latest/clap/
- https://docs.rs/crossterm/latest/crossterm/
