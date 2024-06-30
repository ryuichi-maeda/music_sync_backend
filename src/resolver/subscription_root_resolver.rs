use async_graphql::{Context, Subscription};
use futures_util::stream::Stream;
use std::time::Duration;

use crate::domain_model::room::room::Room;
use crate::domain_model::room::room_updated::RoomUpdated;
use crate::domain_model::room::value_object::room_pin::RoomPin;
use crate::domain_model::user::guest_user::GuestUser;
use crate::domain_model::user::user::User;
use crate::domain_model::user::value_object::user_name::UserName;
use crate::Dependency;

// Subscription
pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    pub async fn interval(&self, #[graphql(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;
                value += n;
                yield value;
            }
        }
    }

    pub async fn room_updated<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        room_id: String,
    ) -> impl Stream<Item = RoomUpdated> {
        let ctx = ctx.data::<Dependency>().unwrap();
        let repository = ctx.get_room_repository();
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;
                let room = repository.find_by_id(room_id.clone()).await;
                if room.is_err() {
                    continue;
                } else {
                    let room = room.unwrap();
                    let room_updated = RoomUpdated::new(room);
                    yield room_updated;
                }
            }
        }
    }
}
