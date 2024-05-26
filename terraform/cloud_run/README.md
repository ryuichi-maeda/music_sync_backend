# Setup

## Dependencies

- terraform
- gcloud

## gcloudのプロジェクト設定

gcloud コマンドでプロジェクト設定を行う。

# Run

反映させたい環境のディレクトリに移動。

```bash
cd environment/dev
```

DB名などを設定する。

```bash
cp secret.tfvars.example secret.tfvars
```

`secret.tfvars` の中身を適切な値に書き換える。

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
terraform plan -var-file="secret.tfvars"
```

特に問題なければ、AWSに反映させる。

```bash
terraform apply -var-file="secret.tfvars"
```
