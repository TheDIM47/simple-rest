FROM rust:1.83-slim as build
WORKDIR /build
COPY . /build/
ADD . .

RUN cargo build --release
RUN echo "done!"

ENV APP_OPTS=""

FROM debian:bookworm-slim
WORKDIR /opt/monolith-mock
COPY --from=build /build/target/release/ .
COPY --from=build /build/resources/ ./resources/
CMD /opt/monolith-mock/simple-rest ${APP_OPTS}
