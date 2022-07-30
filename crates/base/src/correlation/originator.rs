use serde_json::json;

use crate::correlation::CorrelationProvider;
use crate::ddd::constants::{CORRELATION_ID_KEY, TRACE_ID_KEY};
use crate::ddd::metadata::MetaData;
use crate::message::generic::EventMessage;
use crate::message::Message;

/// `OriginatorCorrelationProvider` provides message identifier to
///  other messages that are created as result of processing the first message.
#[derive(Debug)]
pub struct OriginatorCorrelationProvider {
    correlation_id: String,
    trace_id: String,
}

impl OriginatorCorrelationProvider {
    pub fn new(correlation_id: impl Into<String>, trace_id: impl Into<String>) -> Self {
        Self {
            correlation_id: correlation_id.into(),
            trace_id: trace_id.into(),
        }
    }
}

impl CorrelationProvider<EventMessage> for OriginatorCorrelationProvider {
    fn correlation_for(&self, message: EventMessage) -> MetaData {
        let correlation_id = self.correlation_id.clone();
        let trace_id = self.trace_id.clone();

        let trace_id = message
            .metadata()
            .get(TRACE_ID_KEY)
            .unwrap_or(json!(trace_id));

        MetaData::new()
            .insert(CORRELATION_ID_KEY, correlation_id)
            .insert(TRACE_ID_KEY, trace_id)
    }
}

#[cfg(test)]
mod originator_correlation_test {
    use crate::correlation::originator::OriginatorCorrelationProvider;
    use crate::correlation::CorrelationProvider;
    use crate::ddd::constants::TRACE_ID_KEY;
    use crate::message::generic::EventMessage;

    #[test]
    fn test_origin_correlation() {
        let message = EventMessage::new("SomethingHappened", "event-content")
            .add_meta("key1", "value1")
            .add_meta(TRACE_ID_KEY, "message-trace-id");
        let correlation = OriginatorCorrelationProvider::new("cor-1", "trace-id");

        let data = correlation.correlation_for(message);

        assert!(data.get(TRACE_ID_KEY).is_some());
        assert_eq!(data.get(TRACE_ID_KEY).unwrap(), "message-trace-id");
    }

    #[test]
    fn test_origin_correlation_no_trace() {
        // message has no trace id
        let message =
            EventMessage::new("SomethingHappened", "event-content").add_meta("key1", "value1");
        let correlation = OriginatorCorrelationProvider::new("cor-1", "trace-id");

        let data = correlation.correlation_for(message);

        assert!(data.get(TRACE_ID_KEY).is_some());
        assert_eq!(data.get(TRACE_ID_KEY).unwrap(), "trace-id");
    }
}
