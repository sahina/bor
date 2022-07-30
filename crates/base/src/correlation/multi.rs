use std::fmt::{Debug, Formatter};

use crate::correlation::CorrelationProvider;
use crate::ddd::metadata::MetaData;
use crate::message::generic::EventMessage;

/// `MultiCorrelationProvider` implementation defines correlation headers by the header names.
/// The headers from messages with these keys are returned as correlation data.
// #[derive(Debug)]
pub struct MultiCorrelationProvider {
    delegates: Vec<Box<dyn CorrelationProvider<EventMessage>>>,
}

impl Debug for MultiCorrelationProvider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MultiCorrelationProvider")
    }
}

impl MultiCorrelationProvider {
    pub fn new(delegates: Vec<Box<dyn CorrelationProvider<EventMessage>>>) -> Self {
        MultiCorrelationProvider { delegates }
    }
}

impl CorrelationProvider<EventMessage> for MultiCorrelationProvider {
    fn correlation_for(&self, message: EventMessage) -> MetaData {
        let mut data = MetaData::new();

        self.delegates.iter().for_each(|provider| {
            let correlation_data = provider.correlation_for(message.clone());
            data.extend(correlation_data);
        });

        data
    }
}

#[cfg(test)]
mod multi_correlation_test {
    use crate::correlation::multi::MultiCorrelationProvider;
    use crate::correlation::originator::OriginatorCorrelationProvider;
    use crate::correlation::simple::SimpleCorrelationProvider;
    use crate::correlation::CorrelationProvider;
    use crate::ddd::constants::{CORRELATION_ID_KEY, TRACE_ID_KEY};
    use crate::message::generic::EventMessage;

    #[test]
    fn test_multi_correlation() {
        let message = EventMessage::new("SomethingHappened", "event-content")
            .add_meta(TRACE_ID_KEY, "message-trace-id")
            .add_meta("key1", "value1")
            .add_meta("key2", "value2")
            .add_meta("key3", "value3")
            .add_meta("key4", "value4");

        let simple_1 = SimpleCorrelationProvider::new(vec!["key1".to_owned()]);
        let simple_2 = SimpleCorrelationProvider::new(vec!["key2".to_owned()]);
        let originator = OriginatorCorrelationProvider::new("cor-1", "trace-id");
        let delegates: Vec<Box<dyn CorrelationProvider<EventMessage>>> =
            vec![Box::new(simple_1), Box::new(simple_2), Box::new(originator)];

        let multi = MultiCorrelationProvider::new(delegates);

        let data = multi.correlation_for(message);

        assert_eq!(data.get(TRACE_ID_KEY).unwrap(), "message-trace-id");
        assert_eq!(data.get(CORRELATION_ID_KEY).unwrap(), "cor-1");
        assert!(data.get("key1").is_some());
        assert!(data.get("key2").is_some());
    }
}
