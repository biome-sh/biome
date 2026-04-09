$pkg_name="testpkgbindproducer"
$pkg_origin="biome-testing"
$pkg_version="0.1.0"
$pkg_maintainer="The Biome Maintainers <humans@biome.sh>"
$pkg_license=@("Apache-2.0")
$pkg_exports=@{
    "setting" = "setting1"
}

function Invoke-Install {
    mkdir "$pkg_prefix\hooks"
    Set-Content -Path "$pkg_prefix\hooks\run" -Value "while (`$true) { Start-Sleep 1 }"
}
