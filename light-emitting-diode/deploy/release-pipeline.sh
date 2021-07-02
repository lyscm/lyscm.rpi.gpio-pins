USERNAME=lyscm

export CR_PAT=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
echo $CR_PAT | docker login ghcr.io -u $USERNAME --password-stdin

CONTAINER_NAME=light-emitting-diode
TAG=ghcr.io/$USERNAME/$CONTAINER_NAME:arm32v7

docker buildx build --platform linux/arm/v7 -t $TAG .. --push