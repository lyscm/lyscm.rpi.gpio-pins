### BUILD ###
FROM --platform=$BUILDPLATFORM rust as builder

# Set arguments.
ARG TARGETPLATFORM
ARG APPLICATIONNAME="light-emitting-diode"
ARG TARGETPLATFORMPATH=/.buildtargetplatform

WORKDIR /lyscm/$TARGETPLATFORM

RUN case "$TARGETPLATFORM" in \
    "linux/arm/v7") echo armv7-unknown-linux-gnueabihf > $TARGETPLATFORMPATH ;; \
    "linux/arm/v6") echo arm-unknown-linux-gnueabihf > $TARGETPLATFORMPATH ;; \
    *) exit 1 ;; \
    esac

# Compile application.
RUN rustup target add $(cat $TARGETPLATFORMPATH)
RUN apt-get update && apt install -y gcc-arm-linux-gnueabihf

COPY ./src ./src/
COPY Cargo.lock Cargo.toml ./
COPY ./.cargo ./.cargo/

RUN cargo build --release --target $(cat $TARGETPLATFORMPATH)

RUN cp ./target/$(cat $TARGETPLATFORMPATH)/release/$APPLICATIONNAME .

### RUNTIME ###
FROM debian:buster-slim as runtime

ARG TARGETPLATFORM
ARG APPLICATIONNAME="light-emitting-diode"

WORKDIR /lyscm/$TARGETPLATFORM

COPY --from=builder /lyscm/$TARGETPLATFORM/$APPLICATIONNAME .

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD [ "./light-emitting-diode" ]