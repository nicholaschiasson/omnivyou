FROM rust:1.55-alpine

RUN cargo install --locked trunk

ENTRYPOINT [ "trunk", "build", "--release" ]
