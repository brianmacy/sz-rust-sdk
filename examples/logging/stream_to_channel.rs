//! Stream Senzing log messages to a dev console through a bounded channel.
//!
//! The callback never blocks the logging thread: `try_send` drops a message when the console falls
//! behind, and the drops are counted. A writer thread prints JSONL lines (a desktop app would read
//! them from this process's stdout). Dropping the registration drops the closure and its sender,
//! which ends the writer loop.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("logging-stream-to-channel")?;
    let logging = env.get_logging()?;

    let (tx, rx) = mpsc::sync_channel::<String>(1024);
    let dropped = Arc::new(AtomicUsize::new(0));
    let drops = Arc::clone(&dropped);

    // Senzing 4.5.0: register before the engine is initialized.
    let options = SzLogCallbackOptions::new(SzLogLevel::Debug).format(SzLogFormat::Jsonl);
    let registration = logging.register(options, move |record| {
        if tx.try_send(record.message().into_owned()).is_err() {
            drops.fetch_add(1, Ordering::Relaxed);
        }
    })?;

    // Writer thread (println! locks stdout per line, so other threads can still print).
    let writer = thread::spawn(move || {
        let mut lines = 0usize;
        for line in rx {
            println!("[dev-console] {line}");
            lines += 1;
        }
        lines
    });

    let engine = env.get_engine()?;
    for i in 0..5 {
        engine.add_record(
            "TEST",
            &format!("STREAM_{i}"),
            r#"{"NAME_FULL": "Jane Doe"}"#,
            None,
        )?;
    }
    drop(engine);
    ExampleEnvironment::cleanup(env)?;
    drop(registration); // closes the channel: the writer drains and exits

    let lines = writer.join().expect("writer thread");
    println!(
        "streamed {lines} lines, dropped {}",
        dropped.load(Ordering::Relaxed)
    );
    Ok(())
}
