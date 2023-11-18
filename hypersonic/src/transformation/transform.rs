use std::fs;
use std::ops::IndexMut;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::sync::atomic::{AtomicUsize, Ordering};

use dashmap::DashMap;

use crate::{core::Asset, utils::StandardProfiler};
use crate::platform::Args;
use crate::utils::{StaticContainer, hash_path_buff_sha_256};
use crate::utils::Queue;
use crate::default_plugins::{DefaultHTMLTransformer, DefaultJSTransformer, DefaultNoopTransformer};

use super::{TransformerContainer, TransformerContext};
use super::actions::Action;

pub fn transform(
    args: &Args,
    assets: Arc<StaticContainer<Asset>>,
    assets_index: Arc<DashMap<String, usize>>,
    profiler: &StandardProfiler,
) {
    let mut handles = Vec::<JoinHandle<()>>::new();
    
    let (queue, mut receivers) = Queue::<Action>::new(args.threads);
    let in_pipeline = Arc::new(AtomicUsize::new(0));

    // Entry asset
    queue.push(Action::EntryAsset(args.entry.clone()));
    in_pipeline.fetch_add(1, Ordering::Acquire);

    let profiler_end_transformations = profiler.start();

    for t in 0..args.threads {
        let args = args.clone();
        let assets = assets.clone();
        let assets_index = assets_index.clone();
        let queue = queue.clone();
        let in_pipeline = in_pipeline.clone();
        let receiver = receivers.index_mut(t).take().unwrap();
        let profiler = profiler.clone();

        handles.push(thread::spawn(move || {
            let mut transformers = TransformerContainer::new();

            transformers.add("*.html", Box::new(DefaultHTMLTransformer::new()));
            transformers.add("*.js", Box::new(DefaultJSTransformer::new(false, false)));
            transformers.add("*.jsx", Box::new(DefaultJSTransformer::new(true, false)));
            transformers.add("*.ts", Box::new(DefaultJSTransformer::new(false, true)));
            transformers.add("*.tsx", Box::new(DefaultJSTransformer::new(true, true)));
            transformers.add("*.css", Box::new(DefaultNoopTransformer::new()));

            // let mut hits = 0;
            loop {
                if in_pipeline.load(Ordering::Acquire) == 0 {
                    queue.disconnect_all();
                    break;
                }

                let action_opt = queue.recv(&receiver);
                if action_opt.is_none() {
                    break;
                }
                let action = action_opt.unwrap();

                if args.log_level.is_verbose() {
                    println!("T{}: {:?}", t, action);
                }

                let profiler_end = profiler.start();

                match action {
                    Action::EntryAsset(file_path) => {
                        let hash = hash_path_buff_sha_256(&file_path);
                        let id = assets.push(Asset {
                            file_path,
                            transformer_pattern: String::from(""),
                            content: String::from(""),
                            content_hash: String::from(""),
                        });
                        assets_index.insert(hash, id);
                        if args.profiling {
                            profiler_end("CreateAsset");
                        }
                        queue.push(Action::ReadContents(id));
                    },
                    Action::CreateAsset(file_path) => {
                        let hash = hash_path_buff_sha_256(&file_path);
                        if assets_index.contains_key(&hash) {
                            continue;
                        }
                        in_pipeline.fetch_add(1, Ordering::Acquire);

                        let id = assets.push(Asset {
                            file_path,
                            transformer_pattern: String::from(""),
                            content: String::from(""),
                            content_hash: String::from(""),
                        });
                        assets_index.insert(hash, id);

                        if args.profiling {
                            profiler_end("CreateAsset");
                        }
                        queue.push(Action::ReadContents(id));
                    }
                    Action::ReadContents(id) => {
                        let asset_container = assets.index(id);
                        let container_result = asset_container.lock();
                        let mut container = container_result.unwrap();
                        let asset = container.get_value_mut().unwrap();

                        let content = fs::read_to_string(&asset.file_path);
                        if content.is_err() {
                            panic!("Unable to read file: {:?}", &asset.file_path);
                        }

                        asset.content = content.unwrap();

                        if args.profiling {
                            profiler_end("ReadContents");
                        }
                        queue.push(Action::AssignTransformers(id));
                    }
                    Action::AssignTransformers(id) => {
                        let asset_container = assets.index(id);
                        let container_result = asset_container.lock();
                        let mut container = container_result.unwrap();
                        let asset = container.get_value_mut().unwrap();

                        let pattern_result = transformers.match_pattern(&asset.file_path);
                        if pattern_result.is_err() {
                            panic!("No transformers match {:?}", &asset.file_path.file_name().unwrap());
                        }

                        asset.transformer_pattern = pattern_result.unwrap();

                        if args.profiling {
                            profiler_end("AssignTransformers");
                        }
                        queue.push(Action::TransformContents(id, 0));
                    }
                    Action::TransformContents(id, index) => {
                        let asset_container = assets.index(id);
                        let container_result = asset_container.lock();
                        let mut container = container_result.unwrap();
                        let asset = container.get_value_mut().unwrap();

                        let transformer_opt = transformers.index(&asset.transformer_pattern, index);
                        if transformer_opt.is_none() {
                            queue.push(Action::Done(id));
                            continue;
                        }
                        let transformer = transformer_opt.unwrap();

                        let ctx = Box::new(TransformerContext {
                            log_level: args.log_level.clone(),
                            queue: queue.clone(),
                        });

                        let result = transformer.transform(&ctx, asset);
                        if result.is_err() {
                            let err = result.err();
                            panic!("Error at: {}\n{}", transformer.get_name(), err);
                        }

                        if args.profiling {
                            profiler_end(&format!("TransformContents - {}", transformer.get_name()));
                        }
                        let next_transformer = index + 1;
                        if transformers.index(&asset.transformer_pattern, next_transformer).is_none() {
                            queue.push(Action::Done(id));
                            continue;
                        } 
                        queue.push(Action::TransformContents(id, next_transformer));

                    }
                    Action::Done(_) => {
                        in_pipeline.fetch_sub(1, Ordering::Relaxed);
                    }
                }
                // hits += 1;
            }
            if args.log_level.is_verbose() {
                println!("T{}: CLOSED", t);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    if args.log_level.is_verbose() {
        println!("");
    }

    profiler_end_transformations("Transformation");
}
