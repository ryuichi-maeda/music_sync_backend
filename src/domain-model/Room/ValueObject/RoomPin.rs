use rand::Rng;

pub struct RoomPin {
  value: u32
}

/// 000000から999999までの乱数を生成
impl RoomPin {
  pub fn new() {
    rand::thread_rng().gen_range(000000,1000000)
  }
}