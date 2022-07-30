use tracing::Level;

pub fn init_trace() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // builds the subscriber.
        .finish();

    // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let _ = tracing::subscriber::set_global_default(subscriber);
}
