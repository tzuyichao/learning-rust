$library_name = $args[0]
Write-Host "Execute: cargo new --lib $library_name"
cargo new --lib $library_name
Write-Host "Execute: Copy-Item .gitignore to $library_name"
Copy-Item -Path .\.gitignore -Destination .\$library_name\.
