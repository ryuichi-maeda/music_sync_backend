use async_graphql::{Context, Result as GqlResult, Subscription};
use futures_util::stream::Stream;
use std::time::Duration;

use crate::domain_model::room::room::Room;
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
        room_pin: String,
    ) -> impl Stream<Item = GqlResult<Room>> {
        let ctx = ctx.data::<Dependency>().unwrap();
        let repository = ctx.get_room_repository();
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;
                let room = repository.find_by_room_pin(room_pin.clone()).await;
                match room {
                    Ok(room) => {
                        yield Ok(room);
                    },
                    Err(err) => {
                        yield Err(anyhow::anyhow!(err.to_string()).into());
                    }
                }
            }
        }
    }
}
