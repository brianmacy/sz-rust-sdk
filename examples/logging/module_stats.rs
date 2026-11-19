//! Count Senzing log messages per module and level, for selected modules.
//!
//! The filter and the counting are ordinary user code inside the closure; native does the rest.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

type Stats = BTreeMap<(String, SzLogLevel), usize>;

fn main() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("logging-module-stats")?;
    let logging = env.get_logging()?;

    // Senzing 4.5.0: register before the engine is initialized.
    let stats: Arc<Mutex<Stats>> = Arc::default();
    let sink = Arc::clone(&stats);
    let options = SzLogCallbackOptions::new(SzLogLevel::Trace).format(SzLogFormat::Bare);
    let registration = logging.register(options, move |record| {
        let module = record.module();
        if module.starts_with("sz") {
            let mut stats = sink.lock().unwrap_or_else(|e| e.into_inner());
            *stats
                .entry((module.into_owned(), record.level()))
                .or_default() += 1;
        }
    })?;

    let engine = env.get_engine()?;
    for i in 0..10 {
        let record = format!(r#"{{"NAME_FULL": "Person {i}", "PHONE_NUMBER": "555-01{i:02}"}}"#);
        engine.add_record("TEST", &format!("STATS_{i}"), &record, None)?;
    }
    drop(engine);
    ExampleEnvironment::cleanup(env)?;
    drop(registration); // after destroy: nothing else logs

    println!("{:<20} {:<8} {:>6}", "MODULE", "LEVEL", "COUNT");
    for ((module, level), count) in stats.lock().unwrap_or_else(|e| e.into_inner()).iter() {
        println!("{module:<20} {level:<8} {count:>6}");
    }
    Ok(())
}
