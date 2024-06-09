mod MusicLibraryRepositoryTrait;

use MusicLibraryRepositoryTrait::MusicLibraryRepositoryTrait;

pub struct MusicLibraryRepository;

impl MusicLibraryRepository {
    pub fn new() -> Self {
        MusicRepository
    }
}

// TODO: 実装する
impl MusicLibraryRepositoryTrait for MusicLibraryRepository {
    fn find_by_id(&self, id: MusicLibraryID) -> Result<MusicLibrary, MusicLibraryRepositoryError> {
        Ok(MusicLibrary {
            id: id,
            user_id: UserID::new(),
            music_name: MusicName::new(),
            artist_name: ArtistName::new(),
            genre: Genre::new(),
            release_date: ReleaseDate::new(),
            created_at: CreatedAt::new(),
            updated_at: UpdatedAt::new(),
            is_deleted: false,
        })
    }

    fn find_by_user_id(&self, user_id: UserID) -> Result<Vec<MusicLibrary>, MusicLibraryRepositoryError> {
        Ok(vec![MusicLibrary {
            id: MusicLibraryID::new(),
            user_id: user_id,
            music_name: MusicName::new(),
            artist_name: ArtistName::new(),
            genre: Genre::new(),
            release_date: ReleaseDate::new(),
            created_at: CreatedAt::new(),
            updated_at: UpdatedAt::new(),
            is_deleted: false,
        }])
    }

    fn find_all(&self) -> Result<Vec<MusicLibrary>, MusicLibraryRepositoryError> {
        Ok(vec![MusicLibrary {
            id: MusicLibraryID::new(),
            user_id: UserID::new(),
            music_name: MusicName::new(),
            artist_name: ArtistName::new(),
            genre: Genre::new(),
            release_date: ReleaseDate::new(),
            created_at: CreatedAt::new(),
            updated_at: UpdatedAt::new(),
            is_deleted: false,
        }])
    }

    fn save(&self, music_library: MusicLibrary) -> Result<(), MusicLibraryRepositoryError> {
        Ok(())
    }

    fn delete(&self, music_library: MusicLibrary) -> Result<(), MusicLibraryRepositoryError> {
        Ok(())
    }
}

