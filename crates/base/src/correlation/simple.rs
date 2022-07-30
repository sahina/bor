use crate::correlation::CorrelationProvider;
use crate::ddd::metadata::MetaData;
use crate::message::generic::EventMessage;
use crate::message::Message;

/// `SimpleCorrelationProvider` implementation defines correlation headers by the header names.
/// The headers from messages with these keys are returned as correlation data.
#[derive(Debug)]
pub struct SimpleCorrelationProvider {
    header_names: Vec<String>,
}

impl SimpleCorrelationProvider {
    pub fn new(header_names: impl Into<Vec<String>>) -> Self {
        SimpleCorrelationProvider {
            header_names: header_names.into(),
        }
    }
}

impl CorrelationProvider<EventMessage> for SimpleCorrelationProvider {
    fn correlation_for(&self, message: EventMessage) -> MetaData {
        let mut data = MetaData::new();
        let metadata = message.metadata();

        self.header_names
            .iter()
            .filter(|header| metadata.contains_key(*header))
            .for_each(|header| {
                let val = metadata.get(header);
                data.add(header.clone(), val);
            });

        data
    }
}

#[cfg(test)]
mod simple_correlation_test {
    use crate::correlation::simple::SimpleCorrelationProvider;
    use crate::correlation::CorrelationProvider;
    use crate::message::generic::EventMessage;

    #[test]
    fn test_simple_correlation() {
        let message = EventMessage::new("SomethingHappened", "event-content")
            .add_meta("key1", "value1")
            .add_meta("key2", "value2");
        let correlation = SimpleCorrelationProvider::new(vec!["key1".to_owned()]);

        let data = correlation.correlation_for(message);

        assert_eq!(data.len(), 1);
        assert!(data.get("key1").is_some());
    }
}
