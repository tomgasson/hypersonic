mod platform;
mod core;
mod resolver;
mod transformation;
mod default_plugins;
mod utils;

use std::sync::Arc;

use dashmap::DashMap;

use crate::core::Asset;
use crate::platform::Args;
use crate::transformation::transform;
use crate::utils::{StandardProfiler, StaticContainer};


fn main() {
    let args = Args::new();

    println!("ENTRY:     {:?}", args.entry);
    println!("LOGGING:   {:?}", args.log_level);
    println!("PROFILING: {}", args.profiling);
    println!("THREADS:   {}", args.threads);
    println!("");

    let assets = StaticContainer::<Asset>::new(10_000_000);
    let assets_index = Arc::new(DashMap::<String, usize>::new());
    let profiler = StandardProfiler::new();

    let profiler_end = profiler.start();

    transform(
        &args,
        assets.clone(),
        assets_index.clone(),
        &profiler,
    );

    profiler_end("build-time-total");

    println!("Performance Breakdown:");
    println!("  Total Time:      {:.5} s (total)", profiler.get_seconds("build-time-total"));
    println!("  Total Assets:    {}", assets.len());

    if args.profiling {
        println!("  Transformation:  {:.5} s (total)", profiler.get_seconds("Transformation"));
        println!("    CreateAsset:   {:.5} ms (average)", profiler.get_milli("CreateAsset"));
        println!("    ReadContents:  {:.5} ms (average)", profiler.get_milli("ReadContents"));
        println!("    AssignPattern: {:.5} ms (average)", profiler.get_milli("TransformAssignPattern"));
        println!("    Transformers:");

        let mut profiles: Vec<String> = profiler.get_profiles().clone();
        profiles.sort();
        for profile in  profiles {
            if profile.starts_with("TransformContents - ") {
                let title = profile.replace("TransformContents - ", "");
                let perf = profiler.get_milli(&profile);
                println!("      {}: {:.5} ms (average)", title, perf);
            }
        }
    }
}
