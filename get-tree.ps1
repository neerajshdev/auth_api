function Get-Tree {
    param (
        [string]$Path = ".",
        [string[]]$Exclude
    )

    function Get-TreeRecursive {
        param (
            [string]$CurrentPath,
            [string[]]$Exclude,
            [string]$Prefix = ""
        )

        $items = Get-ChildItem -Path $CurrentPath | Where-Object { 
            $name = $_.Name
            $excluded = $false
            foreach ($pattern in $Exclude) {
                if ($name -eq $pattern) {
                    $excluded = $true
                    break
                }
            }
            -not $excluded
        }

        $count = $items.Count
        $index = 0

        foreach ($item in $items) {
            $index++
            $isLast = $index -eq $count
            $connector = if ($isLast) { "└── " } else { "├── " }
            Write-Output "$Prefix$connector$($item.Name)"

            if ($item.PSIsContainer) {
                $newPrefix = if ($isLast) { "$Prefix    " } else { "$Prefix│   " }
                Get-TreeRecursive -CurrentPath $item.FullName -Exclude $Exclude -Prefix $newPrefix
            }
        }
    }

    Get-TreeRecursive -CurrentPath $Path -Exclude $Exclude
}

Get-Tree -Path "." -Exclude "target", "node_modules", ".idea", "readme.md", "markdown", "get-tree.ps1"
