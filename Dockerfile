FROM messense/rust-musl-cross:x86_64-musl as chef
ENV SQLX_OFFLINE=true
WORKDIR /app
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/advent-of-code-friend /advent-of-code-friend
WORKDIR /data
ENTRYPOINT ["/advent-of-code-friend"]
