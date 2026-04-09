CHANNEL ?= stable
IMAGE_NAME ?= supervisor

image: Dockerfile
	@echo "Building supervisor image..."
	@echo "${BIO_AUTH_TOKEN}" > bio_auth_token.txt
	@DOCKER_BUILDKIT=1 docker build \
	  --build-arg CHANNEL=${CHANNEL} \
	  --secret id=bio_auth_token,src=bio_auth_token.txt \
	  -t ${IMAGE_NAME} .
	@rm bio_auth_token.txt
