ARG BUILD=production
FROM rust:1.58.0-bullseye as base

ENV PORT=80
ENV HOST=0.0.0.0

WORKDIR /app

RUN rustup toolchain install nightly && \
    rustup default nightly

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

FROM base as production-build
RUN cargo build --release

FROM base as development-build
RUN cargo install cargo-watch
RUN cargo build

FROM ${BUILD}-build as build

EXPOSE ${PORT}

HEALTHCHECK --interval=5s --timeout=3s --retries=3 \
CMD curl -f http://${HOST}:${PORT}/api/ping || exit 1

CMD ["cargo", "run"]
