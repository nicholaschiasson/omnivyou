FROM rust:1.55-alpine

RUN cargo install --locked trunk

WORKDIR ${GITHUB_WORKSPACE}

ENTRYPOINT [ "trunk", "build", "--release" ]
