//! Exercise for using logging statements appropriately.
//!
//! This exercise contains many println! statements which
//! should not be released as part of making the code public.
//! It would be better to add useful logging statements with
//! appropriate severity levels to the MemoryBuffer API instead.
//!
//! 1) Integrate the `log` and `env_logger` crates
//! 2) Choose where new logging calls make sense
//! 3) Verify that RUST_LOG works as expected

use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Permission {
    ReadOnly,
    ReadWrite,
}

use buffer::MemoryBuffer;

mod buffer {
    use std::collections::HashMap;
    use super::{TruncatedDebug, Permission};

    const BLOCK_SIZE: usize = 32;

    #[derive(Debug)]
    pub struct MemoryBuffer {
        buffer: TruncatedDebug<[u16; BLOCK_SIZE]>,
        permissions: TruncatedDebug<HashMap<usize, Permission>>,
    }

    impl MemoryBuffer {
        /// Create a new memory buffer.
        pub fn new() -> MemoryBuffer {
            MemoryBuffer {
                buffer: TruncatedDebug([0; BLOCK_SIZE]),
                permissions: TruncatedDebug(HashMap::new()),
            }
        }

        /// Mark every memory slot as being read-only.
        pub fn make_read_only(&mut self) {
            self.change_permissions(0, BLOCK_SIZE, Permission::ReadOnly);
        }

        /// Mark all of the memory slots between `start` and `end` as having the given permission.
        fn change_permissions(&mut self, start: usize, end: usize, perm: Permission) {
            for idx in start..end {
                self.permissions.insert(idx, perm);            
            }
        }

        /// Return the permission for the given slot, defaulting to ReadWrite if not explicit
        /// permission has been set.
        fn permission_at(&self, idx: usize) -> Permission {
            *self.permissions.get(&idx).unwrap_or(&Permission::ReadWrite)
        }

        /// Write a value to the given slot. Returns Err if the slot does not have ReadWrite
        /// permissions attached to it.
        pub fn write_memory_at(&mut self, idx: usize, val: u16) -> Result<(), ()> {
            if idx > self.buffer.len() {
                return Err(());
            }
            if self.permission_at(idx) != Permission::ReadOnly {
                self.buffer[idx] = val;
                Ok(())
            } else {
                Err(())
            }
        }

        /// Write a value to the given slot. Returns Err if the slot does not have ReadWrite
        /// permissions attached to it.
        pub fn read_memory_at(&mut self, idx: usize) -> Result<u16, ()> {
            if idx > self.buffer.len() {
                Err(())
            } else {
                Ok(self.buffer[idx])
            }
        }

        /// The size of this memory buffer.
        pub fn size(&self) -> usize {
            BLOCK_SIZE
        }
    }
}

struct TruncatedDebug<T: Debug>(T);

impl<T: Debug> Deref for TruncatedDebug<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Debug> DerefMut for TruncatedDebug<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Debug> Debug for TruncatedDebug<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut truncated = format!("{:?}", self.0);
        let last = truncated.chars().last();
        truncated.truncate(20);
        truncated.push_str("...");
        if let Some(last) = last {
            truncated.push(last);
        }
        write!(f, "{}", truncated)
    }
}

fn main() {
    // Create a memory buffer representation.
    let mut buffer = MemoryBuffer::new();

    // Initialize each memory slot with a unique value.
    for i in 0..buffer.size() {
        let _ = buffer.write_memory_at(i, (i + 1) as u16);
    }

    // Mark every memory slot as read-only.
    buffer.make_read_only();

    // Try to overwrite each memory slot with 0.
    // This is supposed to silently fail, since the buffer
    // should be read-only.
    for i in 0..buffer.size() {
        let _ = buffer.write_memory_at(i, 0);
    }

    // Look for any evidence that any writes succeeded.
    for i in 0..buffer.size() {
        if buffer.read_memory_at(i) == Ok(0) {
            // ????
        }
    }
}
