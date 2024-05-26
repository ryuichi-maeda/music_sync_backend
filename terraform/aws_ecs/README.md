# Setup

## Dependencies

- terraform
- aws cli

## 環境変数の設定

```bash
cp .env.aws-credentials.example .env.aws-credentials
```

`.aws-credentials` にAWSアカウントのアクセスキーとシークレットキーを設定。

環境変数の読み込み。

```bash
source .aws-credentials
```

# Run

反映させたい環境のディレクトリに移動。

```bash
cd environment/dev
```

初期化。

```bash
terraform init
```

以下のコマンドで、反映前に確認。

```bash
# フォーマット
terraform fmt

# 構文チェック
terraform validate

# 変更されるリソースの確認
terraform plan
```

特に問題なければ、AWSに反映させる。

```bash
terraform apply
```
