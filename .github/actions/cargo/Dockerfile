FROM rust:1.55-alpine

RUN rustup target add wasm32-unknown-unknown

WORKDIR ${GITHUB_WORKSPACE}

ENTRYPOINT [ "cargo" ]
