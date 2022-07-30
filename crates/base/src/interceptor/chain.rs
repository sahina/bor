use std::error::Error;

use async_trait::async_trait;

use crate::message::Message;

///  The interceptor chain manages the flow of a message through a chain of interceptors and ultimately to the message
/// handler. Interceptors may continue processing via this chain by calling the `proceed()`
/// method. Alternatively, they can block processing by returning without calling either of these
/// methods.
#[async_trait]
pub trait InterceptorChain<T: Message> {
    type Error: Error;

    /// Signals the Interceptor Chain to continue processing the message.
    /// The return value of the message processing
    async fn proceed(&self) -> Result<T, Self::Error>;
}
