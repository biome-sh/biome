#!/usr/bin/env bats

load 'helpers'

setup() {
    reset_bio_root
    start_supervisor
}

teardown() {
    stop_supervisor
}

@test "bio svc load: a bad topology value is rejected" {
    run ${bio} svc load --topology=beelzebub core/redis
    assert_failure
    [[ "${output}" =~ "'beelzebub' isn't a valid value for '--topology <TOPOLOGY>'" ]]
}

@test "bio svc load: a bad strategy value is rejected" {
    run ${bio} svc load --strategy=beelzebub core/redis
    assert_failure
    [[ "${output}" =~ "Invalid value for '--strategy <STRATEGY>'" ]]
}

@test "bio svc load: origin/name (standalone service)" {
    run ${bio} svc load core/redis
    assert_success

    wait_for_service_to_run redis

    latest_redis=$(latest_from_builder core/redis stable)
    assert_package_and_deps_installed "${latest_redis}"

    assert_spec_exists_for redis

    assert_spec_value redis ident core/redis
    assert_spec_value redis group default
    assert_spec_value redis channel stable
    assert_spec_value redis topology standalone
    assert_spec_value redis update_strategy none
    assert_spec_value redis desired_state up
    assert_spec_value redis bldr_url "https://bldr.biome.sh"
}

@test "bio svc load: origin/name/version (standalone service)" {
    run ${bio} svc load core/redis/3.2.4
    assert_success

    wait_for_service_to_run redis

    latest_redis=$(latest_from_builder core/redis/3.2.4 stable)
    assert_package_and_deps_installed "${latest_redis}"
    assert_spec_exists_for redis

    assert_spec_value redis ident core/redis/3.2.4
    assert_spec_value redis group default
    assert_spec_value redis channel stable
    assert_spec_value redis topology standalone
    assert_spec_value redis update_strategy none
    assert_spec_value redis desired_state up
    assert_spec_value redis bldr_url "https://bldr.biome.sh"
}

@test "bio svc load: origin/name/version/release (standalone service)" {
    desired_version="core/redis/3.2.3/20160920131015"
    run ${bio} svc load "${desired_version}"
    assert_success

    wait_for_service_to_run redis

    assert_package_and_deps_installed "${desired_version}"
    assert_spec_exists_for redis

    assert_spec_value redis ident "${desired_version}"
    assert_spec_value redis group default
    assert_spec_value redis channel stable
    assert_spec_value redis topology standalone
    assert_spec_value redis update_strategy none
    assert_spec_value redis desired_state up
    assert_spec_value redis bldr_url "https://bldr.biome.sh"
}

@test "bio svc load: local hart file (standalone service)" {
    # First, grab a hart file!
    desired_version="core/redis/3.2.4/20170514150022"
    hart_path=$(download_hart_for "${desired_version}")

    run ${bio} pkg install "${hart_path}"
    assert_success
    run ${bio} svc load "${desired_version}"
    assert_success

    wait_for_service_to_run redis

    assert_package_and_deps_installed "${desired_version}"
    assert_spec_exists_for redis

    assert_spec_value redis ident "${desired_version}"
    assert_spec_value redis group default
    assert_spec_value redis channel stable
    assert_spec_value redis topology standalone
    assert_spec_value redis update_strategy none
    assert_spec_value redis desired_state up
    assert_spec_value redis bldr_url "https://bldr.biome.sh"
}

@test "bio svc load: prefer local packages over pulling from Builder" {
    desired_version="core/redis/3.2.3/20160920131015"
    # Pre-install this older package. Loading the service should not cause a
    # newer package to be installed.
    run ${bio} pkg install "${desired_version}"

    run ${bio} svc load "core/redis"
    assert_success

    wait_for_service_to_run redis

    assert_package_and_deps_installed "${desired_version}"
    assert_spec_exists_for redis
}

@test "bio svc load: change spec with --force (standalone service)" {
    run ${bio} svc load core/redis
    assert_success

    wait_for_service_to_run redis

    # Assert the default values in the service spec
    assert_spec_value redis ident core/redis
    assert_spec_value redis group default
    assert_spec_value redis channel stable
    assert_spec_value redis topology standalone
    assert_spec_value redis update_strategy none
    assert_spec_value redis desired_state up
    assert_spec_value redis bldr_url "https://bldr.biome.sh"

    # Now, "reload" and change a few settings (chosen here arbitrarily)
    run ${bio} svc load --force --channel=unstable --strategy=at-once core/redis
    assert_success

    # Assert the spec values after the update
    assert_spec_value redis ident core/redis
    assert_spec_value redis group default
    assert_spec_value redis channel unstable # <-- changed!
    assert_spec_value redis topology standalone
    assert_spec_value redis update_strategy at-once # <-- changed!
    assert_spec_value redis desired_state up
    assert_spec_value redis bldr_url "https://bldr.biome.sh"
}

@test "bio svc load: loading an existing service without --force is an error" {
    run ${bio} svc load core/redis
    assert_success

    wait_for_service_to_run redis

    # Assert the contents of the spec file; we'll compare again later
    assert_spec_value redis ident core/redis
    assert_spec_value redis group default
    assert_spec_value redis channel stable
    assert_spec_value redis topology standalone
    assert_spec_value redis update_strategy none
    assert_spec_value redis desired_state up
    assert_spec_value redis bldr_url "https://bldr.biome.sh"

    # Now, try to load again, but without --force
    run ${bio} svc load --channel=unstable --strategy=at-once core/redis
    assert_failure

    # Check that the spec file values didn't change
    assert_spec_value redis ident core/redis
    assert_spec_value redis group default
    assert_spec_value redis channel stable
    assert_spec_value redis topology standalone
    assert_spec_value redis update_strategy none
    assert_spec_value redis desired_state up
    assert_spec_value redis bldr_url "https://bldr.biome.sh"
}

@test "bio svc load: application and environment are properly set in a spec" {
    run ${bio} svc load core/redis
    assert_success

    wait_for_service_to_run redis

    assert_spec_value redis ident core/redis
}

@test "bio svc load: spec idents can change when force-loading using a different ident" {
    vsn="core/redis/3.2.3/20160920131015"

    stop_supervisor

    HAB_UPDATE_STRATEGY_FREQUENCY_MS=5000 background ${bio} run
    retry 5 1 launcher_is_alive

    run ${bio} svc load "${vsn}"
    assert_success
    wait_for_service_to_run redis
    initial_pid=$(pid_of_service redis)

    assert_spec_value redis ident "${vsn}"

    run ${bio} svc load --channel=unstable --strategy=at-once --force core/redis
    assert_success

    # loading causes a restart anyway
    wait_for_service_to_restart redis "${initial_pid}"
    new_pid=$(pid_of_service redis)

    # The ident should have changed (among other things)
    assert_spec_value redis ident core/redis
    assert_spec_value redis channel unstable
    assert_spec_value redis update_strategy at-once

    # Wait for the new version to be installed
    wait_for_service_to_restart redis "${new_pid}"

    latest_redis=$(latest_from_builder core/redis unstable)
    assert_package_and_deps_installed "${latest_redis}"

    updated_running_version=$(current_running_version_for redis)
    assert_equal "${latest_redis}" "$updated_running_version"
    # assert latest redis is installed, though not necessarily
    # *running* (that's for the updater to do)
}
