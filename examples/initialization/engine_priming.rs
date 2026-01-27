//! Prime the Senzing engine for optimal performance

use std::time::Instant;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("sz-rust-sdk-priming-example")?;
    let engine = ExampleEnvironment::get_engine_with_setup(&env)?;

    let start_time = Instant::now();
    engine.prime_engine()?;
    let elapsed = start_time.elapsed();

    println!("Engine primed in {:.2}s", elapsed.as_secs_f64());

    let stats = engine.get_stats()?;
    println!("Stats: {}", stats);

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_priming() {
        if env::var("SENZING_ENGINE_CONFIGURATION_JSON").is_ok() {
            let result = main();
            assert!(result.is_ok(), "Engine priming should succeed");
        }
    }
}
