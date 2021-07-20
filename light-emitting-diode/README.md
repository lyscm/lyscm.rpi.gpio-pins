# **L**ight **E**mitting **D**iode <h1> 

[![Light Emitting Diode](https://github.com/lyscm/lyscm.rpi.gpio-pins/actions/workflows/light-emitting-diode-CI.yml/badge.svg)](https://github.com/lyscm/lyscm.rpi.gpio-pins/actions/workflows/light-emitting-diode-CI.yml)

#### STILL IN WORKING PROCESS!

#  <h1> 
## Initiate package(s) <h2> 

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
$body = @{pin='23';command_type='blink';duration='1000'}

Invoke-WebRequest -Method POST `
    -Body $body `
    -Uri 'http://127.0.0.1:8080/v1.0/gpio/led/transit' 
```
Bash:
```bash
curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"pin": "23", "command_type": "blink", "duration": "1000"}' \
    http://127.0.0.1:8080/v1.0/gpio/led/transit
```