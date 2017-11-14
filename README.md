# ohlc-rs
OHLC chart generator

Put this in your `Cargo.toml`
```toml
[dependencies.ohlc]
git = "https://github.com/UninterestinAcc/ohlc-rs"
```

## Quick start
```rust
OHLCRenderOptions::new().render_ohlc(data, |p| {});
```
* `OHLCRenderOptons::new()` generates rendering options
* `.render_ohlc(...)` renders the chart
* `data` should be a vector of the provided OHLC object
* `p` is a reference to a path
* `|...| {...}` the callback function which you can code in. **Note:** *The image located at the path is destroyed once the callback function exits, so don't do anything async with the path.*
