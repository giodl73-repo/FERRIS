//! Controlled system-native revision 2 fixture.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelatedOwnerKind {
    ParentProcess,
    Thread,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeIdentity {
    pub process_id: u32,
    pub related_owner: u32,
    pub related_kind: RelatedOwnerKind,
}

#[cfg(windows)]
mod platform {
    use super::{NativeIdentity, RelatedOwnerKind};

    #[link(name = "kernel32")]
    unsafe extern "system" {
        #[link_name = "GetCurrentProcessId"]
        fn get_current_process_id() -> u32;
        #[link_name = "GetCurrentThreadId"]
        fn get_current_thread_id() -> u32;
    }

    pub fn native_identity() -> NativeIdentity {
        // Both OS functions take no pointers and return caller-owned IDs.
        unsafe {
            NativeIdentity {
                process_id: get_current_process_id(),
                related_owner: get_current_thread_id(),
                related_kind: RelatedOwnerKind::Thread,
            }
        }
    }
}

#[cfg(unix)]
mod platform {
    use super::{NativeIdentity, RelatedOwnerKind};

    unsafe extern "C" {
        fn getpid() -> i32;
        fn getppid() -> i32;
    }

    pub fn native_identity() -> NativeIdentity {
        // POSIX process identity functions take no arguments and return IDs.
        unsafe {
            NativeIdentity {
                process_id: getpid() as u32,
                related_owner: getppid() as u32,
                related_kind: RelatedOwnerKind::ParentProcess,
            }
        }
    }
}

pub use platform::native_identity;

#[cfg(test)]
mod tests {
    use super::{RelatedOwnerKind, native_identity};

    #[test]
    fn obtains_process_and_related_owner_identity() {
        let identity = native_identity();
        assert_ne!(identity.process_id, 0);
        assert_ne!(identity.related_owner, 0);
        assert!(matches!(
            identity.related_kind,
            RelatedOwnerKind::ParentProcess | RelatedOwnerKind::Thread
        ));
    }
}
