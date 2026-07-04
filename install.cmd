@echo off
:: Copyright (C) 2026 The pgmoneta community
::
:: This program is free software: you can redistribute it and/or modify
:: it under the terms of the GNU General Public License as published by
:: the Free Software Foundation, either version 3 of the License, or
:: (at your option) any later version.
::
:: This program is distributed in the hope that it will be useful,
:: but WITHOUT ANY WARRANTY; without even the implied warranty of
:: MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
:: GNU General Public License for more details.
::
:: You should have received a copy of the GNU General Public License
:: along with this program. If not, see <https://www.gnu.org/licenses/>.
:: Usage: curl -fsSL https://raw.githubusercontent.com/pgmoneta/pgmoneta_mcp/main/install.cmd -o install.cmd && install.cmd
:: Override install directory: set "INSTALL_DIR=C:\Tools" && install.cmd
setlocal EnableDelayedExpansion

if not defined REPO set "REPO=pgmoneta/pgmoneta_mcp"
if not defined INSTALL_DIR set "INSTALL_DIR=%USERPROFILE%\.local\bin"
set "TMP=%TEMP%\pgmoneta-mcp-install-%RANDOM%%RANDOM%"

where powershell >nul 2>&1 || (echo error: PowerShell is required & exit /b 1)

echo Fetching latest release...
for /f "usebackq delims=" %%v in (`powershell -NoProfile -NonInteractive -Command ^
    "(Invoke-RestMethod 'https://api.github.com/repos/%REPO%/releases/latest').tag_name"`) do set "VERSION=%%v"
if "!VERSION!"=="" (echo error: could not fetch latest release & exit /b 1)
echo Version: !VERSION!

set "ASSET=pgmoneta-mcp-!VERSION!-x86_64-pc-windows-msvc.zip"
set "URL=https://github.com/%REPO%/releases/download/!VERSION!/!ASSET!"

echo Downloading !ASSET!...
mkdir "!TMP!" >nul 2>&1
powershell -NoProfile -NonInteractive -Command ^
    "Invoke-WebRequest -Uri '!URL!' -OutFile '!TMP!\!ASSET!' -UseBasicParsing" || ^
    (echo error: download failed & rd /s /q "!TMP!" >nul 2>&1 & exit /b 1)

powershell -NoProfile -NonInteractive -Command ^
    "$ErrorActionPreference='Stop'; Expand-Archive -Path '!TMP!\!ASSET!' -DestinationPath '!TMP!\out' -Force" || ^
    (echo error: could not extract archive & rd /s /q "!TMP!" >nul 2>&1 & exit /b 1)
if not exist "!TMP!\out\pgmoneta-mcp-server.exe" (echo error: binary not found in archive: pgmoneta-mcp-server.exe & rd /s /q "!TMP!" >nul 2>&1 & exit /b 1)
if not exist "!TMP!\out\pgmoneta-mcp-admin.exe" (echo error: binary not found in archive: pgmoneta-mcp-admin.exe & rd /s /q "!TMP!" >nul 2>&1 & exit /b 1)
if not exist "!TMP!\out\pgmoneta-mcp-client.exe" (echo error: binary not found in archive: pgmoneta-mcp-client.exe & rd /s /q "!TMP!" >nul 2>&1 & exit /b 1)
if not exist "!TMP!\out\pgmoneta-mcp-inspector.exe" (echo error: binary not found in archive: pgmoneta-mcp-inspector.exe & rd /s /q "!TMP!" >nul 2>&1 & exit /b 1)

if not exist "!INSTALL_DIR!" mkdir "!INSTALL_DIR!"
copy /y "!TMP!\out\pgmoneta-mcp-server.exe" "!INSTALL_DIR!\pgmoneta-mcp-server.exe" >nul || ^
    (echo error: could not write pgmoneta-mcp-server.exe to !INSTALL_DIR! & exit /b 1)
copy /y "!TMP!\out\pgmoneta-mcp-admin.exe" "!INSTALL_DIR!\pgmoneta-mcp-admin.exe" >nul || ^
    (echo error: could not write pgmoneta-mcp-admin.exe to !INSTALL_DIR! & exit /b 1)
copy /y "!TMP!\out\pgmoneta-mcp-client.exe" "!INSTALL_DIR!\pgmoneta-mcp-client.exe" >nul || ^
    (echo error: could not write pgmoneta-mcp-client.exe to !INSTALL_DIR! & exit /b 1)
copy /y "!TMP!\out\pgmoneta-mcp-inspector.exe" "!INSTALL_DIR!\pgmoneta-mcp-inspector.exe" >nul || ^
    (echo error: could not write pgmoneta-mcp-inspector.exe to !INSTALL_DIR! & exit /b 1)

rd /s /q "!TMP!" >nul 2>&1
echo Installed:
echo   !INSTALL_DIR!\pgmoneta-mcp-server.exe
echo   !INSTALL_DIR!\pgmoneta-mcp-admin.exe
echo   !INSTALL_DIR!\pgmoneta-mcp-client.exe
echo   !INSTALL_DIR!\pgmoneta-mcp-inspector.exe
echo Run "pgmoneta-mcp-client --help" to get started.
echo Run "pgmoneta-mcp-server --help" to see server options.
endlocal