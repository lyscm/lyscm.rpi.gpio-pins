USERNAME=lyscm

export CR_PAT=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
echo $CR_PAT | docker login ghcr.io -u $USERNAME --password-stdin

CONTAINER_NAME=light-emitting-diode
#TAG=ghcr.io/$USERNAME/$CONTAINER_NAME:arm32v7

#docker buildx build --platform linux/arm/v7 -t $TAG .. --push


# CONTAINER_NAME=light-emitting-diode
# IMAGE_NAME=ghcr.io/lyscm/$CONTAINER_NAME:arm32v7
#
# docker stop $CONTAINER_NAME
# docker rm $CONTAINER_NAME
# 
# docker pull $IMAGE_NAME
#
# docker run \
#    -d \
#    -p 8080:8000 \
#    --device /dev/gpiomem \
#    --name $CONTAINER_NAME \
#    --restart=unless-stopped \
#    --privileged \
#    $IMAGE_NAME