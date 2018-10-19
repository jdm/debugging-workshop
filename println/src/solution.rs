//! Exercise for using large values with debug printing.
//!
//! This program terminates after printing "error: buffer is not readonly" or
//! "ok: buffer is readonly". It also "helpfully" prints many instances of the
//! buffer after each modification, which makes it difficult to understand
//! what is happening.
//!
//! First make the debug printing less verbose by uncommenting and completing
//! the commented TruncatedDebug implementation. Then try adding println!
//! calls to help figure out why the buffer can still be modified after
//! `make_read_only` is called.

use std::fmt::{self, Debug};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

const BLOCK_SIZE: usize = 32;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Permission {
    ReadOnly,
    ReadWrite,
}

#[derive(Debug)]
struct MemoryBuffer {
  buffer: TruncatedDebug<[u16; BLOCK_SIZE]>,
  permissions: TruncatedDebug<HashMap<usize, Permission>>,
}

impl MemoryBuffer {
    /// Create a new memory buffer.
    fn new() -> MemoryBuffer {
        MemoryBuffer {
            buffer: TruncatedDebug([0; BLOCK_SIZE]),
            permissions: TruncatedDebug(HashMap::new()),
        }
    }

    /// Mark every memory slot as being read-only.
    fn make_read_only(&mut self) {
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
    fn write_memory_at(&mut self, idx: usize, val: u16) -> Result<(), ()> {
        let idx = idx % self.buffer.len();
        if self.permission_at(idx) != Permission::ReadOnly {
            self.buffer[idx] = val;
            Ok(())
        } else {
            Err(())
        }
    }

    /// The size of this memory buffer.
    fn size(&self) -> usize {
        BLOCK_SIZE
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
        println!("{:?}", buffer);
    }

    // Mark every memory slot as read-only.
    buffer.make_read_only();
    println!("{:?}", buffer);

    // Try to overwrite each memory slot with 0.
    // This is supposed to silently fail, since the buffer
    // should be read-only.
    for i in 0..buffer.size() {
        let _ = buffer.write_memory_at(i, 0);
        println!("{:?}", buffer);
    }

    // Look for any evidence that any writes succeeded.
    if buffer.buffer.iter().any(|v| *v == 0) {
        println!("error: buffer is not readonly");
    } else {
        println!("ok: buffer is readonly");
    }
}
