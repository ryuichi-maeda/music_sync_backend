# Setup

## Dependencies

- terraform
- aws cli

## 環境変数の設定

```
cp .env.aws-credentials.example .env.aws-credentials
```

`.aws-credentials` にAWSアカウントのアクセスキーとシークレットキーを設定。

環境変数の読み込み。

```
source .aws-credentials
```

# Run

反映させたい環境のディレクトリに移動。

```
cd environment/dev
```

以下のコマンドで、反映前に確認。

```
# フォーマット
terraform fmt

# 構文チェック
terraform validate

# 変更されるリソースの確認
terraform plan
```

特に問題なければ、AWSに反映させる。

```
terraform apply
```
