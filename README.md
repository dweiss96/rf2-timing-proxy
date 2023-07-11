# rust rf2 proxy
Simple WebSocket Proxy for rF2 Livetiming

Run with

```bash
cargo run
```

## Relese build
### Install Zig
See https://ziglang.org/download or https://github.com/ziglang/zig/wiki/Install-Zig-from-a-Package-Manager

### Install zigbuild cargo script
https://github.com/rust-cross/cargo-zigbuild

### Build all release variants 
use the `build_for_all_targets_zigbuild` script

```
Connect to the WebSocket using the URL:
- `ws://127.0.0.1:8000/ws` as a client or
- `ws://127.0.0.1:8000/rf2ws/<servename>` as a server
