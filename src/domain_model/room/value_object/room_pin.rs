use anyhow::Result;
use async_graphql::SimpleObject;
use rand::Rng;

#[derive(Debug, PartialEq, Eq, Clone, SimpleObject)]
pub struct RoomPin {
    value: String,
}

impl RoomPin {
    /// 000000から999999までの乱数を生成
    pub fn random_new() -> Self {
        RoomPin {
            value: rand::thread_rng().gen_range(0..999999).to_string(),
        }
    }

    pub fn new(value: String) -> Result<RoomPin> {
        if value.len() != 6 {
            return Err(anyhow::anyhow!(
                "Invalid room pin value. It must be 6 digits."
            ));
        }

        match value.parse::<u32>() {
            Ok(_) => Ok(RoomPin { value: value }),
            Err(_) => Err(anyhow::anyhow!(
                "Invalid room pin value. It must be a number."
            )),
        }
    }

    pub fn new_from_i32(value: i32) -> Result<RoomPin> {
        if value < 0 || value > 999999 {
            return Err(anyhow::anyhow!(
                "Invalid room pin value. It must be between 0 and 999999."
            ));
        }
        Ok(RoomPin {
            value: value.to_string(),
        })
    }

    pub fn raw_value(&self) -> String {
        self.value.clone()
    }
}
