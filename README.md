# Hypersonic Bundler

_This does not yet produce an output bundle._

Full credit to the Parcel team as this is heavily inspired by Parcel.

This is an experimental web bundler that aims to explore how to and the benefits of parallelizing the bundling pipeline. It's not intended that this be used for any production use cases, it's simply a scratch research project.

## Approach

This project is written in Rust and intends to explore the benefit of parallelizing as much work as possible. The hope is that maximizing the utilization of the host hardware will result in faster build times.

Exactly like Parcel, this follows a middleware "plugins" approach to inform steps of the bundle pipeline.

The core is an orchestrator that coordinates feeding data into these middlewares and the result is distributable web assets - html/js/css files.

## Usage

```bash
cargo build
./target/debug/hypersonic ./fixtures/basic/index.html
```

Configuring logging and threads

```bash
env \
  HS_THREADS=1 \
  HS_LOG_LEVEL=3 \
  ./target/debug/hypersonic ./fixtures/basic/index.html
```

Which will produce an output that looks like this:
```
ENTRY:     "/home/dalsh/Development/alshdavid/hypersonic/./fixtures/basic/index.html"
LOGGING:   Verbose
PROFILING: true
THREADS:   1

T0: EntryAsset("/home/dalsh/Development/alshdavid/hypersonic/./fixtures/basic/index.html")
T0: ReadContents(0)
T0: AssignTransformers(0)
T0: TransformContents(0, 0)
T0: CreateAsset("/home/dalsh/Development/alshdavid/hypersonic/./fixtures/basic/./scripts/index.js")
T0: Done(0)
T0: ReadContents(1)
T0: AssignTransformers(1)
T0: TransformContents(1, 0)
T0: CreateAsset("/home/dalsh/Development/alshdavid/hypersonic/fixtures/basic/scripts/b.js")
T0: Done(1)
T0: ReadContents(2)
T0: AssignTransformers(2)
T0: TransformContents(2, 0)
T0: Done(2)
T0: CLOSED

Performance Breakdown:
  Total Time:      0.00242 s (total)
  Total Assets:    3
  Transformation:  0.00239 s (total)
    CreateAsset:   0.03935 ms (average)
    ReadContents:  0.01436 ms (average)
    AssignPattern: 0.00000 ms (average)
    Transformers:
      DefaultHTMLTransformer: 0.24647 ms (average)
      DefaultJSTransformer javascript: 0.81248 ms (average)
```