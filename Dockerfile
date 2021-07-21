ARG REPOSITORY_NAME="lyscm/lyscm.rpi.gpio-pins"
ARG APPLICATION_NAME="lyscm_rpi_gpio-pins"

####################################################################################################
## Builder
####################################################################################################
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

####################################################################################################
## Runtime
####################################################################################################
FROM debian:buster-slim as runtime

# Arguments
ARG ACTIX_PORT
ARG ACTIX_HOST
ARG REPOSITORY_NAME
ARG APPLICATION_NAME

# Environment variables
ENV RUST_BACKTRACE=1
ENV ACTIX_PORT=${ACTIX_PORT}
ENV ACTIX_HOST=${ACTIX_HOST}
ENV REPOSITORY_NAME=${REPOSITORY_NAME}
ENV APPLICATION_NAME=${APPLICATION_NAME}
ENV RUST_LOG=${APPLICATION_NAME}=info,actix=info

# Set ports
EXPOSE ${ACTIX_PORT}

# Import from builder
WORKDIR /opt/${APPLICATION_NAME}
COPY --from=builder /tmp/${APPLICATION_NAME} .

# Run binary
ENTRYPOINT [ "/bin/bash" ]
CMD [ "-c", "$(echo ./$APPLICATION_NAME)" ]
#

# Labels
LABEL org.opencontainers.image.source https://github.com/${REPOSITORY_NAME}
