Describe "bio file upload" {
    bio pkg install core/redis
    Load-SupervisorService "core/redis" -Remote "alpha.biome.dev"
    Load-SupervisorService "core/redis" -Remote "beta.biome.dev"

    $message = "Hello from Biome!"
    Set-Content message.txt -Value $message
    bio file upload `
        redis.default `
    ([DateTime]::Now.Ticks) `
        message.txt `
        --remote-sup=bastion.biome.dev
    Start-Sleep 5

    @("alpha", "beta") | ForEach-Object {
        It "should upload the file to $_" {
            $uploadedMessage = docker exec "${env:COMPOSE_PROJECT_NAME}_${_}_1" cat /bio/svc/redis/files/message.txt
            $uploadedMessage | Should -Be $message
        }
    }
}
