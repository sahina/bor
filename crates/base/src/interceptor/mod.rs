use std::error::Error;

use async_trait::async_trait;

use crate::message::Message;

pub mod chain;

/// Message interceptors are used to intercept and modify messages as they flow through the
/// system. This can be useful for a variety of purposes, such as logging, authentication, or authorization.
#[async_trait]
pub trait MessageHandlerInterceptor<T: Message>: Send + Sync {
    type Error: Error;
    /// Invoked before a Message is handled by a designated `MessageHandler<T>`.
    /// The interceptor is responsible for the continuation of the handling process by invoking the
    /// `InterceptorChain::proceed()`
    async fn handle(&self, handle: T) -> Result<T, Self::Error>;
}
