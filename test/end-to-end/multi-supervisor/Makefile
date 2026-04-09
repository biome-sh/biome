CHANNEL ?= stable
IMAGE_NAME ?= supervisor-testing-${CHANNEL}

biome_integration_base: Dockerfile
	@echo "Building biome_integration_base image..."
	@echo "${BIO_AUTH_TOKEN}" > bio_auth_token.txt
	@DOCKER_BUILDKIT=1 docker build \
	  --no-cache \
	  --build-arg CHANNEL=${CHANNEL} \
	  --secret id=bio_auth_token,src=bio_auth_token.txt \
	  -t biome_integration_base .
	@rm bio_auth_token.txt

supervisor_image: supervisor/Dockerfile
	$(MAKE) -C supervisor \
	  CHANNEL=${CHANNEL} \
	  IMAGE_NAME=${IMAGE_NAME} \
	  BIO_AUTH_TOKEN=${BIO_AUTH_TOKEN}

run: supervisor_image
	SUPERVISOR_IMAGE=${IMAGE_NAME} ./run_test_case.sh redis
