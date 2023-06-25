@echo off

REM Default values for development mode
set SERVER_ADDRESS=127.0.0.1
set SERVER_PORT=8000

REM Initialize variables for argument parsing
set MODE=
set SKIP_BUILD=
set OVERWRITE_ADDRESS=
set OVERWRITE_PORT=
set DEFAULT_PROD_ADDRESS = vertec.io/vertex-management-service
set DEFAULT_PROD_PORT = 443

REM Parse the arguments
:parse_args
if "%~1"=="" (
    goto check_mode
)

if "%~1"=="-mode" (
    set MODE=%~2
    shift
    shift
    goto parse_args
)

if "%~1"=="-no-build" (
    set SKIP_BUILD=true
    shift
    goto parse_args
)

if "%~1"=="-address" (
    set OVERWRITE_ADDRESS=%~2
    shift
    shift
    goto parse_args
)

if "%~1"=="-port" (
    set OVERWRITE_PORT=%~2
    shift
    shift
    goto parse_args
)

if "%~1"=="-help" (
    goto display_help
)

goto parse_args

REM Check the mode selection
:check_mode
echo %MODE%
if "%MODE%"=="prod" (
    REM Production mode configuration
    if defined OVERWRITE_ADDRESS (
        set SERVER_ADDRESS=%OVERWRITE_ADDRESS%
    ) else (
        set SERVER_ADDRESS=%DEFAULT_PROD_ADDRESS%
    )

    if defined OVERWRITE_PORT (
        set SERVER_PORT=%OVERWRITE_PORT%
    ) else (
        set SERVER_PORT=%DEFAULT_PROD_PORT
    )

    if defined SKIP_BUILD (
        REM Skip build step
        REM Run the compiled executable
        target\release\your_executable_name
        exit /b
    ) else (
        REM Build the application in release mode
        cargo build --release

        REM Run the compiled executable
        target\release\your_executable_name
        exit /b
    )
)

REM Development mode configuration
if defined OVERWRITE_ADDRESS (
    set SERVER_ADDRESS=%OVERWRITE_ADDRESS%
)

if defined OVERWRITE_PORT (
    set SERVER_PORT=%OVERWRITE_PORT%
)

REM Run the application using `cargo run` in dev mode
cargo run
exit /b

REM Display the help documentation
:display_help
echo "Usage: start_server.bat -mode [prod|dev] [-no-build] [-address <address>] [-port <port>]"
echo.
echo Options:
echo   -mode         Select the server mode: prod or dev.
echo   -no-build     Skip the build step for production mode.
echo   -address, -a  Specify the server address.
echo   -port, -p     Specify the server port.
echo   -help         Display this help documentation.
exit /b

