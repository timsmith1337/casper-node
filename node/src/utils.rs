//! Various functions that are not limited to a particular module, but are too small to warrant
//! being factored out into standalone crates.

pub(crate) mod gossip_table;
mod round_robin;

use std::{
    cell::RefCell,
    fmt::{self, Display, Formatter},
};

use lazy_static::lazy_static;
use libc::{c_long, sysconf, _SC_PAGESIZE};

pub(crate) use gossip_table::{GossipAction, GossipTable};
pub(crate) use round_robin::WeightedRoundRobin;

/// Sensible default for many if not all systems.
const DEFAULT_PAGE_SIZE: usize = 4096;

lazy_static! {
    /// OS page size.
    pub static ref OS_PAGE_SIZE: usize = {
        // https://www.gnu.org/software/libc/manual/html_node/Sysconf.html
        let value: c_long = unsafe { sysconf(_SC_PAGESIZE) };
        if value < 0 {
            DEFAULT_PAGE_SIZE
        } else {
            value as usize
        }
    };
}

/// Moves a value to the heap and then forgets about, leaving only a static reference behind.
#[inline]
pub(crate) fn leak<T>(value: T) -> &'static T {
    Box::leak(Box::new(value))
}

/// A display-helper that shows iterators display joined by ",".
#[derive(Debug)]
pub(crate) struct DisplayIter<T>(RefCell<Option<T>>);

impl<T> DisplayIter<T> {
    pub(crate) fn new(item: T) -> Self {
        DisplayIter(RefCell::new(Some(item)))
    }
}

impl<I, T> Display for DisplayIter<I>
where
    I: IntoIterator<Item = T>,
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(src) = self.0.borrow_mut().take() {
            let mut first = true;
            for item in src.into_iter().take(f.width().unwrap_or(usize::MAX)) {
                if first {
                    first = false;
                    write!(f, "{}", item)?;
                } else {
                    write!(f, ", {}", item)?;
                }
            }

            Ok(())
        } else {
            write!(f, "DisplayIter:GONE")
        }
    }
}