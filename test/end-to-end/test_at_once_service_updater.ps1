# Test the at-once service update strategy
# The timing of this test assumes the following environment variables are set:
# BIO_UPDATE_STRATEGY_PERIOD_MS=3000
# BIO_UPDATE_STRATEGY_PERIOD_BYPASS_CHECK=1

$supLog = New-SupervisorLogFile("test_at_once_service_updater")
Start-Supervisor -LogFile $supLog -Timeout 45 -SupArgs @( `
        "--keep-latest-packages=1"
) | Out-Null
$testChannel="at-once-$([DateTime]::Now.Ticks)"
$pkg="biome-testing/nginx"
$initialRelease="biome-testing/nginx/1.17.4/20191115184838"
$updatedRelease="biome-testing/nginx/1.17.4/20191115185517"

Describe "at-once update and rollback" {
    bio pkg promote $initialRelease $testChannel
    Load-SupervisorService $pkg -Strategy "at-once" -UpdateCondition "track-channel" -Channel $testChannel

    It "loads initial release" {
        Wait-Release -Ident $initialRelease
    }

    Context "promote update" {
        bio pkg promote $updatedRelease $testChannel

        It "updates release" {
            Wait-CommandLinesOfOutput "bio pkg list $pkg" 2
            Wait-Release -Ident $updatedRelease
            # The first package should eventually be automatically uninstalled
            Wait-CommandLinesOfOutput "bio pkg list $pkg" 1
            bio pkg list $pkg | Should -Be $updatedRelease
        }
    }

    Context "demote update" {
        bio pkg demote $updatedRelease $testChannel

        It "rollback release" {
            Wait-Release -Ident $initialRelease
        }
    }

    AfterAll {
        bio bldr channel destroy $testChannel --origin biome-testing
        Unload-SupervisorService -PackageName $pkg -Timeout 20
        Stop-Supervisor
    }
}
