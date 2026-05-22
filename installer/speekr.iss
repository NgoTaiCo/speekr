; speekr Installer Script
; https://github.com/NgoTaiCo/speekr

#define AppName "speekr"
#define AppVersion "1.0.0"
#define AppPublisher "NgoTaiCo"
#define AppURL "https://github.com/NgoTaiCo/speekr"
#define AppExeName "speekr.exe"
#define AppDescription "Neural text-to-speech and translation from your system tray"

[Setup]
AppId={{A5B3C7D2-4E8F-4A1B-9C3D-5F6E7A8B9C0D}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher={#AppPublisher}
AppPublisherURL={#AppURL}
AppSupportURL={#AppURL}/issues
AppUpdatesURL={#AppURL}/releases
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
AllowNoIcons=yes
LicenseFile=..\LICENSE
OutputDir=Output
OutputBaseFilename=speekr-setup-{#AppVersion}
Compression=lzma
SolidCompression=yes
WizardStyle=modern
WizardResizable=no
MinVersion=10.0
PrivilegesRequired=lowest
UninstallDisplayName={#AppName}

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon";  Description: "{cm:CreateDesktopIcon}";          GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "startupentry"; Description: "Start speekr automatically on Windows login"; GroupDescription: "Startup:";              Flags: unchecked

[Files]
Source: "..\target\release\{#AppExeName}"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
; Start Menu
Name: "{group}\{#AppName}";                         Filename: "{app}\{#AppExeName}"; Comment: "{#AppDescription}"
Name: "{group}\{cm:UninstallProgram,{#AppName}}";   Filename: "{uninstallexe}"
; Desktop (optional task)
Name: "{autodesktop}\{#AppName}";                   Filename: "{app}\{#AppExeName}"; Comment: "{#AppDescription}"; Tasks: desktopicon

[Registry]
; Run-on-startup (optional task) — stored per-user so no admin rights needed
Root: HKCU; Subkey: "SOFTWARE\Microsoft\Windows\CurrentVersion\Run"; \
  ValueType: string; ValueName: "{#AppName}"; \
  ValueData: """{app}\{#AppExeName}"""; \
  Flags: uninsdeletevalue; Tasks: startupentry

[Run]
Filename: "{app}\{#AppExeName}"; \
  Description: "{cm:LaunchProgram,{#AppName}}"; \
  Flags: nowait postinstall skipifsilent

[Code]
// Check for Python 3
function PythonFound(): Boolean;
var
  ResultCode: Integer;
begin
  Result := Exec(ExpandConstant('python'), '--version', '', SW_HIDE,
                 ewWaitUntilTerminated, ResultCode)
            and (ResultCode = 0);
end;

// Check for edge-tts pip package
function EdgeTtsFound(): Boolean;
var
  ResultCode: Integer;
begin
  Result := Exec(ExpandConstant('python'), '-c "import edge_tts"', '',
                 SW_HIDE, ewWaitUntilTerminated, ResultCode)
            and (ResultCode = 0);
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then
  begin
    if not PythonFound() then
      MsgBox(
        'Python 3 was not found on this system.' + #13#10 + #13#10 +
        'speekr requires Python 3 + edge-tts for neural voice quality.' + #13#10 +
        'Without it, speekr will fall back to the built-in Windows voice.' + #13#10 + #13#10 +
        '  1. Install Python from https://python.org' + #13#10 +
        '  2. Run: pip install edge-tts',
        mbInformation, MB_OK)
    else if not EdgeTtsFound() then
      MsgBox(
        'The edge-tts package is not installed.' + #13#10 + #13#10 +
        'speekr will fall back to the built-in Windows voice until you run:' + #13#10 + #13#10 +
        '  pip install edge-tts',
        mbInformation, MB_OK);
  end;
end;
