# RAPSBERRY PI GPIO PINS - REPOSITORY <h1> 

[![build](https://github.com/lyscm/lyscm.rpi.gpio-pins/actions/workflows/lyscm.rpi.gpio-pins-CI.yml/badge.svg?branch=master)](https://github.com/lyscm/lyscm.rpi.gpio-pins/actions/workflows/lyscm.rpi.gpio-pins-CI.yml)

#### STILL IN WORKING PROCESS! <h2> 

## Initiate package(s) <h2> 

#
    
Set parameters:

```bash
CONTAINER_NAME=gpio-pins-api
TAG=ghcr.io/lyscm/lyscm.rpi.gpio-pins
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

## Try out: <h2> 

Powershell:
```powershell
Invoke-WebRequest -Method POST -Uri 'http://localhost:8080/v1.0/gpio/led/transit' `
-ContentType 'application/json' `
-Body '{ 
    "pin": 23, 
    "command_type": "blink", 
    "duration": 500
}'
```
Bash:
```bash
curl --location --request POST 'http://localhost:8080/v1.0/gpio/led/transit' \
--header 'Content-Type: application/json' \
--data-raw '{
    "pin": 23,
    "command_type": "blink",
    "duration": 500
}'
```
