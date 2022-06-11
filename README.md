# **RAPSBERRY PI GPIO PINS - GRPC SERVER** <h1> 

[![build](https://github.com/lyscm/lyscm.rpi.gpio-pins/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/lyscm/lyscm.rpi.gpio-pins/actions/workflows/ci.yml)

---

## **Initiate package(s)** <h2> 

---

**Set parameters:**

> Bash:

```bash
CONTAINER_NAME=gpio-server
TAG=ghcr.io/lyscm/lyscm.rpi.gpio-pins
```

> Powershell:

```powershell
$CONTAINER_NAME="gpio-server"
$TAG="ghcr.io/lyscm/lyscm.rpi.gpio-pins"
```

**Remove any existing container:**

> Bash | Powershell

```bash
docker stop $CONTAINER_NAME
docker rm $CONTAINER_NAME
docker pull $TAG
```

**Run container:**

> Bash

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

> Powershell
```powershell
docker run `
    -d `
    -p 8080:8000 `
    --device /dev/gpiomem `
    --name $CONTAINER_NAME `
    --restart unless-stopped `
    --privileged `
    $TAG
```