# RAPSBERRY PI GPIO PINS - REPOSITORY <h1> 

[![Light Emitting Diode](https://github.com/lyscm/lyscm.rpi.gpio-pins/actions/workflows/light-emitting-diode-CI.yml/badge.svg?branch=master)](https://github.com/lyscm/lyscm.rpi.gpio-pins/actions/workflows/light-emitting-diode-CI.yml) 
[![Digital Humidity Tempurature](https://github.com/lyscm/lyscm.rpi.gpio-pins/actions/workflows/digital-humidity-tempurature-CI.yml/badge.svg?branch=master)](https://github.com/lyscm/lyscm.rpi.gpio-pins/actions/workflows/digital-humidity-tempurature-CI.yml)

#### STILL IN WORKING PROCESS!

#  <h1> 
## Initiate package(s) - **L**ight **E**mitting **D**iode: <h2> 

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

## Try out: <h2> 



Powershell:
```powershell
$body = @{status='on';pin='23'}
Invoke-WebRequest -Uri 'http://127.0.0.1:8080/v1.0/gpio/led/switch?pin=23&status=on' -Method POST -Body $body
```
Bash:
```bash
curl --location --request POST 'http://127.0.0.1:8080/v1.0/gpio/led/switch?pin=23&status=on'
```
# <h1> 
## Initiate package(s) - **D**igital **H**umidity **T**empurature: <h2> 

Set parameters:

```bash
OWNER=lyscm
CONTAINER_NAME=lyscm.rpi-hat-api
TAG=ghcr.io/lyscm/lyscm.rpi.gpio-pins/digital-humidity-tempurature
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
$body = @{status='on';pin='23'}
Invoke-WebRequest -Uri 'http://127.0.0.1:8080/v1.0/gpio/hat/switch?pin=23&status=on' -Method POST -Body $body
```
Bash:
```bash
curl --location --request POST 'http://127.0.0.1:8080/v1.0/gpio/hat/switch?pin=23&status=on'
```
