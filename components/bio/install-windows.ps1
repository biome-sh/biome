# Set the script to stop upon error
$ErrorActionPreference="stop"

# If the variable `$env:DEBUG` is set, then print the shell commands as we execute.
if (-not ([string]::IsNullOrEmpty($env:DEBUG))) {
    Set-PSDebug -Trace 2
}

Set-Variable BIO_ZIP_URL -Option ReadOnly -value "https://github.com/biome-sh/biome/releases/download/bio-1.5.75/bio-1.5.75-x86_64-windows.zip"
Set-Variable BIO_ZIP_SHA256 -Option ReadOnly -value "6AD638B4D4B7E33A9C7FCFEF3317B3F0B7243E162FD26F8411BE97255CEB633E"

Write-Output "Downloading Biome bootstrap binary."

# Create a temp working directory
$tmp_parent = [System.IO.Path]::GetTempPath()
$tmp_dir = [System.Guid]::NewGuid()
$work_dir = New-Item -Path (Join-Path $tmp_parent $tmp_dir ) -ItemType Directory

try {
    # Download the biome zip
    $zip_filename = $BIO_ZIP_URL.split("/")[-1]
    $zip_dest = (Join-Path ($work_dir) $zip_filename)
    $web_client = New-Object System.Net.WebClient
    $web_client.DownloadFile($BIO_ZIP_URL, $zip_dest)

    # Validate the SHA256 of the archive
    $shasum = Get-FileHash $zip_dest
    if ($shasum.hash -ne $BIO_ZIP_SHA256) {
      throw "Biome archive SHA256 checksum is different than expected."
    }

    # Unpack the archive
    Expand-Archive $zip_dest $work_dir

    # Use Biome bootstrap binary to install biome/bio from bldr
    $bio_exe_path = (Join-Path ($work_dir) "bio.exe")
    & "$bio_exe_path" pkg install -fb biome/bio | Out-Null

    Write-Output "Installation of Biome 'bio' program complete."

} finally {
    # Cleanup the temp working directory
    try { Remove-Item $work_dir -Recurse -Force } catch {
        Write-Warning "Unable to delete $work_dir"
    }
}
