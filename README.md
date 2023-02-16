# rust rf2 proxy
Simple WebSocket Proxy for rF2 Livetiming

Run with

```bash
cargo run
```

Build release variant with

```bash
cargo build -r
```
Connect to the WebSocket using the URL:
- `ws://127.0.0.1:8000/ws` as a client or
- `ws://127.0.0.1:8000/rf2ws/<servename>` as a server
