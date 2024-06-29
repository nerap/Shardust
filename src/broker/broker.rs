use uuid::Uuid;

pub struct Broker {
    uuid: Uuid,
}


impl Broker {
    pub fn new() -> Broker {
        Broker {
            uuid: Uuid::new_v4(),
        }
    }

    pub fn start(&self) -> () {
        println!("Broker {} started", self.uuid);
    }
}
