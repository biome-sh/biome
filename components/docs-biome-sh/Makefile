# we use pushd/popd here, and /bin/sh of our biomees/buildkite image is not bash
# so we have to override the default shell here
SHELL=bash

preview_netlify: biome_web_docs
	rm -rf biome-web-docs/_vendor/github.com/biome-sh/biome/components/docs-biome-sh/*
	cp -R content biome-web-docs/_vendor/github.com/biome-sh/biome/components/docs-biome-sh
	cp -R static biome-web-docs/_vendor/github.com/biome-sh/biome/components/docs-biome-sh
	cp -R config.toml biome-web-docs/_vendor/github.com/biome-sh/biome/components/docs-biome-sh
	pushd biome-web-docs && bash ./scripts/netlify-deploy-preview.sh && popd

serve: biome_web_docs
	printf "go 1.22\n\nuse .\nuse ../" > biome-web-docs/hugo.work
	pushd biome-web-docs && make bundle && HUGO_MODULE_WORKSPACE=hugo.work hugo server --buildDrafts --buildFuture --noHTTPCache --ignoreVendorPaths "github.com/biome-sh/biome/components/docs-biome-sh" && popd

biome_web_docs:
	if [ -d "biome-web-docs/" ]; then \
		pushd biome-web-docs && git reset HEAD --hard; git clean -fd; git pull --ff-only origin main; rm -rf public && popd;  \
	else \
		git clone https://github.com/biome/biome-web-docs.git; \
	fi

clean_all:
	rm -rf biome-web-docs
	hugo mod clean

lint: assets
	hugo -D
