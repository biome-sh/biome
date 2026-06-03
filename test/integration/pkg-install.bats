#!/usr/bin/env bats

load 'helpers'

setup() {
    reset_bio_root
}

@test "bio pkg install: origin/name (standalone service)" {
    run ${bio} pkg install core/redis
    assert_success

    latest_redis=$(latest_from_builder core/redis stable)
    assert_package_and_deps_installed "${latest_redis}"
}

@test "bio pkg install: origin/name/version (standalone service)" {
    run ${bio} pkg install core/redis/3.2.4
    assert_success

    latest_redis=$(latest_from_builder core/redis/3.2.4 stable)
    assert_package_and_deps_installed "${latest_redis}"
}

@test "bio pkg install: origin/name/version/release (standalone service)" {
    desired_version="core/redis/3.2.3/20160920131015"

    run ${bio} pkg install "${desired_version}"
    assert_success
    assert_package_and_deps_installed "${desired_version}"
}

@test "bio pkg install: local bart file (standalone service)" {
    desired_version="core/redis/3.2.4/20170514150022"

    # First, grab a bart file! Then set it aside and clean everything
    # out of /bio. This way, we'll have a bart file, but nothing else,
    # which is exactly what we want.
    run ${bio} pkg install "${desired_version}"
    assert_success
    cp $(cached_artifact_for "${desired_version}") /tmp
    reset_bio_root

    # Now, install from just the local bart file
    run ${bio} pkg install /tmp/core-redis-3.2.4-20170514150022-x86_64-linux.bart
    assert_success
    assert_package_and_deps_installed ${desired_version}
}

@test "bio pkg install: local bart from /bio/cache/artifacts (standalone service)" {
    desired_version="core/redis/3.2.4/20170514150022"

    # First, grab a bart file!
    run ${bio} pkg install "${desired_version}"
    assert_success
    # We don't want to remove everything in /bio, because we want the
    # artifact cache to remain.
    remove_installed_packages
    empty_key_cache

    # Now install from the local bart file *from the cache*
    run ${bio} pkg install "$(cached_artifact_for ${desired_version})"
    assert_success
    assert_package_and_deps_installed "${desired_version}"
}

@test "bio pkg install: installing from a file that isn't a bart fails" {
    echo "lolwut" > /tmp/not-a.bart

    run ${bio} pkg install /tmp/not-a.bart
    assert_failure
    [[ "$output" =~ "Can't read keyname" ]]
}

@test "bio pkg install: installing from a nonexistent file that looks a bart fails" {
    run ${bio} pkg install looks-like/an-ident.bart
    assert_failure
    [[ "$output" =~ "File not found at: looks-like/an-ident.bart" ]]
}
