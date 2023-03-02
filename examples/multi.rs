use std::time::Duration;
use async_trait::async_trait;
use tokio;
use tiny_tokio_actor::{Actor, ActorContext, ActorSystem, EventBus, Handler, Message, ConcurrentHandler, SystemEvent};

// This example uses the multi actors, a type of actor that has multiple listeners and thus can have its handlers
// called concurrently. Note that this is inherently unsafe and all internal data of the actor should be synchronized properly.

#[derive(Debug, Clone)]
struct Event;
impl SystemEvent for Event {}

#[derive(Actor)]
pub struct MyConcurrentActor {}

// Some normal comment
#[derive(Debug, Clone, Message)]
#[response(String)]
struct MyMessage(String);

#[async_trait]
unsafe impl ConcurrentHandler<Event, MyMessage> for MyConcurrentActor {
    async unsafe fn handle(&mut self, msg: MyMessage) -> String {
        println!("{:?}", msg);
        tokio::time::sleep(Duration::from_secs(5)).await;
        "aye".into()
    }
}

#[tokio::main]
async fn main() {
    let bus = EventBus::new(100);
    let system = ActorSystem::new("multi_actor", bus);
    let a = system.create_concurrent_actor(
        "multi",
        4,
        MyConcurrentActor {})
        .await
        .unwrap();
    a.tell(MyMessage("m1".to_string())).unwrap();
    a.tell(MyMessage("m2".to_string())).unwrap();
    a.tell(MyMessage("m3".to_string())).unwrap();
    a.tell(MyMessage("m4".to_string())).unwrap();

    tokio::time::sleep(Duration::from_secs(100)).await;
}