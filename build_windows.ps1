<#
.SYNOPSIS
    Build Honse Helper for Windows — checks dependencies, builds frontend + Rust backend.
.DESCRIPTION
    Verifies Rust toolchain, Visual Studio Build Tools, WebView2, and Trunk;
    then runs trunk build + cargo build.
#>

$ErrorActionPreference = "Stop"
$Host.UI.RawUI.WindowTitle = "Honse Helper — Windows Build"

# ── helpers ──────────────────────────────────────────────────────────────────
function Write-Step($msg)  { Write-Host "`n>> $msg" -f Cyan }
function Write-Info($msg)  { Write-Host "   $msg" -f Gray }
function Write-Ok($msg)    { Write-Host "   ✓ $msg" -f Green }
function Write-Warn($msg)  { Write-Host "   ⚠ $msg" -f Yellow }
function Write-Fail($msg)  { Write-Host "   ✗ $msg" -f Red; exit 1 }

function Test-Command($name) { $null -ne (Get-Command $name -ErrorAction SilentlyContinue) }

$RootDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# ── 1. Rust toolchain ────────────────────────────────────────────────────────
Write-Step "1/8  Rust toolchain"

if (-not (Test-Command rustc)) {
    Write-Warn "rustc not found. Downloading rustup-init.exe …"
    $url = "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe"
    $exe = "$env:TEMP\rustup-init.exe"
    Invoke-WebRequest -Uri $url -OutFile $exe
    Start-Process -Wait -FilePath $exe -ArgumentList "-y --default-host x86_64-pc-windows-msvc"
    # reload PATH
    $env:Path = [Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [Environment]::GetEnvironmentVariable("Path","User")
    # refresh for the rest of this script
    $rustupHome = Join-Path $env:USERPROFILE ".cargo\bin"
    if (Test-Path $rustupHome) { $env:Path = "$rustupHome;$env:Path" }
    if (-not (Test-Command rustc)) { Write-Fail "rustc still not found after install. Restart terminal and re-run." }
}
Write-Ok "rustc $(& rustc -V | Select-Object -First 1)"

if (-not (Test-Command cargo)) { Write-Fail "cargo not found" }
Write-Ok "cargo $(& cargo -V | Select-Object -First 1)"

# ── 2. wasm32 target ─────────────────────────────────────────────────────────
Write-Step "2/8  wasm32-unknown-unknown target"
$targets = & rustup target list --installed
if ($targets -notcontains "wasm32-unknown-unknown") {
    Write-Info "Installing wasm32-unknown-unknown …"
    & rustup target add wasm32-unknown-unknown
    if ($LASTEXITCODE -ne 0) { Write-Fail "rustup target add failed" }
}
Write-Ok "wasm32-unknown-unknown ready"

# ── 3. Trunk (WASM bundler) ──────────────────────────────────────────────────
Write-Step "3/8  Trunk (WASM bundler)"
if (-not (Test-Command trunk)) {
    Write-Info "Installing trunk via cargo …"
    & cargo install trunk
    if ($LASTEXITCODE -ne 0) { Write-Fail "cargo install trunk failed" }
}
Write-Ok "trunk $(& trunk -V | Select-Object -First 1)"

# ── 4. Visual Studio Build Tools ─────────────────────────────────────────────
Write-Step "4/8  Visual Studio Build Tools (C++ workload)"

$vsInstalled = $false
$vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (-not (Test-Path $vsWhere)) {
    $vsWhere = "${env:ProgramFiles}\Microsoft Visual Studio\Installer\vswhere.exe"
}

if (Test-Path $vsWhere) {
    $instance = & $vsWhere -latest -requires Microsoft.VisualStudio.Workload.NativeDesktop -property installationPath
    if ($instance) {
        $vsInstalled = $true
        Write-Ok "Visual Studio detected: $instance"
    }
}

if (-not $vsInstalled) {
    Write-Warn "Visual Studio with 'Desktop development with C++' workload NOT detected."
    Write-Warn "The build will likely fail at the C++ linking step (tauri-build / rusqlite)."
    Write-Info ""
    Write-Info "Options:"
    Write-Info "  A) Install Visual Studio Community (free) with the C++ workload:"
    Write-Info "     https://visualstudio.microsoft.com/vs/community/"
    Write-Info "     → installer  → Workloads tab  → 'Desktop development with C++'"
    Write-Info ""
    Write-Info "  B) Install Build Tools (smaller, CLI-only):"
    Write-Info "     https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio"
    Write-Info "     Run: vs_BuildTools.exe --add Microsoft.VisualStudio.Workload.NativeDesktop"
    Write-Info ""
    $yn = Read-Host "Continue anyway? (y/N)"
    if ($yn -ne "y" -and $yn -ne "Y") { exit 1 }
}

# ── 5. WebView2 Runtime ──────────────────────────────────────────────────────
Write-Step "5/8  WebView2 Runtime"

$wv2 = $false
# check via registry (Windows 10 1803+ / 11)
$regPaths = @(
    "HKLM:\SOFTWARE\Microsoft\Edge\WebView2",
    "HKLM:\SOFTWARE\WOW6432Node\Microsoft\Edge\WebView2",
    "HKCU:\SOFTWARE\Microsoft\Edge\WebView2"
)
foreach ($p in $regPaths) {
    if (Test-Path $p) { $wv2 = $true; break }
}
# also check installed package
if (-not $wv2) {
    $pkg = Get-AppxPackage -Name "Microsoft.WebView2" -ErrorAction SilentlyContinue
    if ($pkg) { $wv2 = $true }
}
# check common install location
if (-not $wv2) {
    $sysPath = "$env:SYSTEMROOT\System32\WebView2Loader.dll"
    if (Test-Path $sysPath) { $wv2 = $true }
}

if ($wv2) {
    Write-Ok "WebView2 Runtime detected"
} else {
    Write-Warn "WebView2 Runtime not detected."
    Write-Warn "Download from: https://developer.microsoft.com/en-us/microsoft-edge/webview2/"
    Write-Warn "Or run: https://go.microsoft.com/fwlink/p/?LinkId=2124703 (Evergreen Bootstrapper)"
    Write-Info ""
    $yn = Read-Host "Continue anyway? (y/N)"
    if ($yn -ne "y" -and $yn -ne "Y") { exit 1 }
}

# ── 6. Build frontend ────────────────────────────────────────────────────────
Write-Step "6/8  Build frontend (trunk build)"

$frontendDir = Join-Path $RootDir "honse-helper"
if (-not (Test-Path $frontendDir)) { Write-Fail "Frontend directory not found: $frontendDir" }

Push-Location $frontendDir
try {
    $env:TRUNK_DIST_DIR = "dist"
    & trunk build --release
    if ($LASTEXITCODE -ne 0) { Write-Fail "trunk build failed" }
    Write-Ok "Frontend built → $frontendDir\dist"
} finally {
    Pop-Location
}

# ── 7. Build Rust backend ────────────────────────────────────────────────────
Write-Step "7/8  Build Rust binaries (cargo build --release)"

Push-Location $RootDir
try {
    & cargo build --release
    if ($LASTEXITCODE -ne 0) { Write-Fail "cargo build failed" }
    Write-Ok "Rust binaries built → target\release\"
} finally {
    Pop-Location
}

# ── 8. Summary ───────────────────────────────────────────────────────────────
Write-Step "8/8  Build complete"

$release = Join-Path $RootDir "target\release"
$artifacts = @(
    "honse_helper.exe",
    "honse_worker.exe"
)
foreach ($bin in $artifacts) {
    $path = Join-Path $release $bin
    if (Test-Path $path) {
        $size = (Get-Item $path).Length / 1MB
        Write-Ok "$bin  ({0:N1} MB)" -f $size
    } else {
        Write-Warn "$bin not found (expected at $path)"
    }
}

Write-Host ""
Write-Host "══════════════════════════════════════════════════════════════" -f Cyan
Write-Host "  Honse Helper Windows build finished successfully!" -f Green
Write-Host "  Binaries: $release" -f Gray
Write-Host "  Run: honse_helper.exe" -f Gray
Write-Host "══════════════════════════════════════════════════════════════" -f Cyan
