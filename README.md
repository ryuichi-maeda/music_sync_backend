# MusicSync backend

## Setup

コンテナ立ち上げ．

```
docker compose up -d
```

### Docker containers

| コンテナ名 | PORT | 説明 |
| --- | --- | --- |
| rust | 8000 | rustアプリケーション，ソースコードが変更されると自動で検知してリビルドされる |
| db | 3306 | MySQLデータベース |
| spectaql | 8001 | GraphQL API ドキュメント |

## SpectaQL

GraphQL APIのドキュメント生成。

GitHub Pagesにホスティングされている（https://ryuichi-maeda.github.io/music_sync_backend/）。

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

| 言葉 | 意味 |
| --- | --- |
| ルーム | 音楽ライブラリを共有させたいユーザーが集まる場所 |
| ルームピン | ルームに参加するための6桁の数字 |
| ホスト | ルーム作成者 |
| ゲスト | ルーム参加者 |
| ゲストユーザー | アプリに登録していないユーザー |
| 音楽ライブラリ | 音楽アプリでユーザーが追加した音楽全て |
| 共有プレイリスト | 各ユーザーの音楽ライブラリの共通の音楽をまとめたもの |


## コミットメッセージ　prefix
Conventional Commit Messageを採用
https://gist.github.com/qoomon/5dfcdf8eec66a051ecd85625518cfd13
