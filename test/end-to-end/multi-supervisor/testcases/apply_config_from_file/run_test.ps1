Describe "apply config from file" {
    bio pkg install core/redis
    Load-SupervisorService "core/redis" -Remote "alpha.biome.dev"
    Load-SupervisorService "core/redis" -Remote "beta.biome.dev"

    $new_port=2112
    Set-Content redis_config.toml -Value "port = $new_port`nprotected-mode = `"no`""
    bio config apply `
        redis.default `
    ([DateTime]::Now.Ticks) `
        redis_config.toml `
        --remote-sup=bastion.biome.dev
    Start-Sleep 5

    @("alpha", "beta") | ForEach-Object {
        It "should call redis cli SET on applied port on $_" {
            bio pkg exec core/redis redis-cli -h "$_.biome.dev" -p $new_port SET from_stdin_port $new_port
            $LASTEXITCODE | Should -Be 0
        }
    }

    @("alpha", "beta") | ForEach-Object {
        It "should call redis cli GET on applied port on $_" {
            bio pkg exec core/redis redis-cli -h "$_.biome.dev" -p $new_port GET from_stdin_port
            $LASTEXITCODE | Should -Be 0
        }
    }
}
