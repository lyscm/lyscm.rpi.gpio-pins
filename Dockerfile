ARG REPOSITORY_NAME="lyscm/lyscm.rpi.gpio-pins"
ARG APPLICATION_NAME="lyscm_rpi_gpio-pins"

FROM alpine:latest as base

ARG TARGETPLATFORM
ARG REPOSITORY_NAME
ARG APPLICATION_NAME

ENV TARGETPLATFORM=${TARGETPLATFORM}
ENV REPOSITORY_NAME=${REPOSITORY_NAME}
#ENV TZ=Etc/UTC
#
#RUN apk update \
#    && apk add --no-cache ca-certificates tzdata \
#    && rm -rf /var/cache/apk/*

LABEL org.opencontainers.image.source https://github.com/${REPOSITORY_NAME}

### BUILD ###
FROM --platform=$BUILDPLATFORM rust as builder

# Set arguments.
ARG TARGETPLATFORM
ARG APPLICATION_NAME
ARG TARGETPLATFORM_PATH=/.buildtargetplatform

WORKDIR /tmp

RUN case "${TARGETPLATFORM}" in \
    "linux/arm/v7") echo "armv7-unknown-linux-gnueabihf" > ${TARGETPLATFORM_PATH} ;; \
    "linux/arm/v6") echo "arm-unknown-linux-gnueabihf" > ${TARGETPLATFORM_PATH} ;; \
    *) exit 1 ;; \
    esac

# Compile application.
RUN rustup target add $(cat ${TARGETPLATFORM_PATH})
RUN apt-get update && apt install -y gcc-arm-linux-gnueabihf

COPY ./src/ ./src/
COPY Cargo.toml ./
COPY ./.cargo ./.cargo/

RUN cargo build --release --target $(cat ${TARGETPLATFORM_PATH})

RUN cp ./target/$(cat ${TARGETPLATFORM_PATH})/release/${APPLICATION_NAME} .

### RUNTIME ###
FROM base as runtime
ARG ACTIX_PORT
ARG ACTIX_HOST
ENV ACTIX_PORT=${ACTIX_PORT}
ENV ACTIX_HOST=${ACTIX_HOST}
ENV RUST_LOG=${APPLICATION_NAME}=info,actix=info

EXPOSE ${ACTIX_PORT}

WORKDIR /opt/${APPLICATION_NAME}

COPY --from=builder /tmp/${APPLICATION_NAME} .

RUN chmod 777 "${APPLICATION_NAME}"
RUN mv "${APPLICATION_NAME}" .initiate 

CMD ["./.initiate"]