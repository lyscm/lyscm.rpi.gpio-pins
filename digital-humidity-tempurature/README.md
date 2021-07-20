# **D**igital **H**umidity **T**empurature <h1> 

[![Digital Humidity Tempurature](https://github.com/lyscm/lyscm.rpi.gpio-pins/actions/workflows/digital-humidity-tempurature-CI.yml/badge.svg)](https://github.com/lyscm/lyscm.rpi.gpio-pins/actions/workflows/digital-humidity-tempurature-CI.yml)

#### STILL IN WORKING PROCESS!

# <h1> 
## Initiate package(s): <h2> 

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
    -p 8081:8000 \
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
