FROM rust:alpine AS builder

WORKDIR /workspace

COPY . .

RUN cargo install --path .

FROM alpine

WORKDIR /app

COPY --from=builder --chown=app_user:app_user /workspace/target/release/app_name .

RUN adduser -D app_user

USER app_user

CMD [ "./app_name" ]
