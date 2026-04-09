Describe "user/service encryption works" {

    # The Supervisors for this test have started up with `--org=e2e`,
    # but because of how we check for the service being up via the
    # HTTP gateway, we have to pass the name of the organization name
    # here. It is not actually an argument to `bio svc load`.
    Load-SupervisorService "core/redis" -Remote "alpha.biome.dev" -Org "e2e"
    Load-SupervisorService "core/redis" -Remote "beta.biome.dev" -Org "e2e"
    Load-SupervisorService "core/redis" -Remote "gamma.biome.dev" -Org "e2e"

    Context "uploading without using user encryption" {
        $message = "Hello from Biome!"
        Set-Content message.txt -Value $message
        bio file upload `
            redis.default@e2e `
        ([DateTime]::Now.Ticks) `
            message.txt `
            --remote-sup=bastion.biome.dev
        Start-Sleep 5

        # This is somewhat surprising, as it implies that running a
        # Supervisor with an organization is *not* a signal that
        # config and file uploads *must* be encrypted when sent into
        # the network. Encryption is apparently optional.

        It "you actually can get the file on alpha" {
            $uploadedMessage = docker exec "${env:COMPOSE_PROJECT_NAME}_alpha_1" cat /bio/svc/redis/files/message.txt
            $uploadedMessage | Should -Be $message
        }

        It "you actually can get the file on beta" {
            $uploadedMessage = docker exec "${env:COMPOSE_PROJECT_NAME}_beta_1" cat /bio/svc/redis/files/message.txt
            $uploadedMessage | Should -Be $message
        }

        It "you actually can get the file on gamma" {
            $uploadedMessage = docker exec "${env:COMPOSE_PROJECT_NAME}_gamma_1" cat /bio/svc/redis/files/message.txt
            $uploadedMessage | Should -Be $message
        }

    }

    Context "uploading with user encryption" {
        $new_message = "Hello from an ENCRYPTED Biome!"
        Set-Content encrypted_message.txt -Value $new_message
        bio file upload `
            redis.default@e2e `
        ([DateTime]::Now.Ticks) `
            encrypted_message.txt `
            --remote-sup=bastion.biome.dev `
            --user=mctesterson
        Start-Sleep 5

        # Note that files are only encrypted in transit, and not on disk.

        It "should upload the file to alpha" {
            $uploadedMessage = docker exec "${env:COMPOSE_PROJECT_NAME}_alpha_1" cat /bio/svc/redis/files/encrypted_message.txt
            $uploadedMessage | Should -Be $new_message
        }

        It "should upload the file to beta" {
            $uploadedMessage = docker exec "${env:COMPOSE_PROJECT_NAME}_beta_1" cat /bio/svc/redis/files/encrypted_message.txt
            $uploadedMessage | Should -Be $new_message
        }

        # This does make sense, because the supervisor does not have
        # access to the key material needed to decrypt the encrypted
        # message.
        It "should NOT upload the file to gamma, because it's missing the user key" {
            $uploadedMessage = docker exec "${env:COMPOSE_PROJECT_NAME}_gamma_1" cat /bio/svc/redis/files/encrypted_message.txt
            $uploadedMessage | Should -Be $null
        }
    }
}
