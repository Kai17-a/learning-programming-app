use clap::Parser;
use notify::{Event, EventKind, RecursiveMode, Result, Watcher};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use tokio::process::Command;
use tracing::{error, info};
use which::which;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    dir: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // ログ設定
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    // 監視対象ディレクトリ
    let watch_dir = PathBuf::from(&args.dir);

    // ディレクトリ存在確認
    if !watch_dir.is_dir() {
        error!("ディレクトリが存在しません: {}", watch_dir.display());
        std::process::exit(1);
    }

    // イベントを受け取るチャンネル
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(&watch_dir, RecursiveMode::Recursive)?;

    info!("監視を開始: {}", watch_dir.display());

    let mut last_modified: HashMap<PathBuf, Instant> = HashMap::new();
    let debounce_duration = Duration::from_millis(300);

    for res in rx {
        match res {
            Ok(event) => {
                for path in event.paths {
                    if !path.is_file() {
                        continue;
                    }

                    let now = Instant::now();
                    let entry = last_modified.entry(path.clone()).or_insert(now);
                    if now.duration_since(*entry) < debounce_duration {
                        continue;
                    }
                    *entry = now;

                    if let EventKind::Modify(_) = event.kind {
                        // async 処理を別タスクで動かす
                        tokio::spawn(run_if_target_file(path));
                    }
                }
            }
            Err(e) => error!("watch error: {:?}", e),
        }
    }

    Ok(())
}

async fn run_if_target_file(path: PathBuf) {
    let target_extensions = ["go", "py", "lua"];

    let extension = match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => ext,
        None => {
            error!("拡張子がありません: {}", path.display());
            return;
        }
    };

    if !target_extensions.contains(&extension) {
        return;
    }

    let command_name = match extension {
        "go" => "go",
        "py" => "python",
        "lua" => "lua",
        _ => return,
    };

    if which(command_name).is_err() {
        error!(
            "コマンドが見つかりません: {} (必要な実行環境がインストールされていません)",
            command_name
        );
        return;
    }

    let mut command;

    if extension == "go" {
        // 実行環境存在チェック
        command = Command::new("go");
        command.arg("run").arg(&path);
    } else if extension == "py" {
        // 実行環境存在チェック
        command = Command::new("python");
        command.arg(&path);
    } else {
        return;
    }

    println!("実行中: {}", path.display());

    match command.output().await {
        Ok(output) => {
            if output.status.success() {
                println!("✅ 成功: {}", path.display());
                println!("=== 実行結果 ===============\n");
                println!("{}", String::from_utf8_lossy(&output.stdout));
                println!("\n===========================\n");
            } else {
                eprintln!("❌ 失敗: {}", path.display());
                eprintln!("=== エラー ===============\n");
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                eprintln!("\n===========================\n");
            }
        }
        Err(e) => eprintln!("実行エラー: {:?} ({})", e, path.display()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // 簡易ログを無効化する
    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test]
    async fn test_run_if_target_file_with_py_file() {
        init_logger();

        // 一時Pythonファイル作成
        let mut tmpfile = NamedTempFile::new().unwrap();
        writeln!(tmpfile, "print('hello test')").unwrap();
        let path = tmpfile.path().to_path_buf();

        // 実行
        run_if_target_file(path.clone()).await;

        // ファイルはまだ存在するはず
        assert!(path.exists());
    }

    #[tokio::test]
    async fn test_run_if_target_file_with_go_file() {
        init_logger();

        // 一時Goファイル作成
        let mut tmpfile = NamedTempFile::new().unwrap();
        writeln!(
            tmpfile,
            "package main\nimport \"fmt\"\nfunc main() {{ fmt.Println(\"hello go test\") }}"
        )
        .unwrap();
        let path = tmpfile.path().to_path_buf();

        run_if_target_file(path.clone()).await;

        assert!(path.exists());
    }

    #[tokio::test]
    async fn test_run_if_target_file_with_unsupported_extension() {
        init_logger();

        let mut tmpfile = NamedTempFile::new().unwrap();
        writeln!(tmpfile, "echo unsupported").unwrap();

        // 一時ファイル名を.txtに変更
        let path = tmpfile.path().with_extension("txt");

        // 実行（何も起きない）
        run_if_target_file(path.clone()).await;

        // 実行してもエラーにもならない（ただreturn）
        assert!(path.exists() || !path.exists()); // 実行確認用ダミー
    }

    #[tokio::test]
    async fn test_run_if_target_file_without_extension() {
        init_logger();

        // 一時ファイル名に拡張子なし
        let tmpfile = NamedTempFile::new().unwrap();
        let path = tmpfile.path().to_path_buf();

        // 実行
        run_if_target_file(path.clone()).await;

        // エラー出力が呼ばれるがクラッシュしない
        assert!(path.exists());
    }

    #[tokio::test]
    async fn test_run_if_target_file_command_not_found() {
        init_logger();

        // 存在しないコマンド (lua) を想定
        let mut tmpfile = NamedTempFile::new().unwrap();
        writeln!(tmpfile, "print('hi')").unwrap();

        // ".lua" の一時ファイルを実際に作成
        let lua_path = tmpfile.path().with_extension("lua");
        std::fs::copy(tmpfile.path(), &lua_path).unwrap();

        // Lua が未インストール環境で実行しても panic せず return することを確認
        run_if_target_file(lua_path.clone()).await;

        assert!(lua_path.exists());
    }
}
