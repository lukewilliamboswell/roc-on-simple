## Build and run

**note** change `libapp.dylib` to `libapp.so` or `app.lib` for linux or windows

```
$ roc build --lib --emit-llvm-ir --output libapp.dylib examples/simple.roc
$ cargo run
```

## Generate glue with

```
$ roc glue ../roc/crates/glue/src/RustGlue.roc temp-glue/ platform/main.roc
```
