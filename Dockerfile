FROM rust

WORKDIR /workspace

COPY . .

# Compile and install 3rd party dependencies
RUN cargo install --path .

ENTRYPOINT [ "./target/release/app_name"]
