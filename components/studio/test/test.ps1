# This is a lightweight test to verify a studio can be created before merging a PR.
# This (hopefully) prevents us spending time building the first half of a release
# only to hit a broken studio.
#
# Failure case: because this creates a studio from source, we don't exercise changes
# in our plan.sh, and could still end up with a bad studio build.

param(
    [Parameter(Mandatory = $true)]
    [string]$CorePackageChannel
)

$ErrorActionPreference = "Stop"

$env:BIO_LICENSE = "accept-no-persist"

bio pkg install --channel $CorePackageChannel core/powershell
if ($LASTEXITCODE -ne 0) {
    throw "Failed to install core/powershell from the '$CorePackageChannel' channel (exit code $LASTEXITCODE)"
}

bio pkg install --channel $CorePackageChannel core/7zip
if ($LASTEXITCODE -ne 0) {
    throw "Failed to install core/7zip from the '$CorePackageChannel' channel (exit code $LASTEXITCODE)"
}

bio pkg install biome/bio-plan-build-ps1
if ($LASTEXITCODE -ne 0) {
    throw "Failed to install biome/bio-plan-build-ps1 from the '$env:BIO_BLDR_CHANNEL' channel (exit code $LASTEXITCODE)"
}

mkdir "bin/powershell" | Out-Null
mkdir "bin/bio" | Out-Null
mkdir "bin/7zip" | Out-Null

Copy-Item "$(bio pkg path core/powershell)/bin/*" "bin/powershell" -Recurse
Copy-Item "$(bio pkg path biome/bio)/bin/*" "bin/bio"
Copy-Item "$(bio pkg path core/7zip)/bin/*" "bin/7zip"
Copy-Item "$(bio pkg path biome/bio-plan-build-ps1)/bin/*" "bin/"

try {
    & bin/powershell/pwsh.exe -NoProfile -ExecutionPolicy bypass -NoLogo -File "bin/bio-studio.ps1" new
    $exit_code = $LASTEXITCODE
} finally {
    # The test can exit before the Studio has closed all open
    # handles to the following files/directories. This sleep
    # gives those processes a chance to finish.
    Start-Sleep 5
    Remove-Item "bin/7zip" -Recurse
    Remove-Item "bin/powershell" -Recurse
    Remove-Item "bin/bio" -Recurse
    Remove-Item "bin/environment.ps1"
    Remove-Item "bin/shared.ps1"
    Remove-Item "bin/bio-plan-build.ps1"
}
exit $exit_code
