
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MusicLibrary {
  pub user_id: UserID,
  pub musics: Vec<Music>,
}

impl MusicLibrary {
  /// コンストラクタ
  ///
  /// # 引数
  /// - `user_id` - ユーザーID
  ///
  /// # 戻り値
  /// - `MusicLibrary` - 音楽ライブラリ
  pub fn new(user_id: UserID) -> Self {
    MusicLibrary {
      user_id,
      musics: Vec::new(),
    }
  }

  /// 音楽を追加する
  ///
  /// # 引数
  /// - `music`
  pub fn add_music(&mut self, music: Music) {
    self.musics.push(music);
  }

  /// 音楽を複数追加する
  /// 
  /// # 引数
  /// - `musics`
  pub fn add_musics(&mut self, musics: Vec<Music>) {
    self.musics.extend(musics);
  }

  /// 音楽を取得する
  /// 
  pub fn get_musics<'a>(&self) -> &'a Vec<Music> {
    &self.musics
  }

  /// 音楽を削除する
  ///
  /// # 引数
  /// - `music`
  pub fn remove_music(&mut self, music: Music) {
    self.musics.retain(|m| *m != music);
  }

  /// 音楽を複数削除する
  /// 
  /// # 引数
  /// - `musics`
  pub fn remove_musics(&mut self, musics: Vec<Music>) {
    self.musics.retain(|music| !musics.contains(music));
  }
}