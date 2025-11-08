[Setup]
AppName=RS File Sorter
AppVersion={{VERSION}}
DefaultDirName={commonpf}\RSFileSorter
OutputDir=.
OutputBaseFilename=RSFileSorterSetup
Compression=lzma
SolidCompression=yes
DefaultGroupName=RS File Sorter
UninstallDisplayIcon={app}\rsfs-app.exe
DisableDirPage=no
DisableProgramGroupPage=no
  
[Files]
Source: "target\release\rsfs-app.exe"; DestDir: "{app}";
Source: "README.md"; DestDir: "{app}"; Flags: isreadme

[Dirs]
Name: "{app}\rsc"; Permissions: users-full

[Icons]
Name: "{group}\RS File Sorter"; Filename: "{app}\rsfs-app.exe"
Name: "{group}\Uninstall RS File Sorter"; Filename: "{uninstallexe}"