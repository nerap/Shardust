#[cfg(test)]
mod tests {
    use crate::broker::broker::Broker;

    #[test]
    fn test_broker_initialization() {
        let broker = Broker::new();
        assert_eq!(broker.uuid.is_nil(), false);
    }
}
