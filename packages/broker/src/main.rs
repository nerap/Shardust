mod broker;

use broker::broker::Broker;

fn main() {
    let broker = Broker::new();
    broker.start();
}
