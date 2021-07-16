FROM debian:buster-slim as base
ARG TARGETPLATFORM
ARG REPOSITORY_NAME="lyscm.rpi.gpio-pins"
ARG APPLICATIONNAME="light-emitting-diode"
ARG OWNER="lyscm"

LABEL org.opencontainers.image.source https://github.com/${OWNER}/${REPOSITORY_NAME}

WORKDIR /$OWNER/$TARGETPLATFORM

### BUILD ###
FROM --platform=$BUILDPLATFORM rust as builder
ARG TARGETPLATFORM
ARG OWNER="lyscm"
ARG APPLICATIONNAME="light-emitting-diode"

# Set arguments.
WORKDIR /$OWNER/$TARGETPLATFORM
ARG TARGETPLATFORMPATH=/.buildtargetplatform

RUN case "$TARGETPLATFORM" in \
    "linux/arm/v7") echo armv7-unknown-linux-gnueabihf > $TARGETPLATFORMPATH ;; \
    "linux/arm/v6") echo arm-unknown-linux-gnueabihf > $TARGETPLATFORMPATH ;; \
    *) exit 1 ;; \
    esac

# Compile application.
RUN rustup target add $(cat $TARGETPLATFORMPATH)
RUN apt-get update && apt install -y gcc-arm-linux-gnueabihf

COPY ./src/ ./src/
COPY Cargo.toml ./
COPY ./.cargo ./.cargo/

RUN cargo build --release --target $(cat $TARGETPLATFORMPATH)

RUN cp ./target/$(cat $TARGETPLATFORMPATH)/release/$APPLICATIONNAME .

### RUNTIME ###
FROM base as runtime
COPY --from=builder /$OWNER/$TARGETPLATFORM/$APPLICATIONNAME .
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD [ "./light-emitting-diode" ]