use std::time::Duration;
use async_trait::async_trait;
use tokio;
use tiny_tokio_actor::{Actor, ActorSystem, EventBus, Message, MultiHandler, SystemEvent};

// This example uses the multi actors, a type of actor that has multiple listeners and thus can have its handlers
// called concurrently. Note that this is inherently unsafe and all internal data of the actor should be synchronized properly.

#[derive(Debug, Clone)]
struct Event;
impl SystemEvent for Event {}

pub struct MyMultiActor {}

impl Actor<Event> for MyMultiActor {}

#[derive(Debug, Clone)]
struct MyMessage(String);

impl Message for MyMessage {
    type Response = ();
}

#[async_trait]
unsafe impl MultiHandler<Event, MyMessage> for MyMultiActor {
    async unsafe fn handle(&mut self, msg: MyMessage) -> () {
        println!("{:?}", msg);
        tokio::time::sleep(Duration::from_secs(5)).await
    }
}

#[tokio::main]
async fn main() {
    let bus = EventBus::new(100);
    let system = ActorSystem::new("multi_actor", bus);
    let a = system.create_multi_actor("multi", 4, MyMultiActor {}).await.unwrap();
    a.tell(MyMessage("m1".to_string())).unwrap();
    a.tell(MyMessage("m2".to_string())).unwrap();
    a.tell(MyMessage("m3".to_string())).unwrap();
    a.tell(MyMessage("m4".to_string())).unwrap();

    tokio::time::sleep(Duration::from_secs(100)).await;
}