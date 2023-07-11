FROM alpine:latest
ADD target/x86_64-apple-darwin/release/rust_rf2_proxy /rust_rf2_proxy
EXPOSE 5398
CMD ["/rust_rf2_proxy"]
