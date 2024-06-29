mod broker;
mod partition;
mod storage;
mod zookeeper;

use broker::broker::Broker;

fn main() {
    let broker = Broker::new();
    broker.start();
}
