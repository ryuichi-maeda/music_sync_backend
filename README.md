# MusicSync backend

## Setup

Rustアプリケーションは `Docker` と `ローカル` のどちらで動かしても大丈夫です。

### RustアプリケーションをDockerで動かしたい場合

コンテナ立ち上げ。

```
docker compose up -d
```

マイグレーション。

```
docker compose exec app bash
make migrate
```

### RustアプリケーションをLocalで動かしたい場合

コンテナ立ち上げ（DBのみ）。

```
docker compose up db -d
```

マイグレーション。

```
cargo install sqlx-cli
make migrate
```

Rustアプリケーション実行。

```
cargo run
```

コードの変更を監視し、自動で `cargo run` したい場合。

```
cargo install cargo-watch
cargo watch -x run
```


### Docker containers

| コンテナ名 | PORT | 説明 |
| --- | --- | --- |
| rust | 8000 | rustアプリケーション，ソースコードが変更されると自動で検知してリビルドされる |
| db | 3306 | MySQLデータベース |
| spectaql | 8001 | GraphQL API ドキュメント |

## SpectaQL

GraphQL APIのドキュメント生成。

GitHub Pagesにホスティングされている（[https://ryuichi-maeda.github.io/music_sync_backend/](https://ryuichi-maeda.github.io/music_sync_backend/)）。

SpectaQLのインストール。

```
npm install -g spectaql
```

以下のコマンドでドキュメントを生成する。

```
make gen-api-doc
```

以下のコマンドでも可。

```
cd docs/spectaql
npx spectaql config.yaml
```

## ユビキタス言語

| 言葉 |英名| 意味 |
| --- | --- |--- |
| ルーム | Room | 音楽ライブラリを共有させたいユーザーが集まる場所 |
| ルームピン | RoomPin | ルームに参加するための6桁の数字 |
| ホスト | Host | ルーム作成者 |
| ゲスト | Guest | ルーム参加者 |
| ゲストユーザー | GuestUser | アプリに登録していないユーザー |
| 登録済みユーザー | RegisteredUser | アプリに登録しているユーザー |
| 音楽ライブラリ | Library | 音楽アプリでユーザーが追加した音楽全て |
| 共有プレイリスト | SharedLibrary | 各ユーザーの音楽ライブラリの共通の音楽をまとめたもの |

## コミットメッセージ　prefix

Conventional Commit Messageを採用
https://gist.github.com/qoomon/5dfcdf8eec66a051ecd85625518cfd13
