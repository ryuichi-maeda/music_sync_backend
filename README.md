# MusicSync backend

## Setup

コンテナ立ち上げ．

```
docker compose up -d
```

### ユビキタス言語

| 言葉 | 意味 |
| --- | --- |
| ルーム | 音楽ライブラリを共有させたいユーザーが集まる場所 |
| ルームピン | ルームに参加するための6桁の数字 |
| ホスト | ルーム作成者 |
| ゲスト | ルーム参加者 |
| ゲストユーザー | アプリに登録していないユーザー |
| 音楽ライブラリ | 音楽アプリでユーザーが追加した音楽全て |
| 共有プレイリスト | 各ユーザーの音楽ライブラリの共通の音楽をまとめたもの |

### Docker containers

| コンテナ名 | URL | 説明 |
| --- | --- | --- |
| rust |  | rustアプリケーション，ソースコードが変更されると自動で検知してリビルドされる |
| db |  | MySQLデータベース |
| swagger-ui | http://localhost:8002/ | [./openapi/openapi.yaml](./openapi/openapi.yaml) で定義されているAPI仕様の確認 |
| swagger-api | http://localhost:8003/ | [./openapi/openapi.yaml](./openapi/openapi.yaml) の定義に基づいたモックサーバー (Prism) |

## Swagger

APIドキュメント．

[./openapi/openapi.yaml](./openapi/openapi.yaml) で管理．

編集内容をSwagger UIとモックサーバーに反映させるためには，コンテナのリスタートが必要．

```
docker container restart swagger-ui swagger-api
```

## コミットメッセージ　prefix
Conventional Commit Messageを採用
https://gist.github.com/qoomon/5dfcdf8eec66a051ecd85625518cfd13
