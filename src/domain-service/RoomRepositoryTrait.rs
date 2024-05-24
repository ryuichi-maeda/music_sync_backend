
pub trait RoomRepositoryTrait {
    fn find_by_id(&self, id: RoomID) -> Result<Room, RoomRepositoryError>;
    fn save(&self, room: Room) -> Result<(), RoomRepositoryError>;
    fn delete(&self, room: Room) -> Result<(), RoomRepositoryError>;
    fn check_existence(&self, id: RoomID) -> Result<bool, RoomRepositoryError>;
}