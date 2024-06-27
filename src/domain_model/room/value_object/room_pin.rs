use rand::Rng;

#[derive(Debug, PartialEq, Eq)]
pub struct RoomPin {
    value: u32,
}

/// 000000から999999までの乱数を生成
impl RoomPin {
    pub fn new() -> Self {
        RoomPin {
            value: rand::thread_rng().gen_range(0..999999),
        }
    }
}
