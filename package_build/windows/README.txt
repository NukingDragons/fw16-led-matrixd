I highly recommend just using the install method with powershell
But if you insist...

Place the exe files wherever you want, and then create a windows service targetting the "fw16-led-matrixd.exe" binary.
Use the "ledcli.exe" binary to interact with the service

Create a windows service like so:
sc.exe create fw16-led-matrixd binpath= "C:\Path\To\Matrixd.exe \"-l C:\Path\To\LogFile.log -c C:\Path\To\Config.toml\""
