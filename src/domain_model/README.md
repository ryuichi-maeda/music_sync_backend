# ドメインモデル

## 集約とドメインモデルの概要

### 音楽ライブラリ集約
- ユーザーが利用する音楽アプリ（Apple Music、Spotifyなど）の音楽ライブラリ情報を管理します。
- 各ユーザーには1:1で音楽ライブラリが紐づいています。

#### 主な操作
- 音楽ライブラリの作成
- 音楽ライブラリへの追加
- 音楽ライブラリの取得
- 音楽ライブラリの音楽削除

### ルーム集約
- ユーザー同士で共有ライブラリを作成するためのルームを管理します。
- 一つのルームには最大5人のユーザーを保持できます。
- ルームの確定はホストのみが行うことができます。
- ホストがルームから退出すると、ルームは解散（削除）されます。

#### 主な操作
- ルーム作成
- ルーム参加
- ルーム確定
- ルーム退出
- ルーム削除
- ルーム内のユーザー取得

### ユーザー集約
- ユーザー情報を管理します。
- ユーザー名は1文字から14文字の範囲で設定可能です。
- 現在はゲストユーザーのみをサポートしています。

#### 主な操作
- ユーザー作成
- ユーザー名変更
- パスワード変更
- メールアドレス変更
- ユーザー削除