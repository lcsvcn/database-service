FROM rust:latest

# 2. Copy the files in your machine to the Docker image
WORKDIR /app

COPY . .

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/rust-app"]
