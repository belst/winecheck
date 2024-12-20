#![no_std]

#[cfg(target_os = "windows")]
use windows::core::s;
#[cfg(target_os = "windows")]
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};

#[cfg(not(any(target_os = "windows", doc)))]
pub fn is_wine() -> bool {
    return false;
}

#[cfg(all(target_os = "windows", not(doc)))]
pub fn is_wine() -> bool {
    unsafe {
        let Ok(handle) = GetModuleHandleA(s!("ntdll.dll")) else {
            // Cannot say for sure
            return false;
        };
        return GetProcAddress(handle, s!("wine_get_version")).is_some();
    }
}

/// Returns `true` if the current process is running under wine.
///
/// # Platform-specific behavior
/// - On Windows, this function checks if the current process is running under wine by checking if
///   the `wine_get_version` function is available in the `ntdll.dll` library.
/// - On other platforms, this function always returns `false`.
///
/// # Examples
/// ```no_run
/// use winecheck::is_wine;
///
/// assert!(is_wine());
/// ```
#[cfg(doc)]
pub fn is_wine() -> bool {
    unimplemented!("This function only exists for documentation purposes");
}
