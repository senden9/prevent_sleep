# Prevent Sleep

## Keep Your Windows PC Awake

This small utility prevents your Windows PC from automatically turning off the screen or going to sleep.

## How it works

This tool utilizes [`SetThreadExecutionState`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setthreadexecutionstate) to inform the system that it is in use, thereby preventing the system from entering sleep or turning off the display while the application is running.
