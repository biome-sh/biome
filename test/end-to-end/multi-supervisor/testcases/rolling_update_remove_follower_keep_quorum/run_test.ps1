# This tests that removing a follower from a functioning leader topology
# service group will continue to perform a succesful roaming update
# We will load services on three nodes and then stop the supervisor on one
# of the follower nodes. Next we perform an update and expect the remaining
# two nodes to update. Prior to https://github.com/biome-sh/biome/pull/7167
# a rolling update after a member death would cause the leader to wait for dead
# members to update themselves which of course will never happen. So we
# perform another update which should succeed if the leader is ignoring dead
# members as it should.

$arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()
switch ($arch) {
    'X64' {
        $script:release1="biome-testing/nginx/1.17.4/20191115184838"
        $script:release2="biome-testing/nginx/1.17.4/20191115185517"
        $script:release3="biome-testing/nginx/1.17.4/20191115185900"
    }
    'Arm64' {
        $script:release1="biome-testing/nginx/1.25.4/20250731123138"
        $script:release2="biome-testing/nginx/1.25.4/20250731123657"
        $script:release3="biome-testing/nginx/1.25.4/20250731123956"
    }
    Default {
        throw "Unsupported architecture: $arch"
    }
}

$script:testChannel = "rolling-$([DateTime]::Now.Ticks)"

Describe "Rolling Update after a follower is removed and quorum is not lost" {
    bio pkg promote $release1 $testChannel
    Load-SupervisorService "biome-testing/nginx" -Remote "alpha.biome.dev" -Topology leader -Strategy rolling -Channel $testChannel
    Load-SupervisorService "biome-testing/nginx" -Remote "beta.biome.dev" -Topology leader -Strategy rolling -Channel $testChannel
    Load-SupervisorService "biome-testing/nginx" -Remote "gamma.biome.dev" -Topology leader -Strategy rolling -Channel $testChannel

    It "loads initial release on alpha" {
        Wait-Release -Ident $release1 -Remote "alpha"
    }
    It "loads initial release on beta" {
        Wait-Release -Ident $release1 -Remote "beta"
    }
    It "loads initial release on gamma" {
        Wait-Release -Ident $release1 -Remote "gamma"
    }

    Context "Remove first follower" {
        BeforeAll {
            $all = 'alpha','beta','gamma'
            $leader = Get-Leader "bastion" "nginx.default"
            $follower = ($all | Where-Object { $_ -ne $leader.Name })[0]
            $survivors = $all | Where-Object { $_ -ne $follower }
            $script:survivor1 = $survivors[0]
            $script:survivor2 = $survivors[1]

            Stop-ComposeSupervisor $follower
            bio pkg promote $release2 $testChannel
        }

        # we expect everyone to be updated now but prior to
        # https://github.com/biome-sh/biome/pull/7167 the leader will
        # indefinitely wait for the dead followers to update
        It "updates to $release2 on $survivor1" {
            Wait-Release -Ident $release2 -Remote $survivor1
        }
        It "updates to $release2 on $survivor2" {
            Wait-Release -Ident $release2 -Remote $survivor2
        }

        Context "update again" {
            # if the leader is not stuck waiting for dead members for the previous update,
            # this update should succeed
            BeforeAll {
                bio pkg promote $release3 $testChannel
            }

            It "updates to $release3 on $survivor1" {
                Wait-Release -Ident $release3 -Remote $survivor1
            }
            It "updates to $release3 on $survivor2" {
                Wait-Release -Ident $release3 -Remote $survivor2
            }
        }
    }

    AfterAll {
        bio bldr channel destroy $testChannel --origin biome-testing
    }
}
