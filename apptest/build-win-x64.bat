@echo off

set destDir=.\target\debug

copy ".\win64\liblcl.lib" ".\liblcl.lib" /V

cargo rustc -- -C link-args="/MANIFEST /manifest:embed /manifestinput:app.manifest /SAFESEH:NO /MACHINE:X64 app.res"

if %errorlevel%==0 (
	if not exist "%destDir%\liblcl.dll" (copy ".\win64\liblcl.dll" "%destDir%\liblcl.dll") 
	rem if not exist "%destDir%\applogo.ico" (copy ".\applogo.ico" "%destDir%\applogo.ico") 
	if exist "%destDir%\apptest.exe" (start "" "%destDir%\apptest.exe") 
) else (
	pause
)