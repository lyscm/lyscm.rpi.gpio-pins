# **L**ight **E**mitting **D**iode <h1> 

[![Light Emitting Diode](https://github.com/lyscm/lyscm.rpi.gpio-pins/actions/workflows/gpio-management-CI.yml/badge.svg?branch=master)](https://github.com/lyscm/lyscm.rpi.gpio-pins/actions/workflows/gpio-management-CI.yml)

#### STILL IN WORKING PROCESS!

#  <h1> 
## Initiate package(s) <h2> 

Set parameters:

```bash
OWNER=lyscm
CONTAINER_NAME=lyscm.rpi-management-api
TAG=ghcr.io/lyscm/lyscm.rpi.gpio-pins/gpio-management
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
$body = @{}
Invoke-WebRequest -Uri 'http://127.0.0.1:8080/v1.0/gpio/management/status' -Method POST -Body $body
```
Bash:
```bash
curl --location --request POST 'http://127.0.0.1:8080/v1.0/gpio/management/status'
```