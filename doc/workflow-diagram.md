# ワークフロー図

## 1. アプリケーション全体のワークフロー

```mermaid
flowchart TD
    A[アプリケーション起動] --> B{コマンド解析}
    
    B -->|watch| C[ファイル監視開始]
    B -->|sections| D[セクション一覧表示]
    B -->|history| E[実行履歴表示]
    B -->|stats| F[統計情報表示]
    B -->|clear| G[履歴クリア]
    B -->|run| H[単発実行]
    B -->|generate-go| I[Go問題生成]
    B -->|help| J[ヘルプ表示]
    
    C --> K[ディレクトリ監視ループ]
    K --> L{ファイル変更検出}
    L -->|変更あり| M[ファイル実行]
    L -->|変更なし| K
    L -->|Ctrl+C| N[監視停止]
    
    M --> O[実行結果表示]
    O --> P[履歴記録]
    P --> K
    
    D --> Q[ディレクトリスキャン]
    Q --> R[セクション情報表示]
    
    E --> S[データベースクエリ]
    S --> T[履歴フォーマット表示]
    
    F --> U[統計計算]
    U --> V[統計情報表示]
    
    G --> W{確認プロンプト}
    W -->|Yes| X[履歴削除実行]
    W -->|No| Y[キャンセル]
    
    H --> Z[指定ファイル実行]
    Z --> AA[結果表示]
    
    I --> BB[Go問題生成ワークフロー]
    
    N --> CC[アプリケーション終了]
    R --> CC
    T --> CC
    V --> CC
    X --> CC
    Y --> CC
    AA --> CC
    BB --> CC
    J --> CC
```

## 2. ファイル実行ワークフロー

```mermaid
flowchart TD
    A[ファイル実行要求] --> B{ファイル存在確認}
    B -->|存在しない| C[エラー: ファイル未発見]
    B -->|存在する| D[ファイル拡張子取得]
    
    D --> E{拡張子確認}
    E -->|.py| F[Pythonハンドラー選択]
    E -->|.go| G[Goハンドラー選択]
    E -->|その他| H[エラー: 未対応形式]
    
    F --> I[Python実行コマンド]
    G --> J[Go実行コマンド]
    
    I --> K[python file.py]
    J --> L[go run file.go]
    
    K --> M{実行結果}
    L --> M
    
    M -->|成功| N[成功結果作成]
    M -->|失敗| O[エラー結果作成]
    
    N --> P[実行時間記録]
    O --> P
    
    P --> Q[セクション名抽出]
    Q --> R[ExecutionRecord作成]
    R --> S[データベース記録]
    S --> T[結果フォーマット]
    T --> U[ユーザーに表示]
    
    C --> V[エラー表示]
    H --> V
    V --> W[継続実行]
    U --> W
```

## 3. データベース操作ワークフロー

```mermaid
flowchart TD
    A[データベース操作要求] --> B{操作種別}
    
    B -->|記録| C[execution_history INSERT]
    B -->|履歴取得| D[履歴SELECT クエリ]
    B -->|統計取得| E[統計計算クエリ群]
    B -->|履歴削除| F[DELETE クエリ]
    
    C --> G[実行記録をINSERT]
    G --> H{INSERT成功}
    H -->|成功| I[記録完了]
    H -->|失敗| J[エラーログ出力]
    
    D --> K[ORDER BY timestamp DESC]
    K --> L{LIMIT指定}
    L -->|あり| M[LIMIT N適用]
    L -->|なし| N[全件取得]
    M --> O[結果返却]
    N --> O
    
    E --> P[COUNT(*) 総実行数]
    P --> Q[COUNT(*) WHERE success=true 成功数]
    Q --> R[AVG(execution_time) 平均時間]
    R --> S[GROUP BY file_path 最多実行ファイル]
    S --> T[MAX(timestamp) 最終実行時刻]
    T --> U[統計オブジェクト作成]
    U --> V[統計結果返却]
    
    F --> W{force フラグ}
    W -->|true| X[即座に削除実行]
    W -->|false| Y[確認プロンプト表示]
    Y --> Z{ユーザー確認}
    Z -->|Yes| X
    Z -->|No| AA[削除キャンセル]
    X --> BB[DELETE FROM execution_history]
    BB --> CC[削除完了]
    
    I --> DD[処理完了]
    J --> DD
    O --> DD
    V --> DD
    AA --> DD
    CC --> DD
```

## 4. Go問題生成ワークフロー

```mermaid
flowchart TD
    A[Go問題生成開始] --> B{skip_preview フラグ}
    
    B -->|false| C[セクション設定読み込み]
    B -->|true| D[即座に生成開始]
    
    C --> E[プレビュー表示]
    E --> F{ユーザー選択}
    
    F -->|y: Yes| D
    F -->|n: No| G[生成キャンセル]
    F -->|m: Modify| H[セクション修正モード]
    F -->|d: Details| I[詳細情報表示]
    
    H --> J[修正オプション表示]
    J --> K{修正種別}
    K -->|セクション削除| L[セクション削除処理]
    K -->|セクション並び替え| M[順序変更処理]
    K -->|カスタムセクション追加| N[新セクション追加]
    K -->|セクション詳細修正| O[セクション内容修正]
    K -->|デフォルトリセット| P[設定初期化]
    K -->|修正完了| E
    
    I --> E
    L --> J
    M --> J
    N --> J
    O --> J
    P --> J
    
    D --> Q[出力ディレクトリ作成]
    Q --> R[セクション生成ループ開始]
    
    R --> S{全セクション処理完了?}
    S -->|No| T[次のセクション処理]
    S -->|Yes| U[生成完了処理]
    
    T --> V[セクションディレクトリ作成]
    V --> W[問題生成ループ開始]
    
    W --> X{セクション内全問題完了?}
    X -->|No| Y[次の問題生成]
    X -->|Yes| Z[セクション完了]
    
    Y --> AA[問題内容生成]
    AA --> BB[難易度設定]
    BB --> CC[ファイル書き込み]
    CC --> W
    
    Z --> S
    
    U --> DD[ファイル検証]
    DD --> EE{Go構文チェック}
    EE -->|成功| FF[生成成功報告]
    EE -->|失敗| GG[エラー報告]
    
    G --> HH[処理終了]
    FF --> HH
    GG --> HH
```

## 5. エラーハンドリングワークフロー

```mermaid
flowchart TD
    A[エラー発生] --> B{エラー種別判定}
    
    B -->|ファイル未発見| C[FileNotFoundError処理]
    B -->|権限エラー| D[PermissionError処理]
    B -->|構文エラー| E[SyntaxError処理]
    B -->|実行時エラー| F[RuntimeError処理]
    B -->|システムエラー| G[SystemError処理]
    B -->|タイムアウト| H[TimeoutError処理]
    B -->|ファイル監視エラー| I[FileWatchError処理]
    
    C --> J[ファイルパス表示]
    D --> K[権限不足メッセージ]
    E --> L[構文エラー詳細表示]
    F --> M[実行時エラー詳細表示]
    G --> N[システムエラー情報表示]
    H --> O[タイムアウト情報表示]
    I --> P[監視エラー情報表示]
    
    J --> Q[エラーメッセージ出力]
    K --> Q
    L --> Q
    M --> Q
    N --> Q
    O --> Q
    P --> Q
    
    Q --> R{継続可能エラー?}
    R -->|Yes| S[アプリケーション継続]
    R -->|No| T[グレースフル終了]
    
    S --> U[次の処理待機]
    T --> V[リソースクリーンアップ]
    V --> W[アプリケーション終了]
```