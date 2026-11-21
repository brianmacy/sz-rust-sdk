//! Toggle Senzing logging verbosity at runtime from a signal (SIGUSR1 = verbose, SIGUSR2 = quiet).
//!
//! The signal handler only flips an atomic flag (async-signal-safe); a watcher thread applies it.
//! `set_default_log_level` is safe at any time, from any thread, so it works on every Senzing version.
//! Per-callback levels (`SzLogRegistration::set_level`) need Senzing >= 4.5.1 / 4.6.0.
//!
//! Try it by hand: `kill -USR1 <pid>`. This demo raises the signals itself so it ends on its own.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

static VERBOSE: AtomicBool = AtomicBool::new(false);
static STOP: AtomicBool = AtomicBool::new(false);

#[cfg(unix)]
extern "C" fn on_signal(signal: libc::c_int) {
    VERBOSE.store(signal == libc::SIGUSR1, Ordering::SeqCst);
}

/// Requests a toggle the way an operator would (a signal on Unix, the flag directly elsewhere).
fn request(verbose: bool) {
    #[cfg(unix)]
    // SAFETY: raising a signal whose handler only stores to an atomic.
    unsafe {
        libc::raise(if verbose {
            libc::SIGUSR1
        } else {
            libc::SIGUSR2
        });
    }
    #[cfg(not(unix))]
    VERBOSE.store(verbose, Ordering::SeqCst);
}

fn main() -> SzResult<()> {
    #[cfg(unix)]
    // SAFETY: installs an async-signal-safe handler (it only stores to an atomic).
    unsafe {
        let handler = on_signal as extern "C" fn(libc::c_int) as libc::sighandler_t;
        libc::signal(libc::SIGUSR1, handler);
        libc::signal(libc::SIGUSR2, handler);
    }

    let env = ExampleEnvironment::initialize("logging-verbose-toggle")?;
    let logging: Arc<dyn SzLogging> = Arc::from(env.get_logging()?);

    // Senzing 4.5.0: change the default output only before the engine is initialized.
    // The default output is our callback, so set_default_log_level controls what it receives.
    let delivered = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&delivered);
    logging.set_default(SzLogLevel::Error, move |_record| {
        counter.fetch_add(1, Ordering::Relaxed);
    })?;
    logging.set_default_log_level(SzLogLevel::Error)?;

    // Watcher thread: applies the flag. Never calls engine methods.
    let watcher_logging = Arc::clone(&logging);
    let watcher = thread::spawn(move || {
        let mut applied = false;
        while !STOP.load(Ordering::SeqCst) {
            let verbose = VERBOSE.load(Ordering::SeqCst);
            if verbose != applied {
                let level = if verbose {
                    SzLogLevel::Trace
                } else {
                    SzLogLevel::Error
                };
                if let Err(e) = watcher_logging.set_default_log_level(level) {
                    eprintln!("set_default_log_level failed: {e}");
                }
                applied = verbose;
            }
            thread::sleep(Duration::from_millis(20));
        }
    });

    let engine = env.get_engine()?;
    for (phase, verbose) in [("quiet", false), ("verbose", true), ("quiet again", false)] {
        request(verbose);
        wait_for_level(
            logging.as_ref(),
            if verbose {
                SzLogLevel::Trace
            } else {
                SzLogLevel::Error
            },
        );
        let before = delivered.load(Ordering::Relaxed);
        for i in 0..5 {
            let id = format!("{phase}-{i}");
            engine.add_record("TEST", &id, r#"{"NAME_FULL": "Robert Smith"}"#, None)?;
        }
        println!(
            "{phase:<12} {} messages delivered",
            delivered.load(Ordering::Relaxed) - before
        );
    }

    STOP.store(true, Ordering::SeqCst);
    watcher.join().expect("watcher thread");
    drop(engine);
    ExampleEnvironment::cleanup(env)?;

    // After destroy nothing logs, so the default output may change again.
    logging.set_default_output(SzLogOutput::Stderr)?;

    // Per-callback toggling on a registration handle (Senzing >= 4.5.1 / 4.6.0).
    let registration = logging.register(SzLogLevel::Error, |r| eprintln!("{r}"))?;
    match registration.set_level(SzLogLevel::Trace) {
        Ok(()) => println!("registration level is now {}", registration.level()?),
        Err(e) => println!("per-callback level: {e}"),
    }
    drop(registration);
    Ok(())
}

/// Waits until the watcher has applied `level` (the effective level reads it back).
fn wait_for_level(logging: &dyn SzLogging, level: SzLogLevel) {
    let start = Instant::now();
    while logging.get_effective_log_level().ok() != Some(level)
        && start.elapsed() < Duration::from_secs(5)
    {
        thread::sleep(Duration::from_millis(10));
    }
}
