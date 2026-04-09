# Rootless Docker Studio

## Building

```
docker build -t biome:bio-base .
docker build --build-arg BLDR_CHANNEL="${channel}" --no-cache -t "${IMAGE_NAME}:${version}" "./default"
```

## TODO:

Bring in the other studio types
