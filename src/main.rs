#![no_main]
#![no_std]
#![windows_subsystem = "console"]

use core::ffi::c_void;
use core::ptr::{null, null_mut};

use windows_sys::Win32::System::Console::GetStdHandle;
use windows_sys::Win32::System::Console::WriteConsoleA;
use windows_sys::Win32::System::Console::STD_OUTPUT_HANDLE;
use windows_sys::Win32::System::Threading::ExitProcess;

#[unsafe(no_mangle)]
fn main() -> ! {
    let message = "Helloo World";

    unsafe {

        let console = GetStdHandle(STD_OUTPUT_HANDLE);

        WriteConsoleA(
            console,
            message.as_ptr() as *const c_void,
            message.len() as u32,
            null_mut(),
            null(),
        );

        ExitProcess(0)
    }
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {}
}