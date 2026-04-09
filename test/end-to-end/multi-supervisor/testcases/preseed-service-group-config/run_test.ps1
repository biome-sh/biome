# Ensures that we can `bio config apply` some configuration to a
# Biome network before any services are running, and have those
# services pick up the configuration the first time they load.
Describe "preseeded service group config" {
    # Add some non-standard configuration to the network BEFORE we run
    # anything in the targeted service group.
    #
    # Normally, Redis is available at port 6379, but here we're setting it
    # to 8888.
    $new_port=8888
    "port = $new_port`nprotected-mode = `"no`"" | bio config apply `
        redis.default `
    ([DateTime]::Now.Ticks) `
        --remote-sup=bastion.biome.dev
    bio pkg install core/redis
    Load-SupervisorService "core/redis" -Remote "alpha.biome.dev"

    It "should call redis cli SET on applied port" {
        bio pkg exec core/redis redis-cli -h "alpha.biome.dev" -p $new_port SET secret_message "Hello World"
        $LASTEXITCODE | Should -Be 0
    }
}
