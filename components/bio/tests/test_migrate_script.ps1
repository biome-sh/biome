[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUseCorrectCasing', '')]
param()

Describe "Migrate biome using migrate.ps1" {
    BeforeAll {
        # Install the last stable core/bio package
        Write-Host "Installing core/bio v1.6.1245..."
        Invoke-Expression "& { $(Invoke-RestMethod https://raw.githubusercontent.com/biome-sh/biome/master/components/bio/install.ps1) } -Version 1.6.1245" | Out-Null
        if (-not $?) {
            throw "Failed to install core/bio"
        }

        # Install core/bio-sup
        Write-Host "Installing core/bio-sup from stable channel..."
        bio pkg install core/bio-sup --channel=stable
        if (-not $?) {
            throw "Failed to install core/bio-sup"
        }

        # Install core/windows-service
        Write-Host "Installing core/windows-service from stable channel..."
        bio pkg install core/windows-service --channel=stable
        if (-not $?) {
            throw "Failed to install core/windows-service"
        }

        # Start the service
        Write-Host "Starting Biome Windows service..."
        Start-Service -Name "Biome"
        Start-Sleep -Seconds 10
    }

    It "verifies core packages are installed before migration" {
        # Check for core/bio
        $bioPath = (Get-Command bio).Path
        $bioPath | Should -Exist

        # Verify bio-sup is installed from core origin
        $coreBioSupOutput = bio pkg list core/bio-sup
        $coreBioSupOutput | Should -Match "core/bio-sup"

        # Verify windows-service is installed from core origin
        $coreWindowsServiceOutput = bio pkg list core/windows-service
        $coreWindowsServiceOutput | Should -Match "core/windows-service"

        # Verify the Biome service is running
        $biomeService = Get-Service -Name "Biome" -ErrorAction SilentlyContinue
        $biomeService | Should -Not -BeNullOrEmpty
        $biomeService.Status | Should -Be "Running"

        # Verify bio-sup process is running
        $bioSup = Get-Process -Name "bio-sup" -ErrorAction SilentlyContinue
        $bioSup | Should -Not -BeNullOrEmpty
    }

    It "successfully migrates from core to biome packages" {
        # Store the pre-migration bio-sup version for comparison
        $preMigrationVersion = $null
        $bioSup = Get-Process -Name "bio-sup" -ErrorAction SilentlyContinue
        if ($bioSup) {
            $bioSupPath = $bioSup.Path
            if ($bioSupPath -match 'core\\bio-sup\\([^/\\]+)') {
                $preMigrationVersion = $Matches[1]
                Write-Host "Pre-migration bio-sup version: $preMigrationVersion"
            }
        }

        # Run the migration script with test auth token
        components/bio/migrate.ps1
        $LASTEXITCODE | Should -Be 0

        # Check that bio is still installed
        $bioPath = (Get-Command bio).Path
        $bioPath | Should -Exist

        # Verify bio-sup is now installed from biome origin
        $biomeBioSupOutput = bio pkg list biome/bio-sup
        $biomeBioSupOutput | Should -Match "biome/bio-sup"

        # Verify windows-service is now installed from biome origin
        $biomeWindowsServiceOutput = bio pkg list biome/windows-service
        $biomeWindowsServiceOutput | Should -Match "biome/windows-service"

        # Verify the Biome service is still running
        $biomeService = Get-Service -Name "Biome" -ErrorAction SilentlyContinue
        $biomeService | Should -Not -BeNullOrEmpty
        $biomeService.Status | Should -Be "Running"

        # Verify bio-sup process is still running
        $bioSup = Get-Process -Name "bio-sup" -ErrorAction SilentlyContinue
        $bioSup | Should -Not -BeNullOrEmpty

        # Verify the running bio-sup is from the biome origin
        $bioSupPath = $bioSup.Path
        $bioSupPath | Should -Match "biome\\bio-sup"

        # Verify version is same or newer after migration
        if ($preMigrationVersion) {
            $bioSupPath -match 'biome\\bio-sup\\([^/\\]+)' | Should -Be $true
            $postMigrationVersion = $Matches[1]
            Write-Host "Post-migration bio-sup version: $postMigrationVersion"

            # Convert versions to [version] objects for proper comparison
            $preVersion = [version]($preMigrationVersion -replace '-.*$', '')
            $postVersion = [version]($postMigrationVersion -replace '-.*$', '')

            # Post-migration version should be same or newer
            $postVersion -ge $preVersion | Should -Be $true
        }
    }

    It "does not restart bio-sup when migration is run a second time" {
        # Store the current bio-sup process ID
        $bioSupBefore = Get-Process -Name "bio-sup" -ErrorAction SilentlyContinue
        $bioSupBefore | Should -Not -BeNullOrEmpty
        $pidBefore = $bioSupBefore.Id
        $bioSupPathBefore = $bioSupBefore.Path
        Write-Host "Current bio-sup PID before second migration: $pidBefore"
        Write-Host "Current bio-sup path before second migration: $bioSupPathBefore"

        # Verify that bio-sup is currently using biome/bio-sup
        $bioSupPathBefore | Should -Match "biome\\bio-sup"

        # Get the current version before the second migration
        $versionBefore = $null
        if ($bioSupPathBefore -match 'biome\\bio-sup\\([^/\\]+)') {
            $versionBefore = $Matches[1]
            Write-Host "Current bio-sup version before second migration: $versionBefore"
        }

        # Run the migration script with test auth token a second time
        Write-Host "Running migration script a second time..."
        components/bio/migrate.ps1
        $LASTEXITCODE | Should -Be 0

        # Check the bio-sup process after the second migration
        Start-Sleep -Seconds 5 # Give a moment for any potential service restart
        $bioSupAfter = Get-Process -Name "bio-sup" -ErrorAction SilentlyContinue
        $bioSupAfter | Should -Not -BeNullOrEmpty
        $pidAfter = $bioSupAfter.Id
        $bioSupPathAfter = $bioSupAfter.Path
        Write-Host "Current bio-sup PID after second migration: $pidAfter"
        Write-Host "Current bio-sup path after second migration: $bioSupPathAfter"

        # Verify that the bio-sup PID has not changed (no restart occurred)
        $pidAfter | Should -Be $pidBefore

        # Get the version after the second migration
        $versionAfter = $null
        if ($bioSupPathAfter -match 'biome\\bio-sup\\([^/\\]+)') {
            $versionAfter = $Matches[1]
            Write-Host "Current bio-sup version after second migration: $versionAfter"
        }

        # Verify that the version is the same
        $versionAfter | Should -Be $versionBefore

        # Double-check that the service is still running properly
        $biomeService = Get-Service -Name "Biome" -ErrorAction SilentlyContinue
        $biomeService | Should -Not -BeNullOrEmpty
        $biomeService.Status | Should -Be "Running"
    }
}