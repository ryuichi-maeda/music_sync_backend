# MusicSync backend

## Setup

コンテナ立ち上げ．

```
docker compose up -d
```

### Docker containers

| コンテナ名 | URL | 説明 |
| --- | --- | --- |
| Swagger UI | http://localhost:8002/ | ./openapi/openapi.yaml で定義されているAPI仕様の確認 |
| Swagger API mock server (Prism) | http://localhost:8003/ | ./openapi/openapi.yaml の定義に基づいたモックサーバー |

## Swagger

APIドキュメント．
./openapi/openapi.yaml で管理．
編集後にSwagger UIとモックサーバーに反映させるために，コンテナのリスタートが必要．

```
docker container restart swagger-ui swagger-api
```
