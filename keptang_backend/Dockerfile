FROM rust
WORKDIR /app
COPY . .
RUN cargo build --release

# ใช้ port ไร
EXPOSE 8080
ENTRYPOINT [ "./target/release/web-server" ]