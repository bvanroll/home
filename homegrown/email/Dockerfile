FROM rust:1.88.0
COPY ./ ./ 
RUN cargo build --release 
CMD ["./target/release/email"]
