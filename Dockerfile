# ---------------------------------------------------
# 1 - Build Stage
#
# Use official rust image to for application build
# ---------------------------------------------------
FROM rust:1.72 as build

# Setup working directory
WORKDIR /usr/src/app
COPY . .
COPY .env.docker .env

# Install dependency (Required by diesel)
RUN apt-get update
RUN apt-get install -y mariadb-server 

# Build application
RUN cargo install --path .

# ---------------------------------------------------
# 2 - Deploy Stage
#
# Use a distroless image for minimal container size
# - Copy `libpq` dependencies into the image (Required by diesel)
# - Copy application files into the image
# ---------------------------------------------------
FROM gcr.io/distroless/cc-debian12

# Set the architecture argument (arm64, i.e. aarch64 as default)
# For amd64, i.e. x86_64, you can append a flag when invoking the build `... --build-arg "ARCH=x86_64"`
ARG ARCH=x86_64

# mysql related (required by diesel)
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libmariadb.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libmariadbclient.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libz.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libssl.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libcrypto.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /lib/${ARCH}-linux-gnu/libcom_err.so* /lib/${ARCH}-linux-gnu/
COPY --from=build /lib/${ARCH}-linux-gnu/libkeyutils.so* /lib/${ARCH}-linux-gnu/
COPY --from=build /lib/${ARCH}-linux-gnu/libc.so* /lib/${ARCH}-linux-gnu/

# Application files
COPY --from=build /usr/local/cargo/bin/calendar-backend /usr/local/bin/calendar-backend
COPY --from=build /usr/src/app/.env /.env
COPY --from=build /usr/src/app/assets /assets
COPY --from=build /usr/src/app/migrations /migrations
COPY --from=build /usr/src/app/diesel.toml /diesel.toml

EXPOSE 8081

CMD ["calendar-backend"]