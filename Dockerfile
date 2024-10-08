FROM rust:1.81.0-slim-bullseye AS build

# View app name in Cargo.toml
ARG APP_NAME=notify

WORKDIR /build

COPY Cargo.lock Cargo.toml ./
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo build --release

COPY src src
RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/server

FROM debian:bullseye-slim AS final
COPY --from=build /bin/server /bin/
COPY .env .
EXPOSE 8080
CMD ["/bin/server"]