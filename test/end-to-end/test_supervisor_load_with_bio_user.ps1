# This endures that given a 'bio' user, a plan with no pkg_svc_user will
# not produce a service that will be run by the bio user. This used to be
# the case but we are no longer using the bio account by default on windows.
# see https://github.com/habitat-sh/habitat/issues/6847
$username = "bio"
$password = "Pass@word1"
net user /add $username $password
net localgroup administrators $username /add
Add-Type -TypeDefinition (Get-Content ".expeditor\scripts\end_to_end\LsaWrapper.cs" | Out-String)
$lsa_wrapper = New-Object -type LsaWrapper
$lsa_wrapper.SetRight($username, "SeServiceLogonRight")

$env:BIO_ORIGIN = "ci"
bio origin key generate ci

bio pkg install core/windows-service
Start-Service Biome
Wait-Supervisor -Timeout 45

Describe "with no svc_user" {
    bio pkg build test/fixtures/windows_plans/dummy
    . .\results\last_build.ps1
    bio pkg install .\results\$pkg_artifact
    $loadOut = bio svc load ci/dummy
    Wait-SupervisorService dummy -Timeout 20

    It "does not create a SVC_USR metafile" {
        Test-Path c:\bio\pkgs\$pkg_ident\SVC_USER | Should -Be $false
    }
    It "Succesfully loads service" {
        $loadOut | Should -Be "The ci/dummy service was successfully loaded"
    }
    It "Reports service on HTTP Gateway as UP" {
        ((Invoke-WebRequest "http://localhost:9631/services/dummy/default" -UseBasicParsing).content | ConvertFrom-Json).process.state | Should -Be "up"
    }
    It "runs hook as current user" {
        # the dummy run hook simply runs ping continuously
        $proc = Get-Process ping -IncludeUserName
        $proc.UserName | Should -Be "NT AUTHORITY\SYSTEM"
    }
    AfterAll {
        bio svc unload ci/dummy
        Start-Sleep -Seconds 5 # ping needs to be forcefully shutdown
    }
}

Describe "with svc_user" {
    bio pkg build test/fixtures/windows_plans/dummy_bio_svc_user
    . .\results\last_build.ps1
    bio pkg install .\results\$pkg_artifact
    $loadOut = bio svc load ci/dummy-bio-user --password $password
    Wait-SupervisorService dummy-bio-user -Timeout 20

    It "does create a SVC_USR metafile" {
        Test-Path c:\bio\pkgs\$pkg_ident\SVC_USER | Should -Be $true
    }
    It "Succesfully loads service" {
        $loadOut | Should -Be "The ci/dummy-bio-user service was successfully loaded"
    }
    It "Reports service on HTTP Gateway as UP" {
        ((Invoke-WebRequest "http://localhost:9631/services/dummy-bio-user/default" -UseBasicParsing).content | ConvertFrom-Json).process.state | Should -Be "up"
    }
    It "runs hook as current user" {
        # the dummy run hook simply runs ping continuously
        $proc = Get-Process ping -IncludeUserName
        $proc.UserName | Should -Be "$env:computername\bio"
    }
}
