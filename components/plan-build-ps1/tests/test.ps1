. $PSScriptRoot\..\..\..\.expeditor\scripts\verify\shared.ps1
$env:BIO_LICENSE = "accept-no-persist"
Install-Biome

bio pkg install core/pester
Import-Module "$(bio pkg path core/pester)\module\pester.psd1"

$test_result = Invoke-Pester -PassThru
exit $test_result.FailedCount
