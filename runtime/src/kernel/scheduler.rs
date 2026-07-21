//! Global FIFO run queue for normal kernel threads.
//!
//! Per-hart idle threads are kept outside this queue in [`PerCore`].

use alloc::boxed::Box;
use alloc::collections::VecDeque;

use crate::arch::interrupt::InterruptGuard;
use crate::kernel::per_core::PerCore;
use crate::kernel::sync::SpinLock;
use crate::kernel::thread::{CurrentThread, Thread};

pub static SCHEDULER: Scheduler = Scheduler::empty();

/// FIFO scheduler for globally runnable normal kernel threads.
pub struct Scheduler {
    threads: SpinLock<VecDeque<Box<Thread>>>,
}

impl Scheduler {
    const fn empty() -> Self {
        Self {
            threads: SpinLock::new(VecDeque::new()),
        }
    }

    pub fn push(&self, thread: Box<Thread>) {
        self.threads.lock().push_back(thread);
    }

    /// Switch to the next globally ready thread or this hart's idle thread.
    ///
    /// When the global queue is empty, a non-idle caller switches to its parked
    /// local idle context. The switch-out path then places a still-runnable
    /// normal thread on the global queue, where another hart may claim it.
    /// If the local idle thread is already running, there is no context to take
    /// from the idle slot and this function returns `false` without switching.
    ///
    /// Returns `false` only when the current idle thread has no work to run. If
    /// a switch occurs, callers that are later rescheduled observe `true` when
    /// they resume; exited callers never return from the switch.
    pub fn run_next(&self) -> bool {
        let _guard = InterruptGuard::new();
        let next = match self.threads.lock().pop_front() {
            Some(next) => next,
            None => match PerCore::take_idle() {
                Some(idle) => idle,
                None => return false, // The local idle thread is already current.
            },
        };
        CurrentThread::switch_to(next);
        true
    }
}
