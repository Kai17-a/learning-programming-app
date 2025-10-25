# Learning Programming App

プログラミング学習者向けのファイル監視型自動実行CLIアプリケーション

## 概要

Learning Programming Appは、プログラミング学習を支援するコマンドラインツールです。ファイルを保存すると自動的にコードが実行され、即座に結果を確認できます。エラーが発生してもアプリケーションは継続動作し、学習者の集中を妨げません。

### 主な特徴

- **🚀 自動実行**: ファイル保存と同時にコードを実行
- **📚 セクション別整理**: テーマ別に練習問題を分類
- **🛡️ エラー耐性**: エラーが発生してもCLIが継続動作
- **📊 実行履歴**: 学習の進捗を記録・表示・統計分析
- **⚡ シンプルなCLI**: 直感的で使いやすいコマンド体系
- **🐍 Python対応**: Pythonファイル（.py）の実行をサポート
- **🐹 Go対応**: Golangファイル（.go）の実行をサポート
- **💾 永続化**: SQLiteによる実行履歴の永続保存

## インストール

### 必要な環境

- **Rust**: 1.82 以降（2021 edition対応）
- **Python**: 3.x（Pythonコードの実行用）
- **Go**: 1.18 以降（Golangコードの実行用）
- **OS**: Windows, macOS, Linux

### インストール手順

#### 方法1: ソースからビルド（推奨）

1. **リポジトリのクローン**
```bash
git clone <repository-url>
cd learning-programming-app
```

2. **依存関係の確認**
```bash
# Rustのバージョン確認
rustc --version

# Pythonのバージョン確認
python --version
# または
python3 --version

# Goのバージョン確認
go version
```

3. **アプリケーションのビルド**
```bash
# リリースビルド（推奨）
cargo build --release

# デバッグビルド（開発用）
cargo build
```

4. **動作確認**
```bash
# ヘルプを表示
cargo run -- --help

# サンプル問題で動作確認
cargo run -- sections
```

#### 方法2: バイナリの直接実行

ビルド後、バイナリファイルを直接実行できます：

```bash
# リリースビルドの場合
./target/release/learning-programming-app --help

# デバッグビルドの場合
./target/debug/learning-programming-app --help
```

#### 方法3: システムへのインストール

```bash
# システムにインストール（~/.cargo/bin/に配置）
cargo install --path .

# インストール後は直接実行可能
learning-programming-app --help
```

## 基本的な使用方法

### クイックスタート

```bash
# 1. サンプル問題の監視を開始
cargo run -- watch

# 2. 別のターミナルでファイルを編集
# examples/section1-basics/hello_world.py または hello_world.go を開いて編集・保存

# 3. 自動実行結果を確認
# 保存と同時にコンソールに実行結果が表示されます
```

### 詳細な使用手順

#### 1. アプリケーションの起動

```bash
# デフォルトディレクトリ（./examples）を監視
cargo run -- watch

# 特定のディレクトリを監視
cargo run -- watch ./my-exercises

# 詳細出力モードで監視
cargo run -- watch --verbose

# ビルド済みバイナリを使用する場合
./target/release/learning-programming-app watch
```

#### 2. 学習の流れ

**Step 1: 利用可能なセクションを確認**
```bash
cargo run -- sections
```

**Step 2: 問題ファイルを編集**
- `examples/section1-basics/` または `examples/section2-control-flow/` 内のファイルを開く
- Python（.py）またはGo（.go）ファイルを選択
- お好みのエディタ（VS Code、Vim、Nano等）でコードを編集
- ファイルを保存（Ctrl+S）

**Step 3: 自動実行を確認**
- ファイル保存と同時に実行結果がコンソールに表示される
- エラーが発生しても監視は継続される
- `Ctrl+C` で監視を停止

**Step 4: 学習進捗を確認**
```bash
# 実行履歴を表示
cargo run -- history

# 統計情報を表示
cargo run -- stats
```

#### 3. 単発実行

監視を開始せずに、特定のファイルを一度だけ実行：

```bash
# Pythonファイルを一度だけ実行
cargo run -- run examples/section1-basics/hello_world.py

# Goファイルを一度だけ実行
cargo run -- run examples/section1-basics/hello_world.go

# 詳細出力で実行
cargo run -- run examples/section1-basics/hello_world.py --verbose
```

## コマンドリファレンス

### 基本コマンド

| コマンド | 説明 | オプション | デフォルト値 |
|---------|------|-----------|------------|
| `watch` | ディレクトリの監視を開始 | `-d, --directory <DIR>` | `./examples` |
|         |                        | `-v, --verbose` | false |
| `sections` | 利用可能なセクション一覧を表示 | `-d, --directory <DIR>` | `./examples` |
| `history` | 実行履歴を表示 | `-l, --limit <NUM>` | 10 |
| `stats` | 実行統計を表示 | なし | - |
| `clear` | 実行履歴をクリア | `-f, --force` | false |
| `run` | 特定のファイルを一度実行 | `-v, --verbose` | false |
| `--help` | ヘルプを表示 | なし | - |
| `--version` | バージョンを表示 | なし | - |

### コマンド詳細

#### `watch` - ファイル監視

```bash
# 基本的な使用法
cargo run -- watch

# オプション付きの使用法
cargo run -- watch --directory ./my-exercises --verbose
cargo run -- watch -d ./exercises -v
```

**オプション:**
- `-d, --directory <DIR>`: 監視するディレクトリを指定
- `-v, --verbose`: 詳細な出力を表示（ファイル変更の詳細など）

#### `sections` - セクション一覧

```bash
# デフォルトディレクトリのセクション表示
cargo run -- sections

# 特定のディレクトリのセクション表示
cargo run -- sections --directory ./my-exercises
```

#### `history` - 実行履歴

```bash
# 最新10件の履歴を表示
cargo run -- history

# 最新20件の履歴を表示
cargo run -- history --limit 20
```

#### `stats` - 統計情報

```bash
# 実行統計を表示
cargo run -- stats
```

#### `clear` - 履歴クリア

```bash
# 確認プロンプト付きでクリア
cargo run -- clear

# 確認なしでクリア
cargo run -- clear --force
```

#### `run` - 単発実行

```bash
# Pythonファイルを一度実行
cargo run -- run examples/section1-basics/hello_world.py

# Goファイルを一度実行
cargo run -- run examples/section1-basics/hello_world.go

# 詳細出力で実行
cargo run -- run examples/section1-basics/hello_world.py --verbose
```

## 使用例とサンプル実行

### 基本的な監視開始

```bash
$ cargo run -- watch
👀 Starting to watch directory: examples
✓ Watching for changes... Press Ctrl+C to stop
```

### ファイル変更時の自動実行

**成功例:**
```bash
# hello_world.py を保存した場合
✓ 15:03:00 Executed hello_world.py (86ms)
Hello, World!
こんにちは、学習者さん！
10 + 5 = 15
```

**エラー例（継続動作）:**
```bash
# 構文エラーがある場合
✗ 15:04:12 Executed error_example.py (23ms)
  File "examples/section1-basics/error_example.py", line 2
    print("Hello World"
                       ^
SyntaxError: '(' was never closed
```

### セクション一覧の表示

```bash
$ cargo run -- sections
📚 Available sections in ./examples:
  • section1-basics (3 files)
  • section2-control-flow (3 files)
```

### 実行履歴の表示

```bash
$ cargo run -- history --limit 5
📋 Recent executions (last 5):
  ✓ 2025-01-20 14:30:15 examples/section1-basics/hello_world.py (0.045s) - section1-basics
    Hello, World!
こんにちは、学習者さん！
10 + 5 = 15
  ✓ 2025-01-20 14:28:42 examples/section1-basics/variables.py (0.032s) - section1-basics
    名前: Python学習者
年齢: 25歳
身長: 170.5cm
  ✗ 2025-01-20 14:25:18 examples/section2-control-flow/loops.py (0.021s) - section2-control-flow
    
  ✓ 2025-01-20 14:23:05 examples/section1-basics/input_output.py (0.156s) - section1-basics
    Enter your name: Alice
Hello, Alice!
  ✓ 2025-01-20 14:20:33 examples/section2-control-flow/if_statements.py (0.038s) - section2-control-flow
    Number is positive

# 履歴がない場合
$ cargo run -- history
📋 Recent executions (last 10):
  No execution history found
```

### 統計情報の表示

```bash
$ cargo run -- stats
📊 Execution Statistics:
  Total executions: 15
  Successful: 12 (80.0%)
  Failed: 3 (20.0%)
  Average execution time: 0.041s
  Most executed file: examples/section1-basics/hello_world.py
  Last execution: 2025-01-20 14:30:15

# 履歴がない場合
$ cargo run -- stats
📊 Execution Statistics:
  Total executions: 0
  Successful: 0 (0.0%)
  Failed: 0
```

### 単発実行

```bash
# Python実行例
$ cargo run -- run examples/section1-basics/hello_world.py --verbose
🚀 Executing file: examples/section1-basics/hello_world.py
✓ 15:03:00 Executed hello_world.py (86ms)
Hello, World!
こんにちは、学習者さん！
10 + 5 = 15

# Go実行例
$ cargo run -- run examples/section1-basics/hello_world.go --verbose
🚀 Executing file: examples/section1-basics/hello_world.go
✓ 15:03:15 Executed hello_world.go (124ms)
Hello, World from Go!

# 通常の実行（verbose なし）
$ cargo run -- run examples/section1-basics/hello_world.py
✓ 15:03:00 Executed hello_world.py (86ms)
Hello, World!
こんにちは、学習者さん！
10 + 5 = 15
```

### 履歴のクリア

```bash
$ cargo run -- clear
Are you sure you want to clear all execution history? [y/N]: y
✓ Execution history cleared

# 確認なしでクリア
$ cargo run -- clear --force
✓ Execution history cleared
```

### 詳細出力モード

```bash
$ cargo run -- watch --verbose
👀 Starting to watch directory: examples
ℹ Verbose mode enabled
✓ Watching for changes... Press Ctrl+C to stop

# ファイル変更時
📝 File changed: examples/section1-basics/hello_world.py
✓ 15:03:00 Executed hello_world.py (86ms)
Hello, World!
こんにちは、学習者さん！
10 + 5 = 15
```

## サンプル問題

アプリケーションには学習用のサンプル問題が含まれています。各ファイルには詳細なコメントと段階的な問題が含まれており、ファイルを保存するだけで即座に実行結果を確認できます。

### Section 1: 基礎編 (`examples/section1-basics/`)

#### `hello_world.py` - 基本的な出力と変数（Python版）
```python
# Hello World - 基本的な出力
# このファイルを保存すると自動的に実行されます

# 問題1: "Hello, World!" を出力してください
print("Hello, World!")

# 問題2: あなたの名前を出力してください
name = "学習者"  # ここを自分の名前に変更してください
print(f"こんにちは、{name}さん！")

# 問題3: 簡単な計算結果を出力してください
result = 10 + 5
print(f"10 + 5 = {result}")
```

#### `hello_world.go` - 基本的な出力と変数（Go版）
```go
package main

import "fmt"

func main() {
    // 問題1: "Hello, World!" を出力してください
    fmt.Println("Hello, World!")
    
    // 問題2: あなたの名前を出力してください
    name := "学習者" // ここを自分の名前に変更してください
    fmt.Printf("こんにちは、%sさん！\n", name)
    
    // 問題3: 簡単な計算結果を出力してください
    result := 10 + 5
    fmt.Printf("10 + 5 = %d\n", result)
}
```

#### `variables.py` / `variables.go` - 変数とデータ型
- 文字列、数値、真偽値の基本操作
- 変数の代入と更新
- データ型の確認と変換
- Python版とGo版の両方を提供

#### `input_output.py` - 入力と出力（Python版）
- `input()` 関数を使用したユーザー入力
- 入力データの処理と出力
- 文字列フォーマット

### Section 2: 制御構造編 (`examples/section2-control-flow/`)

#### `if_statements.py` / `if_statements.go` - 条件分岐
- if、elif/else if、else文の使用
- 比較演算子と論理演算子
- ネストした条件分岐
- Python版とGo版の両方を提供

#### `loops.py` / `loops.go` - ループ処理
- for文とwhile文の基本（Goはfor文のみ）
- range()関数の使用（Python）、range構文（Go）
- ループの制御（break、continue）
- Python版とGo版の両方を提供

#### `functions.py` / `functions.go` - 関数
- 関数の定義と呼び出し
- 引数と戻り値
- ローカル変数とグローバル変数
- Python版とGo版の両方を提供

### 学習の進め方

1. **ファイルを開く**: お好みのエディタで問題ファイルを開く
2. **コードを編集**: コメントの指示に従ってコードを修正
3. **保存**: ファイルを保存（Ctrl+S）
4. **結果確認**: 自動実行された結果をコンソールで確認
5. **繰り返し**: エラーが出た場合は修正して再保存

**ヒント**: エラーが発生してもアプリケーションは停止しません。エラーメッセージを読んで修正し、再度保存してください。

## プロジェクト構造

```
learning-programming-app/
├── src/                    # ソースコード
│   ├── cli/               # CLIインターフェース
│   ├── core/              # コア機能（監視、実行、履歴）
│   ├── handlers/          # 言語ハンドラー
│   ├── utils/             # ユーティリティ
│   └── main.rs            # エントリーポイント
├── tests/                 # テストファイル
├── examples/              # サンプル問題
│   ├── section1-basics/   # 基礎編（Python & Go）
│   └── section2-control-flow/ # 制御構造編（Python & Go）
├── Cargo.toml            # プロジェクト設定
├── Cargo.lock            # 依存関係ロック
└── README.md             # このファイル
```

## 設定とカスタマイズ

### サポートされている言語

現在サポートされている言語と拡張子：
- **Python**: `.py` ファイル（`python` コマンドを使用）
- **Go**: `.go` ファイル（`go run` コマンドを使用）

### 実行環境の設定

#### Pythonコマンドの設定
システムによってPythonコマンドが異なる場合があります：
- Windows: `python` または `py`
- macOS/Linux: `python3` または `python`

アプリケーションは `python` コマンドを使用します。`python3` を使用したい場合は、シンボリックリンクを作成するか、エイリアスを設定してください。

#### Goコマンドの設定
Goは標準的な `go` コマンドを使用します：
- 全プラットフォーム: `go run` コマンドでファイルを実行
- Go 1.18以降を推奨（モジュール機能とジェネリクス対応）

#### 実行タイムアウト
- デフォルト: 30秒
- 長時間実行されるプログラムは自動的に停止されます
- 無限ループの防止機能

### データ保存

#### 実行履歴データベース
- **ファイル**: `learning_app.db`（SQLite）
- **場所**: アプリケーション実行ディレクトリ
- **内容**: 実行履歴、統計情報、実行時間など
- **初期化**: 初回実行時に自動的に作成される

**注意**: 現在の実装では、`run` コマンドでの単発実行は履歴に記録されません。`watch` モードでの自動実行のみが履歴として保存されます。

#### データベースの管理
```bash
# 履歴をクリア
cargo run -- clear

# 強制クリア（確認なし）
cargo run -- clear --force
```

### ログ設定

#### ログレベルの変更
```bash
# デバッグログを有効化
RUST_LOG=debug cargo run -- watch

# 情報レベルのログ
RUST_LOG=info cargo run -- watch

# エラーのみ
RUST_LOG=error cargo run -- watch
```

#### 利用可能なログレベル
- `error`: エラーのみ
- `warn`: 警告以上
- `info`: 情報以上（推奨）
- `debug`: デバッグ情報（開発用）
- `trace`: 全ての詳細情報

### カスタムディレクトリ構造

独自の学習ディレクトリを作成する場合：

```
my-exercises/
├── basics/
│   ├── hello.py
│   ├── hello.go
│   ├── variables.py
│   └── variables.go
├── advanced/
│   ├── classes.py
│   ├── structs.go
│   ├── modules.py
│   └── packages.go
└── projects/
    ├── calculator.py
    └── calculator.go
```

```bash
# カスタムディレクトリを監視
cargo run -- watch my-exercises

# セクション確認
cargo run -- sections --directory my-exercises
```

## 既知の制限事項

### 現在の制限

- **履歴記録**: `run` コマンドでの単発実行は履歴に記録されません
- **言語サポート**: 現在はPythonとGoをサポート
- **文字エンコーディング**: 日本語出力でWindows環境において文字化けが発生する場合があります
- **実行タイムアウト**: 長時間実行されるプログラムは30秒でタイムアウトします

### 今後の改善予定

- 他のプログラミング言語のサポート（JavaScript、Java、C++、Rustなど）
- 単発実行の履歴記録対応
- 設定ファイルによるカスタマイズ機能
- より詳細な統計情報とレポート機能
- 言語固有の設定とカスタマイズ

## トラブルシューティング

### よくある問題と解決方法

#### 1. Pythonが見つからない

**エラー例:**
```bash
Error: Python command not found
thread 'main' panicked at 'Python interpreter not found'
```

**解決方法:**
```bash
# Pythonのインストール確認
python --version
python3 --version

# PATHの確認（Windows）
where python

# PATHの確認（macOS/Linux）
which python
which python3
```

**対処法:**
- Pythonがインストールされていない場合: [Python公式サイト](https://www.python.org/)からインストール
- PATHが通っていない場合: 環境変数PATHにPythonのパスを追加
- `python3`しかない場合: エイリアスまたはシンボリックリンクを作成

#### 2. Goが見つからない

**エラー例:**
```bash
Error: Go command not found
Failed to execute go command: program not found
```

**解決方法:**
```bash
# Goのインストール確認
go version

# PATHの確認（Windows）
where go

# PATHの確認（macOS/Linux）
which go
```

**対処法:**
- Goがインストールされていない場合: [Go公式サイト](https://golang.org/)からインストール
- PATHが通っていない場合: 環境変数PATHにGoのパスを追加
- Go 1.18以降の使用を推奨

#### 3. ファイル監視が開始されない

**エラー例:**
```bash
Error: Permission denied (os error 13)
Error: No such file or directory (os error 2)
```

**解決方法:**
```bash
# ディレクトリの存在確認
ls -la examples/

# 権限の確認
ls -ld examples/

# 権限の修正（必要に応じて）
chmod 755 examples/
```

#### 4. アプリケーションが応答しない

**症状:** `Ctrl+C` を押しても終了しない

**解決方法:**
```bash
# 強制終了（Linux/macOS）
kill -9 <process_id>

# 強制終了（Windows）
taskkill /F /PID <process_id>
```

#### 5. 実行結果が表示されない

**考えられる原因:**
- ファイルの拡張子が `.py` または `.go` でない
- ファイルが監視対象ディレクトリにない
- ファイルに構文エラーがある
- 対応する言語（PythonまたはGo）がインストールされていない

**確認方法:**
```bash
# 詳細モードで実行
cargo run -- watch --verbose

# 単発実行でテスト（Python）
cargo run -- run your_file.py --verbose

# 単発実行でテスト（Go）
cargo run -- run your_file.go --verbose
```

#### 6. データベースエラー

**エラー例:**
```bash
Error: database is locked
Error: unable to open database file
```

**解決方法:**
```bash
# データベースファイルの確認
ls -la learning_app.db

# 権限の修正
chmod 644 learning_app.db

# データベースの再作成（履歴は失われます）
rm learning_app.db
cargo run -- stats  # 新しいDBが作成される
```

#### 7. 文字化け（Windows環境）

**症状:** 日本語出力が文字化けする

**解決方法:**
```bash
# コマンドプロンプトの文字コードをUTF-8に設定
chcp 65001

# PowerShellの場合
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

# 環境変数での設定（永続化）
set PYTHONIOENCODING=utf-8
```

### デバッグ方法

#### 詳細ログの有効化

```bash
# 全ての詳細ログ
RUST_LOG=trace cargo run -- watch --verbose

# 特定のモジュールのログ
RUST_LOG=learning_programming_app::core=debug cargo run -- watch

# ファイル監視のデバッグ
RUST_LOG=notify=debug cargo run -- watch
```

#### 環境情報の確認

```bash
# Rustのバージョン
rustc --version
cargo --version

# システム情報
uname -a  # Linux/macOS
systeminfo  # Windows

# 依存関係の確認
cargo tree
```

### パフォーマンスの問題

#### 大量ファイルの監視

大量のファイルがあるディレクトリを監視する場合：

```bash
# 特定のサブディレクトリのみ監視
cargo run -- watch examples/section1-basics

# 不要なファイルを除外（.gitignoreを活用）
echo "*.tmp" >> .gitignore
echo "*.log" >> .gitignore
```

#### メモリ使用量の確認

```bash
# メモリ使用量の監視（Linux/macOS）
top -p $(pgrep learning-programming-app)

# メモリ使用量の監視（Windows）
tasklist /FI "IMAGENAME eq learning-programming-app.exe"
```

### サポートとヘルプ

問題が解決しない場合：

1. **ヘルプの確認**
```bash
cargo run -- --help
cargo run -- watch --help
```

2. **バージョン情報**
```bash
cargo run -- --version
```

3. **ログファイルの作成**
```bash
RUST_LOG=debug cargo run -- watch 2>&1 | tee debug.log
```

4. **最小再現例の作成**
```bash
# 最小限のテストファイルで確認
echo 'print("test")' > test.py
cargo run -- run test.py
```

## 開発者向け情報

### 開発環境のセットアップ

```bash
# 開発用の依存関係を含むビルド
cargo build

# 開発用の実行（デバッグ情報付き）
RUST_LOG=debug cargo run -- watch

# コードフォーマット
cargo fmt

# リンター実行
cargo clippy

# ドキュメント生成
cargo doc --open
```

### テストの実行

```bash
# 全テストを実行
cargo test

# 特定のテストを実行
cargo test test_python_handler

# 統合テストを実行
cargo test --test integration_tests

# テストカバレッジ（tarpaulinが必要）
cargo tarpaulin --out Html
```

### 新しい言語ハンドラーの追加

#### 1. ハンドラーの実装

`src/handlers/` に新しいハンドラーを作成：

```rust
// src/handlers/javascript.rs
use crate::handlers::base::{LanguageHandler, ExecutionResult};
use async_trait::async_trait;
use std::path::Path;
use tokio::process::Command;

#[derive(Debug, Clone)]
pub struct JavaScriptHandler {
    node_command: String,
}

impl JavaScriptHandler {
    pub fn new() -> Self {
        Self {
            node_command: "node".to_string(),
        }
    }
}

#[async_trait]
impl LanguageHandler for JavaScriptHandler {
    async fn execute(&self, file_path: impl AsRef<Path> + Send) -> Result<ExecutionResult> {
        let mut cmd = Command::new(&self.node_command);
        cmd.arg(file_path.as_ref());
        
        // 実行ロジックの実装
        // ...
    }
    
    fn get_extension(&self) -> &'static str { "js" }
    fn get_name(&self) -> &'static str { "JavaScript" }
}
```

#### 2. ハンドラーの登録

`src/cli/interface.rs` でハンドラーを登録：

```rust
// CommandLineInterface::new() 内で
language_handlers
    .register_handler("js", Arc::new(JavaScriptHandler::new()))
    .await;
```

#### 3. テストの追加

```rust
// tests/handlers/test_javascript.rs
#[tokio::test]
async fn test_javascript_execution() {
    let handler = JavaScriptHandler::new();
    // テストロジック
}
```

### アーキテクチャ

#### コンポーネント構成

```
CLI Layer
├── CommandLineInterface    # メインインターフェース
├── Commands               # コマンド定義
└── ShutdownHandler       # 終了処理

Core Layer
├── FileWatcherService    # ファイル監視
├── AutoExecutorService   # 自動実行
├── HistoryManagerService # 履歴管理
└── LanguageHandlerService # 言語ハンドラー管理

Handler Layer
├── LanguageHandler (trait) # 言語ハンドラーインターフェース
├── PythonHandler          # Python実行
└── [Future handlers]      # 将来の言語サポート

Utils Layer
├── ErrorHandler          # エラー処理
├── DisplayUtils         # 表示ユーティリティ
└── ConfigManager        # 設定管理
```

#### データフロー

```
File Change → FileWatcher → AutoExecutor → LanguageHandler → ExecutionResult → Display
                                        ↓
                                   HistoryManager → SQLite Database
```

### コントリビューション

#### プルリクエストの手順

1. **フォークとクローン**
```bash
git clone https://github.com/your-username/learning-programming-app.git
cd learning-programming-app
```

2. **ブランチの作成**
```bash
git checkout -b feature/new-language-support
```

3. **開発とテスト**
```bash
# 開発
cargo build
cargo test

# フォーマットとリント
cargo fmt
cargo clippy
```

4. **コミットとプッシュ**
```bash
git add .
git commit -m "Add JavaScript language support"
git push origin feature/new-language-support
```

#### コーディング規約

- **Rust標準**: `cargo fmt` と `cargo clippy` に従う
- **非同期処理**: `async/await` を適切に使用
- **エラーハンドリング**: `anyhow` を使用し、適切なエラーメッセージを提供
- **テスト**: 新機能には必ずテストを追加
- **ドキュメント**: パブリック関数にはドキュメントコメントを追加

#### 新機能の提案

新機能を提案する場合：

1. **Issue作成**: 機能の詳細と用途を説明
2. **設計議論**: アーキテクチャへの影響を検討
3. **実装**: プロトタイプまたは完全な実装
4. **レビュー**: コードレビューとテスト

### ビルドとリリース

#### リリースビルド

```bash
# 最適化されたリリースビルド
cargo build --release

# バイナリサイズの最適化
cargo build --release --target x86_64-unknown-linux-musl

# クロスコンパイル（例：Windows向け）
cargo build --release --target x86_64-pc-windows-gnu
```

#### パッケージング

```bash
# Cargo パッケージの作成
cargo package

# crates.io への公開（メンテナー用）
cargo publish
```

## 技術仕様

### 開発環境

- **言語**: Rust 1.82+ (2021 edition)
- **最小サポートRustバージョン (MSRV)**: 1.82.0

### 主要依存関係

| クレート | バージョン | 用途 |
|---------|-----------|------|
| `tokio` | 1.40+ | 非同期ランタイム |
| `clap` | 4.5+ | CLI フレームワーク |
| `console` | 0.15+ | コンソール出力装飾 |
| `notify` | 6.1+ | ファイルシステム監視 |
| `sqlx` | 0.8+ | データベースアクセス |
| `anyhow` | 1.0+ | エラーハンドリング |
| `tracing` | 0.1+ | 構造化ログ |
| `serde` | 1.0+ | シリアライゼーション |
| `chrono` | 0.4+ | 日時処理 |
| `uuid` | 1.10+ | 一意識別子 |
| `async-trait` | 0.1+ | 非同期トレイト |

### 言語サポート

| 言語 | 拡張子 | 実行コマンド | 構文チェック | ステータス |
|------|--------|-------------|-------------|-----------|
| Python | `.py` | `python` | `python -m py_compile` | ✅ 完全サポート |
| Go | `.go` | `go run` | `go build -o /dev/null` | ✅ 完全サポート |
| JavaScript | `.js` | `node` | - | 🚧 計画中 |
| Rust | `.rs` | `cargo run` | `cargo check` | 🚧 計画中 |

### システム要件

#### 最小要件
- **RAM**: 64MB
- **ディスク**: 50MB（バイナリ + データベース）
- **CPU**: 任意のx86_64またはARM64プロセッサ

#### 推奨要件
- **RAM**: 128MB以上
- **ディスク**: 100MB以上（履歴データ用）
- **CPU**: マルチコア（並列処理のため）

### サポートプラットフォーム

| OS | アーキテクチャ | ステータス |
|----|---------------|-----------|
| Windows | x86_64 | ✅ サポート |
| macOS | x86_64, ARM64 | ✅ サポート |
| Linux | x86_64, ARM64 | ✅ サポート |
| FreeBSD | x86_64 | 🧪 実験的 |

### パフォーマンス特性

- **ファイル監視**: リアルタイム（<100ms遅延）
- **実行開始**: <50ms（小さなPythonファイル）
- **メモリ使用量**: 10-20MB（ベースライン）
- **データベース**: SQLite（軽量、ファイルベース）

### セキュリティ

- **コード実行**: ユーザー権限で実行（サンドボックスなし）
- **ファイルアクセス**: 指定ディレクトリのみ監視
- **データ保存**: ローカルSQLiteデータベース
- **ネットワーク**: 外部通信なし

## ライセンス

MIT License

Copyright (c) 2025 Learning Programming App Team

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## 貢献とサポート

### 貢献方法

プルリクエストやイシューの報告を歓迎します：

- 🐛 **バグ報告**: [Issues](https://github.com/your-repo/learning-programming-app/issues)
- 💡 **機能提案**: [Feature Requests](https://github.com/your-repo/learning-programming-app/issues)
- 🔧 **プルリクエスト**: [Pull Requests](https://github.com/your-repo/learning-programming-app/pulls)

### コミュニティ

- **ディスカッション**: [GitHub Discussions](https://github.com/your-repo/learning-programming-app/discussions)
- **Wiki**: [プロジェクトWiki](https://github.com/your-repo/learning-programming-app/wiki)

### 謝辞

このプロジェクトは以下のオープンソースプロジェクトに依存しています：

- [Rust Programming Language](https://www.rust-lang.org/)
- [Tokio](https://tokio.rs/) - 非同期ランタイム
- [clap](https://clap.rs/) - CLI フレームワーク
- [notify](https://github.com/notify-rs/notify) - ファイル監視
- [SQLx](https://github.com/launchbadge/sqlx) - データベースアクセス

---

**Happy Learning! 🚀**

プログラミング学習を楽しんでください。質問や提案がありましたら、お気軽にお知らせください。