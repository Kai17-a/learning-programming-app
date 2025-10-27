# ファイル監視ツール

指定したディレクトリ内のGoおよびPythonファイルの変更を監視し、変更時に自動実行するツール。

## 機能

- ディレクトリの再帰的監視
- Go/Pythonファイルの変更検知
- ファイル変更時の自動実行

## 使用方法

### インストール

```bash
cargo build --release
```

### 実行

```bash
cargo run -- --dir /path/to/watch/directory
```

または

```bash
./target/release/file-watcher --dir /path/to/watch/directory
```

### オプション

- `--dir`, `-d`: 監視対象ディレクトリのパス（必須）

## 対象ファイル

- `.go` ファイル: `go run` で実行
- `.py` ファイル: `python` で実行

## Git Hooks

コミット前とプッシュ前に以下のチェックを実行する。

- `cargo check`
- `cargo fmt --check`
- `cargo clippy -- -D warnings`

### セットアップ

Windows:
```cmd
setup-hooks.bat
```

Linux/Mac:
```bash
./setup-hooks.sh
```

警告が検出された場合、コミットまたはプッシュは中止される。

## 要件

- Rust 1.70+
- Go（Goファイル実行時）
- Python（Pythonファイル実行時）