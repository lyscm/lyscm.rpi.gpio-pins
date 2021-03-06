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
RUN wget -qO /etc/apt/trusted.gpg.d/kitware-key.asc https://apt.kitware.com/keys/kitware-archive-latest.asc \
    && echo "deb https://apt.kitware.com/ubuntu/ focal main" | tee /etc/apt/sources.list.d/kitware.list
RUN apt-get update && apt-get install -y cmake gcc-arm-linux-gnueabihf


COPY ./src/ ./src/
COPY Cargo.toml build.rs ./
COPY ./.cargo ./.cargo/
COPY ./.protos ./.protos/

RUN cargo build --release --target $(cat ${TARGETPLATFORM_PATH})

RUN cp ./target/$(cat ${TARGETPLATFORM_PATH})/release/${APPLICATION_NAME} .

####################################################################################################
## Runtime
####################################################################################################
FROM debian:buster-slim as runtime

# Arguments
ARG GRPC_PORT
ARG GRPC_HOST
ARG REPOSITORY_NAME
ARG APPLICATION_NAME

# Environment variables
ENV RUST_BACKTRACE=1
ENV GRPC_PORT=${GRPC_PORT}
ENV GRPC_HOST=${GRPC_HOST}
ENV REPOSITORY_NAME=${REPOSITORY_NAME}
ENV APPLICATION_NAME=${APPLICATION_NAME}
ENV RUST_LOG=${APPLICATION_NAME}=info,actix=info

# Set ports
EXPOSE ${GRPC_PORT}

# Import from builder
WORKDIR /opt/${APPLICATION_NAME}
COPY --from=builder /tmp/${APPLICATION_NAME} .

# Run binary
ENTRYPOINT [ "/bin/bash" ]
CMD [ "-c", "$(echo ./$APPLICATION_NAME)" ]

# Labels
LABEL org.opencontainers.image.source https://github.com/${REPOSITORY_NAME}
