# RAPSBERRY PI - REPOSITORY <h1> 

[![light-emitting-diode - CI](https://github.com/lyscm/lyscm.rpi.gpio-pins/actions/workflows/light-emitting-diode.yml/badge.svg?branch=master)](https://github.com/lyscm/lyscm.rpi.gpio-pins/actions/workflows/light-emitting-diode.yml)

## Initiate package(s): <h2> 

Set parameters:

```bash
OWNER=lyscm
CONTAINER_NAME=lyscm.rpi-led-api
TAG=ghcr.io/lyscm/lyscm.rpi.gpio-pins/light-emitting-diode
```
Remove any existing container:

```bash
docker stop $CONTAINER_NAME
docker rm $CONTAINER_NAME
docker pull $TAG
```

Run container:

```bash
docker run \
    -d \
    -p 8080:8000 \
    --device /dev/gpiomem \
    --name $CONTAINER_NAME \
    --restart unless-stopped \
    --privileged \
    $TAG
```