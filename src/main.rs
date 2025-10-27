use clap::Parser;
use notify::{Event, EventKind, RecursiveMode, Result, Watcher};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use tokio::process::Command;
use tracing::{error, info};

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
    let target_extensions = ["go", "py"];

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

    let mut command;

    if extension == "go" {
        command = Command::new("go");
        command.arg("run").arg(&path);
    } else if extension == "py" {
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
