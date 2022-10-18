$dep = "SFML-2.5.1"
$dlls = (
    "openal32.dll",
    "sfml-audio-2.dll", 
    "sfml-graphics-2.dll", 
    "sfml-system-2.dll", 
    "sfml-window-2.dll"
)

$dll_missing_run = @()
foreach ($dll in $dlls) {
    if (!(Test-Path "./$dll")) {
        $dll_missing_run += $dll
    }
}
$dll_missing_build = @()
foreach ($dll in $dlls) {
    if (!(Test-Path "target/debug/$dll")) {
        $dll_missing_build = $dll
    }
}


if ((("run" -in $args) -and $dll_missing_run) -or (("--build" -in $args) -and $dll_missing_build)) {
    if (!(Test-Path "tmp")) {
        New-Item "tmp" -Type Directory >$null
        Write-Host "'./tmp' created."
    }
    
    if (!(Test-Path "tmp/$dep.zip")) {
        Write-Host "Downloading 'https://www.sfml-dev.org/files/$dep-windows-vc15-64-bit.zip' . . ."
        Invoke-WebRequest -Uri "https://www.sfml-dev.org/files/$dep-windows-vc15-64-bit.zip" -OutFile "tmp/$dep.zip" >$null
    }
    
    if (!(Test-Path "tmp/$dep")) {
        Expand-Archive "tmp/$dep.zip" -DestinationPath "tmp/" -Force >$null
        Write-Host "'tmp/$dep.zip' to 'tmp/$dep' extracted."
    }
}


if ("run" -in $args) {
    foreach ($dll in $dll_missing_run) {
        Copy-Item -Path "tmp/$dep/bin/$dll" -Destination . -PassThru >$null
        Write-Host "'./$dll' created."
    }
}

if ("build" -in $args)  {
    if (Test-Path "target/debug/") {
        foreach ($dll in $dll_missing_build) {
            Copy-Item -Path $dll -Destination "target/debug/" -PassThru >$null
            Write-Host "'target/debug/$dll' created."
        }
    }
    else {
        Write-Host "Make sure to run: cargo build"
    }
}

if ("clean" -in $args) {
    foreach ($dll in $dlls) {
        if (Test-Path "./$dll") {
            Remove-Item -Path "./$dll"
            Write-Host "'./$dll' removed."
        }
    }
    foreach ($dll in $dlls) {
        if (Test-Path "target/debug/$dll") {
            Remove-Item -Path "target/debug/$dll"
            Write-Host "'target/debug/$dll' removed."
        }
    }
    if ((Test-Path "tmp") -and ("*" -in $args)) {
        Remove-Item -LiteralPath "tmp" -Force -Recurse
        Write-Host "'tmp' removed."
    }
}
