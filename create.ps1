$project_name = $args[0]
Write-Host "Execute: cargo new $project_name"
cargo new $project_name
Write-Host "Execute: Copy-Item .gitignore to $project_name"
Copy-Item -Path .\.gitignore -Destination .\$project_name\.
