. $PSScriptRoot\..\..\..\.expeditor\scripts\verify\shared.ps1
$env:BIO_LICENSE = "accept-no-persist"
Install-Biome

Install-BioPkg -idents @("core/pester")
Import-Module "$(bio pkg path core/pester)\module\pester.psd1"

Invoke-Pester $PSScriptRoot -EnableExit
