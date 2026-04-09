@echo off
pwsh.exe -NoProfile -ExecutionPolicy bypass -NoLogo -Command ". '%~dp0biome.ps1';Uninstall-BioService" %*
