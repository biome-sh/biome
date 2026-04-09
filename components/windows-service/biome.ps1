function Install-BioService {
    if($null -ne (Get-Service Biome -ErrorAction SilentlyContinue)) {
        Write-Error "The Biome service is already installed. Please run 'bio pkg exec biome/windows-service uninstall' first if you wish to reinstall."
        return
    }

    if(!(Test-Path (Join-Path $env:SystemDrive "bio\pkgs\biome\bio-sup"))) {
        $bioProc = Get-Process bio -ErrorAction SilentlyContinue
        if(!$bioProc) {
            Write-Error "Could not locate the Biome CLI. Make sure you are running this via 'bio pkg exec biome/windows-service install'."
            return
        }
        $bioExe = $bioProc[0].Path
        & $bioExe pkg install biome/bio-sup
    }

    $svcPath = Join-Path $env:SystemDrive "bio\svc\windows-service"
    if(!(Test-Path $svcPath)) {
        mkdir $svcPath
    }

    Copy-Item "$PSScriptRoot\*" $svcPath -Force

    &$env:systemroot\system32\sc.exe create Biome binpath= "$svcPath\BioService.exe" start= auto
    if($LASTEXITCODE -ne 0) {
        Write-Error "Failed to install the Biome Service!"
    } else {
        &$env:systemroot\system32\sc.exe description Biome "The Biome Supervisor service"
        Write-Host "Congratulations! The Biome Service has succesfully been installed!"
    }
}

function Uninstall-BioService {
    if($null -eq (Get-Service Biome -ErrorAction SilentlyContinue)) {
        Write-Error "The Biome service is not installed."
        return
    }

    Stop-Service Biome
    while((Get-Service Biome).Status -ne "Stopped") {
        Start-Sleep -Seconds 1
    }

    &$env:systemroot\system32\sc.exe delete Biome

    if($LASTEXITCODE -ne 0) {
        Write-Error "Failed to uninstall the Biome Service!"
    } else {
        Write-Host "The Biome Service has succesfully been uninstalled!"
    }
}
