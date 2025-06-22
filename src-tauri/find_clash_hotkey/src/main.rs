use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::ProcessStatus::K32GetModuleFileNameExW;
use windows::Win32::Foundation::HANDLE;
use std::ptr::null_mut;
use std::ffi::OsString;
use std::os::windows;
use std::os::windows::ffi::OsStringExt;
use std::os::windows::raw::HANDLE;

fn get_foreground_process_name() -> Option<String> {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0 == 0 {
            return None;
        }
        let mut pid = 0;
        GetWindowThreadProcessId(hwnd, &mut pid);
        if pid == 0 {
            return None;
        }

        let process_handle: HANDLE = OpenProcess(windows::Win32::System::Threading::PROCESS_QUERY_LIMITED_INFORMATION | windows::Win32::System::Threading::PROCESS_VM_READ, false, pid);
        if process_handle.0 == 0 {
            return None;
        }

        let mut buffer = [0u16; 260];
        let len = K32GetModuleFileNameExW(process_handle, None, &mut buffer);
        if len == 0 {
            return None;
        }
        let os_string = OsString::from_wide(&buffer[..len as usize]);
        Some(os_string.to_string_lossy().into_owned())
    }
}

fn main() {}