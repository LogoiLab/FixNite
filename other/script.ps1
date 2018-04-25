$LocalAppDataFolder = "$env:LOCALAPPDATA"
$FortniteReplaysFolder = $LocalAppDataFolder + "\FortniteGame\Saved\Demos"
$count = 0
Get-Childitem $FortniteReplaysFolder -Filter *.replay |
  Foreach-Object {
      $bytes  = [System.IO.File]::ReadAllBytes($_.fullname)
      $offset = 0x10
      if($bytes[$offset] -ne 0x2a){
          "Fixing: " + $_.Name
          $bytes[$offset]   = 0x2a
          $bytes[$offset+1] = 0x2a
          $bytes[$offset+2] = 0x3d
          [System.IO.File]::WriteAllBytes($_.DirectoryName + "\fixed-" + $_.Name, $bytes)
          $count++
      }
      else{ "Already correct version: " + $_.Name }
  }
"Fixed " + $count + " Files."
"Press a key..."; $x = $host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
