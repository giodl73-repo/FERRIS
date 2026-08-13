//! Controlled system-native revision 1 fixture.

#[cfg(windows)]
mod platform {
    #[link(name = "kernel32")]
    unsafe extern "system" {
        #[link_name = "GetCurrentProcessId"]
        fn get_current_process_id() -> u32;
    }

    pub fn process_id() -> u32 {
        // The OS function takes no pointers and returns the caller process ID.
        unsafe { get_current_process_id() }
    }
}

#[cfg(unix)]
mod platform {
    unsafe extern "C" {
        fn getpid() -> i32;
    }

    pub fn process_id() -> u32 {
        // POSIX getpid takes no arguments and returns a positive process ID.
        unsafe { getpid() as u32 }
    }
}

pub use platform::process_id;

#[cfg(test)]
mod tests {
    use super::process_id;

    #[test]
    fn obtains_current_process_identity() {
        assert_ne!(process_id(), 0);
    }
}
