@echo off
echo Setting up Git hooks...

REM Set Git hooks directory
git config core.hooksPath .githooks

REM Make hooks executable (Windows doesn't need chmod, but we'll add Git Bash compatibility)
if exist "%PROGRAMFILES%\Git\bin\bash.exe" (
    "%PROGRAMFILES%\Git\bin\bash.exe" -c "chmod +x .githooks/pre-commit .githooks/pre-push"
) else (
    echo Git Bash not found in default location. Hooks created but may need manual permission setup.
)

echo ✅ Git hooks setup complete!
echo Run this script once to enable the hooks for this repository.
pause