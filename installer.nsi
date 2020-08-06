Name "svg2pdf"

RequestExecutionLevel User

OutFile "svg2pdf-windows.exe"
InstallDir $PROFILE\.svg2pdf

Section

    CreateDirectory $INSTDIR\bin
    SetOutPath $INSTDIR\bin
    File target\release\svg2pdf.exe

    EnVar::AddValue "PATH" "$INSTDIR\bin"
    Pop $0

    SetOutPath $INSTDIR
    WriteUninstaller $INSTDIR\uninstall.exe

    MessageBox MB_OK "Success! Installed to: $INSTDIR$\n$\nTo get started, restart your terminal and \
        run the following command:$\n$\n    svg2pdf --help$\n$\nTo uninstall run: $INSTDIR\uninstall.exe"

SectionEnd

Section "Uninstall"

    EnVar::DeleteValue "PATH" "$INSTDIR\bin"
    Pop $0

    Delete $INSTDIR\uninstall.exe
    Delete $INSTDIR\bin\svg2pdf.exe
    RMDir $INSTDIR\bin
    RMDir $INSTDIR

    RMDir /r $LOCALAPPDATA\Svg2pdf

SectionEnd
