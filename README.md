# Learning Programming App

プログラミング学習者向けのファイル監視型自動実行CLIアプリケーション

## 概要

Learning Programming Appは、プログラミング学習を支援するコマンドラインツールです。ファイルを保存すると自動的にコードが実行され、即座に結果を確認できます。エラーが発生してもアプリケーションは継続動作します。

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
| `generate-go` | Go学習問題を生成 | `-s, --skip-preview` | false |
|               |                  | `-o, --output <DIR>` | `./learning-go` |
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

#### `generate-go` - Go学習問題生成

```bash
# デフォルト設定で生成（プレビューあり）
cargo run -- generate-go

# プレビューをスキップして即座に生成
cargo run -- generate-go --skip-preview

# カスタム出力ディレクトリを指定
cargo run -- generate-go --output ./my-go-learning
```

## Go学習問題の生成

### Go問題生成器の使い方

Learning Programming Appには、Go言語学習用の問題を自動生成する機能が含まれています。この機能により、段階的な難易度で構成された包括的なGo学習教材を作成できます。

#### 基本的な使用方法

##### **デフォルト設定で生成**
```bash
# 最も簡単な方法（プレビューあり）
cargo run -- generate-go

# プレビューをスキップして即座に生成
cargo run -- generate-go --skip-preview

# カスタム出力ディレクトリを指定
cargo run -- generate-go --output ./my-go-learning
```

##### **詳細オプション**
```bash
# 全オプションを指定
cargo run -- generate-go --skip-preview --output ./custom-go-problems

# ヘルプを表示
cargo run -- generate-go --help
```

#### 生成プロセスの詳細

##### **Step 1: セクション設定の確認**
プレビューモード（デフォルト）では、生成される内容を事前に確認できます：

```bash
$ cargo run -- generate-go
🚀 Go Learning Problem Generator

📋 Loading section configuration and requesting user approval...

=== Go Learning Sections Preview ===
The following 10 sections will be created:

1. Basic Syntax (section1-basics)
   Description: Variables, constants, and basic data types
   Problems: 10 per section
   Topics: Variables, Constants, Data Types

2. Control Flow (section2-control-flow)
   Description: Conditional statements and loops
   Problems: 10 per section
   Topics: If Statements, For Loops, Switch Statements

[... 8 more sections ...]

Total problems to be generated: 100

Do you want to proceed with this section structure? (y)es / (n)o / (m)odify / (d)etails:
```

**選択肢:**
- `y` (yes): 生成を開始
- `n` (no): 生成をキャンセル
- `m` (modify): セクション構成をカスタマイズ
- `d` (details): 詳細情報を表示

##### **Step 2: セクション構成のカスタマイズ**
`m` (modify) を選択すると、セクション構成をカスタマイズできます：

```bash
=== Section Modification Options ===
1. Remove a section
2. Reorder sections
3. Add a custom section
4. Modify section details
5. Reset to defaults
6. Done with modifications

Choose an option (1-6):
```

**カスタマイズ例:**
```bash
# セクションの削除
Choose an option (1-6): 1
Current sections:
1. Basic Syntax
2. Control Flow
[...]
Enter section number to remove (or 0 to cancel): 3

# セクションの並び替え
Choose an option (1-6): 2
Enter two section numbers to swap (e.g., '1 3') or 0 to cancel: 2 4

# カスタムセクションの追加
Choose an option (1-6): 3
Section ID (e.g., 'section11-custom'): section11-web-development
Section Name: Web Development
Section Description: HTTP servers and web applications
```

##### **Step 3: 生成プロセス**
設定確認後、自動的に生成が開始されます：

```bash
📊 Generation Plan:
   📁 Output directory: ./learning-go
   📚 Sections to create: 10
   📄 Total problems: 100
   🎯 Problems per section: 10

⏳ Starting generation process...
🏗️  Creating Go learning directory structure...
📝 Generating Go problem files with progressive difficulty...
🔍 Validating generated files for syntax and formatting...
🔧 Attempting Go compiler validation (optional)...

🎉 Generation Summary:
   ✅ Successfully created 10 sections
   ✅ Generated 100 Go problem files
   ✅ All files validated for syntax and formatting
   📁 Location: ./learning-go
```

#### 生成される内容

##### **ディレクトリ構造**
```
learning-go/
├── section1-basics/
│   ├── problem01_variables.go
│   ├── problem02_constants.go
│   ├── problem03_data_types.go
│   ├── problem04_zero_values.go
│   ├── problem05_type_inference.go
│   ├── problem06_numeric_types.go
│   ├── problem07_string_operations.go
│   ├── problem08_boolean_operations.go
│   ├── problem09_type_conversion.go
│   └── problem10_variable_scope.go
├── section2-control-flow/
│   ├── problem01_if_statements.go
│   ├── problem02_if_with_initialization.go
│   ├── problem03_for_loops_basic.go
│   ├── problem04_range_loops.go
│   ├── problem05_switch_basic.go
│   ├── problem06_switch_without_expression.go
│   ├── problem07_switch_fallthrough.go
│   ├── problem08_nested_control_flow.go
│   ├── problem09_break_continue.go
│   └── problem10_control_flow_patterns.go
├── section3-functions/
├── section4-packages/
├── section5-structs/
├── section6-interfaces/
├── section7-concurrency/
├── section8-error-handling/
├── section9-pointers/
└── section10-collections/
```

##### **問題ファイルの構造**
各問題ファイルには以下の要素が含まれます：

```go
// Problem: Variable Declaration Practice
// Topic: Variables
// Difficulty: 1

package main

import "fmt"

func main() {
    // TODO: Declare a variable named 'name' of type string and assign it your name
    
    // TODO: Declare a variable named 'age' using short variable declaration and assign it a number
    
    // TODO: Declare multiple variables in one line: x, y both integers with values 10, 20
    
    // TODO: Declare a variable without initialization and observe its zero value
    var count int
    
    fmt.Printf("Name: %s, Age: %d\n", name, age)
    fmt.Printf("X: %d, Y: %d\n", x, y)
    fmt.Printf("Count (zero value): %d\n", count)
}
```

##### **段階的難易度システム**
各セクションの10問は段階的な難易度で構成されます：

- **問題1-3**: 難易度1（基礎レベル）
- **問題4-6**: 難易度2（中級レベル）
- **問題7-10**: 難易度3（上級レベル）

#### 生成される学習コンテンツ

##### **Section 1: Basic Syntax（基礎構文）**
- 変数宣言（var、短縮宣言）
- 定数とiota
- 基本データ型（int、float、string、bool）
- ゼロ値の理解
- 型推論と型変換
- 変数スコープ

##### **Section 2: Control Flow（制御構造）**
- if文（基本、else、初期化付き）
- for文（基本、range、while風）
- switch文（式あり、式なし、fallthrough）
- ネストした制御構造
- break、continue

##### **Section 3: Functions（関数）**
- 関数の定義と呼び出し
- 複数戻り値
- 可変長引数
- 関数リテラルとクロージャ
- defer文

##### **Section 4: Packages（パッケージ）**
- パッケージ宣言
- import文（単一、グループ、エイリアス）
- 標準ライブラリの使用
- エクスポートされた識別子

##### **Section 5: Structs（構造体）**
- 構造体の定義と初期化
- メソッドの定義
- ポインタレシーバー
- 構造体の埋め込み

##### **Section 6: Interfaces（インターフェース）**
- インターフェースの定義
- 空のインターフェース
- 型アサーション
- インターフェースの組み合わせ

##### **Section 7: Concurrency（並行処理）**
- ゴルーチンの基本
- チャネルの作成と操作
- select文
- チャネルの方向性

##### **Section 8: Error Handling（エラー処理）**
- errorインターフェース
- エラーの作成（errors.New、fmt.Errorf）
- エラーハンドリングパターン
- panicとrecover

##### **Section 9: Pointers（ポインタ）**
- ポインタの基本（&、*演算子）
- 構造体へのポインタ
- new()とmake()関数
- メモリ管理の理解

##### **Section 10: Collections（コレクション）**
- 配列の宣言と操作
- スライスの作成と操作
- マップの宣言と操作
- append関数とスライスの拡張

#### 生成されたファイルの使用方法

##### **学習の開始**
```bash
# 生成されたディレクトリに移動
cd learning-go/section1-basics

# 最初の問題を開く
code problem01_variables.go  # VS Code
vim problem01_variables.go   # Vim
nano problem01_variables.go  # Nano

# 問題を解いてテスト実行
go run problem01_variables.go
```

##### **自動監視での学習**
```bash
# Learning Programming App で監視開始
cargo run -- watch learning-go

# 別のターミナルでファイルを編集・保存
# 保存と同時に自動実行される
```

##### **進捗管理**
```bash
# 実行履歴を確認
cargo run -- history

# 統計情報を表示
cargo run -- stats

# 特定のファイルを単発実行
cargo run -- run learning-go/section1-basics/problem01_variables.go
```

## テストとバリデーション

### Go問題生成器のテスト

Go学習問題の品質と正確性を検証するための包括的なテストスイートが含まれています。

#### テストの実行方法

##### 全てのテストを実行
```bash
cargo test --test go_problems_validation_test
```

##### 特定のテストを実行
```bash
# Go構文検証テストのみ
cargo test --test go_problems_validation_test test_go_syntax_validation

# ディレクトリ構造テストのみ
cargo test --test go_problems_validation_test test_directory_structure_creation

# パフォーマンステストのみ
cargo test --test go_problems_validation_test test_problem_generation_performance
```

##### 詳細出力付きでテスト実行
```bash
cargo test --test go_problems_validation_test -- --nocapture
```

#### テストの種類と用途

##### **基本検証テスト**
- `test_go_problem_validation` - 生成されたGo問題の基本構造を検証
- `test_section_config_validation` - セクション設定の妥当性を検証

##### **ファイル・ディレクトリテスト**
- `test_directory_structure_creation` - ディレクトリ構造の作成を検証
- `test_comprehensive_directory_validation` - 既存の生成済みディレクトリを検証

##### **品質検証テスト**
- `test_problem_content_quality` - 問題コンテンツの品質を検証
- `test_go_syntax_validation` - Go コンパイラを使った構文検証

##### **統合・パフォーマンステスト**
- `test_integration_with_file_generation` - ファイル生成システムとの統合テスト
- `test_problem_generation_performance` - 生成パフォーマンスのベンチマーク

#### GoSyntaxValidator の使い方

##### **単一ファイルの検証**
```rust
use std::path::Path;

let file_path = Path::new("example.go");
match GoSyntaxValidator::validate_go_file_syntax(&file_path) {
    Ok(()) => println!("構文は正しいです"),
    Err(error) => println!("構文エラー: {}", error),
}
```

##### **ディレクトリ全体の検証**
```rust
let dir_path = Path::new("learning-go/section1-basics");
let errors = GoSyntaxValidator::validate_directory_go_files(&dir_path)?;
if errors.is_empty() {
    println!("全てのファイルが正しいです");
} else {
    println!("エラーが見つかりました: {:?}", errors);
}
```

##### **Go コンテンツの検証**
```rust
let go_code = r#"
package main
import "fmt"
func main() {
    fmt.Println("Hello, World!")
}
"#;

match GoSyntaxValidator::validate_go_content_syntax(go_code) {
    Ok(()) => println!("コードは正しいです"),
    Err(error) => println!("エラー: {}", error),
}
```

#### 実際の使用例

##### **開発中の検証**
```bash
# 新しい問題を生成した後
cargo test --test go_problems_validation_test test_comprehensive_directory_validation

# 特定のセクションの品質チェック
cargo test --test go_problems_validation_test test_problem_content_quality
```

##### **CI/CDでの自動検証**
```bash
# 全ての検証を実行（Go がインストールされていない環境でも動作）
cargo test --test go_problems_validation_test --quiet
```

#### テストが検証する項目

##### **Go構文の正確性**
✅ 生成されたGoファイルがコンパイル可能  
✅ 適切なpackage main宣言  
✅ func main()関数の存在  
✅ 正しいimport文の構文  

##### **問題構造の品質**
✅ 必須コメント（Problem、Topic、Difficulty）の存在  
✅ 適切なファイル名形式（.go拡張子）  
✅ 難易度レベルの妥当性（1-3の範囲）  
✅ セクションごとに正確に10問生成  

##### **ディレクトリ構造**
✅ learning-goディレクトリの作成  
✅ 全セクションディレクトリの存在  
✅ ファイルの適切な配置  
✅ ディレクトリ権限の正確性  

##### **コンテンツ品質**
✅ 段階的難易度の実装  
✅ 学習目標に沿った問題内容  
✅ 適切なコメントと説明  
✅ 実行可能なサンプルコード  

#### Go コンパイラとの統合

テストスイートは実際のGo コンパイラを使用して構文検証を行います：

- **Go がインストールされている場合**: 実際の `go build` コマンドで構文チェック
- **Go がインストールされていない場合**: 基本的な構造チェックのみ実行
- **エラー処理**: 構文エラーの詳細な報告とログ出力

##### **Go インストール状況の確認**
```bash
# Go のバージョン確認
go version

# テスト実行時のGo検証状況確認
cargo test --test go_problems_validation_test test_go_syntax_validation -- --nocapture
```

#### パフォーマンス検証

テストスイートには生成パフォーマンスのベンチマークも含まれています：

```bash
# パフォーマンステストの実行
cargo test --test go_problems_validation_test test_problem_generation_performance -- --nocapture
```

**期待される性能:**
- 全100問の生成: 5秒以内
- セクションあたり10問: 0.5秒以内
- 個別問題の検証: 0.01秒以内

これらのテストにより、生成されたGo学習問題の品質と正確性を継続的に検証します。

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
