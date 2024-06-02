
pub trait MusicLibraryRepositoryTrait {
    fn find_by_id(&self, id: MusicLibraryID) -> Result<MusicLibrary, MusicLibraryRepositoryError>;
    fn find_by_user_id(&self, user_id: UserID) -> Result<Vec<MusicLibrary>, MusicLibraryRepositoryError>;
    fn find_all(&self) -> Result<Vec<MusicLibrary>, MusicLibraryRepositoryError>;
    fn save(&self, music_library: MusicLibrary) -> Result<(), MusicLibraryRepositoryError>;
    fn delete(&self, music_library: MusicLibrary) -> Result<(), MusicLibraryRepositoryError>;
}