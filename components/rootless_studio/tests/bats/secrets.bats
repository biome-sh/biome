#!/usr/bin/env bats

# TODO (CM): Better parameterize the BIO_DOCKER_STUDIO_IMAGE
# environment variable.
#
# As it is, you can just execute these tests with that in the
# environment, but it would be nice to make it explicit.
#
# BIO_DOCKER_STUDIO_IMAGE=my_image bats secrets.bats

@test "bio pkg build (rootless Docker image): secrets are passed" {
    export BIO_STUDIO_SECRET_FOO=bar
    run bio pkg build -D fixtures/plans/test_build_with_secrets
    [ "$status" -eq 0 ]
}

@test "bio studio run (rootless Docker image): secrets are passed" {
    export BIO_STUDIO_SECRET_FOO=bar
    run bio studio run -D 'echo The result is $FOO'
    [ "$status" -eq 0 ]
    [[ "$output" =~ "The result is bar" ]]
}
