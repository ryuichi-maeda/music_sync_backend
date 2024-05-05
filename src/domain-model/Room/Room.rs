#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Room {
  room_pin: RoomPin,
  members: [User],
  host: User,
  is_active: bool,
  is_confirm: bool,
}

impl Room {
  pub fn new(room_pin: RoomPin, host: User) {
    Room {
      room_pin,
      members: vec![host],
      host,
      is_active: true,
      is_confirm: false,
    }
  }

  pub fn enter_room(&mut self, user: &User) {
    self.members.append(user.clone())
  }

  pub fn confirm_room(&mut self) {
    self.is_confirm = true
  }

  pub fn exit_room(&mut self, user: &User) {
    if let Some(index) = self.members.iter().position(|x| *x == *user) {  
      self.members.remove(index);  
    } 
  }

  pub fn close_room(&mut self) {
    self.is_active = false
  }

  pub fn get_users(&self) -> Vec<User> {
    self.members
  }



}