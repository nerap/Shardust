mod broker;
mod zookeeper;
mod partition;
mod storage;

use crate::broker::broker::Broker;

fn main() {
    let broker = Broker::new();
    broker.start();
}
