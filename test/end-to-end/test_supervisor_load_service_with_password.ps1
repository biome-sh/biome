$username = "bio_test"
$password = "Pass@word1"
net user /add $username $password
net localgroup administrators $username /add
Add-Type -TypeDefinition (Get-Content ".expeditor\scripts\end_to_end\LsaWrapper.cs" | Out-String)
$lsa_wrapper = New-Object -type LsaWrapper
$lsa_wrapper.SetRight($username, "SeServiceLogonRight")

$env:BIO_ORIGIN = "ci"
bio origin key generate ci
bio pkg build test/fixtures/windows_plans/dummy_svc_user
. .\results\last_build.ps1
bio pkg install .\results\$pkg_artifact

bio pkg install core/windows-service
Start-Service Biome
Wait-Supervisor -Timeout 45

Describe "bio svc load" {
    $loadOut = bio svc load ci/dummy --password $password
    Wait-SupervisorService dummy -Timeout 20

    It "Succesfully loads service" {
        $loadOut | Should -Be "The ci/dummy service was successfully loaded"
    }
    It "Reports service on HTTP Gateway as UP" {
        ((Invoke-WebRequest "http://localhost:9631/services/dummy/default" -UseBasicParsing).content | ConvertFrom-Json).process.state | Should -Be "up"
    }
    It "runs hook as svc_user" {
        $proc = Get-Process ping -IncludeUserName
        $proc.UserName | Should -Be "$env:ComputerName\$username"
    }
}
