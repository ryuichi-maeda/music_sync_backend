pub trait UserRepositoryTrait {
    fn find_by_id(&self, id: UserID) -> Result<User, UserRepositoryError>;
    fn find_all(&self) -> Result<Vec<User>, UserRepositoryError>;
    fn save(&self, user: User) -> Result<(), UserRepositoryError>;
    fn update(&self, user: User) -> Result<(), UserRepositoryError>;
    fn delete(&self, user: User) -> Result<(), UserRepositoryError>;
    fn check_existence(&self, id: UserID) -> Result<bool, UserRepositoryError>;
}