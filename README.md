# stay-awake2
Keep a Windows machine awake

[![stay awake](https://github.com/curtisalexander/stay-awake2/actions/workflows/ci.yml/badge.svg)](https://github.com/curtisalexander/stay-awake2/actions/workflows/ci.yml)

## Get
Executable binaries for Windows may be found at the [Release](https://github.com/curtisalexander/stay-awake2/releases) page.

## Usage
The executable `stay-awake.exe` is intended to be run in a terminal in order to keep one's Windows machine awake.

There are two modes one may choose from:
- **System** [Default] &rarr; the machine will not go to sleep but the display could turn off
- **Display** &rarr; the machine will not go to sleep and the display will remain on

### System
The simplest use case is to run the executable without any switches.

```pwsh
stay-awake.exe
```

This will prevent the machine from going to sleep and will await the user pressing the `Enter` key within the terminal before resetting the machine state.

### Display
To keep the machine awake and prevent the display from turning off, utilize the `--display` switch.

```pwsh
stay-awake.exe --display
```

This will prevent the machine from going to sleep (while also keeping the display on) and will await the user pressing the `Enter` key within the terminal before resetting the machine state.

> :memo: As noted in the [Win32 documentation](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setthreadexecutionstate), use of the `SetThreadExecutionState` function (which is the Win32 function called by `stay-awake.exe`) does **_not_** prevent one from putting their computer to sleep by either closing the lid on their laptop or pressing the power button.  In addition, the screen saver may still execute.

### Help
Result of running `stay-awake.exe --help`

```
stay-awake
Keep a Windows machine awake

USAGE:
    stay-awake.exe [OPTIONS]

OPTIONS:
        --display    Keep display on
    -h, --help       Print help information
    -V, --version    Print version information
```

## Testing
In order to test, open PowerShell with elevated (admin) privileges. After executing the program, run the following.

```pwsh
powercfg -requests
```

## Win32 Docs
Application utilizes [SetThreadExecutionState](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setthreadexecutionstate) from the [Win32 API](https://docs.microsoft.com/en-us/windows/win32/).

## Prior Implementations
- [`C#`](https://github.com/curtisalexander/stay-awake-cs)
- [`Rust`](https://github.com/curtisalexander/stay-awake-rs)
    - Loads `kernel32.dll` and performs a [transmute](https://doc.rust-lang.org/stable/std/mem/fn.transmute.html) to get the function [SetThreadExecutionState](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setthreadexecutionstate)

## Alternate Tools
- [Microsoft PowerToys](https://docs.microsoft.com/en-us/windows/powertoys/) includes the [Awake](https://docs.microsoft.com/en-us/windows/powertoys/awake) utility
    - It [also utilizes SetThreadExectionState](https://github.com/microsoft/PowerToys/blob/main/src/modules/awake/Awake/Core/APIHelper.cs#L66-L84) to keep a Windows machine awake
