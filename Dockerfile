FROM rust:1.86.0-bookworm AS build

WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build /app/target/release/proxy-server /app/server
ENV DATABASE-URL=mysql://leon:Dominskuy123!@202.70.133.108:3306/datatest
CMD ["/app/server"]