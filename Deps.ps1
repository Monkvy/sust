$build_folder = './target/debug'
$temp_folder = './temp'

$dep = "SFML-2.5.1"
$dlls = (
    "openal32.dll",
    "sfml-audio-2.dll", 
    "sfml-graphics-2.dll", 
    "sfml-system-2.dll", 
    "sfml-window-2.dll"
)

$dll_missing = @()
foreach ($dll in $dlls) {
    if (!(Test-Path "$build_folder/$dll")) {
        $dll_missing += $dll
    }
}

if (("build" -in $args) -and $dll_missing) {
    if (!(Test-Path $temp_folder)) {
        New-Item $temp_folder -Type Directory >$null
        Write-Host "'$temp_folder' created."
    }
    
    if (!(Test-Path "$temp_folder/$dep.zip")) {
        Write-Host "Downloading 'https://www.sfml-dev.org/files/$dep-windows-vc15-64-bit.zip' . . ."
        Invoke-WebRequest -Uri "https://www.sfml-dev.org/files/$dep-windows-vc15-64-bit.zip" -OutFile "$temp_folder/$dep.zip" >$null
    }
    
    if (!(Test-Path "$temp_folder/$dep")) {
        Expand-Archive "$temp_folder/$dep.zip" -DestinationPath "$temp_folder/" -Force >$null
        Write-Host "'$temp_folder/$dep.zip' extracted."

        Get-ChildItem -Path "$temp_folder/$dep" -Exclude 'bin' |
        ForEach-Object {Remove-Item $_ -Recurse }
        Write-Host "Removed unnecessary items inside '$temp_folder/$dep'."
    }

    Remove-Item "$temp_folder/$dep.zip"
    Write-Host "Removed '$temp_folder/$dep.zip'."
}

if ("build" -in $args)  {
    if (Test-Path $build_folder) {
        foreach ($dll in $dll_missing) {
            Copy-Item -Path "$temp_folder/$dep/bin/$dll" -Destination "$build_folder/" -PassThru >$null
            Write-Host "'$build_folder/$dll' created."
        }
    }
    else {
        Write-Host "'$build_folder' does not exist. Make sure to run: cargo build"
    }
}

if ("clean" -in $args) {
    if (Test-Path $temp_folder) {
        Remove-Item -Path $temp_folder -Recurse
        Write-Host "'$temp_folder' removed."
    }
    if ("*" -in $args) {
        foreach ($dll in $dlls) {
            if (Test-Path $build_folder/$dll) {
                Remove-Item -Path "$build_folder/$dll"
                Write-Host "'$build_folder/$dll' removed."
            }
        }
    }
}
