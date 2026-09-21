// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com>

//! Generic trap dispatcher with priority, chain, and optional matcher support.

use alloc::vec::Vec;
use core::cmp::Ordering;
use alloc::boxed::Box;

/// Trap handler priority
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrapPriority {
    High = 0,
    Normal = 1,
    Low = 2,
}

/// Trap handler result
pub enum TrapResult {
    Handled,   // Stop chain
    Continue,  // Continue to next handler
}

/// Trap context (example, expand as needed)
pub struct TrapContext<'a> {
    pub cause: usize,
    pub regs: &'a [usize],
    // ... add more fields as needed ...
}

/// Matcher type: returns true if this handler should run for the given context
pub type TrapMatcher = dyn Fn(&TrapContext) -> bool + Send + Sync;
/// Handler type
pub type TrapHandler = dyn Fn(&TrapContext) -> TrapResult + Send + Sync;

/// Trap handler entry (with priority, optional matcher, and handler)
pub struct TrapHandlerEntry {
    pub priority: TrapPriority,
    pub matcher: Option<Box<TrapMatcher>>,
    pub handler: Box<TrapHandler>,
}

/// Trap dispatcher: manages a chain of handlers with priority and optional matcher
pub struct TrapDispatcher {
    handlers: Vec<TrapHandlerEntry>,
}

impl TrapDispatcher {
    pub fn new() -> Self {
        TrapDispatcher { handlers: Vec::new() }
    }

    /// Register a handler with priority and optional matcher (closure or fn)
    pub fn register<M, H>(&mut self, priority: TrapPriority, matcher: Option<M>, handler: H)
    where
        M: Fn(&TrapContext) -> bool + Send + Sync + 'static,
        H: Fn(&TrapContext) -> TrapResult + Send + Sync + 'static,
    {
        self.handlers.push(TrapHandlerEntry {
            priority,
            matcher: matcher.map(|m| Box::new(m) as Box<TrapMatcher>),
            handler: Box::new(handler) as Box<TrapHandler>,
        });
        // Keep handlers sorted by priority (lower is higher priority)
        self.handlers.sort_by(|a, b| a.priority.cmp(&b.priority));
    }

    /// Dispatch a trap: call handlers in priority order, stop if Handled
    pub fn dispatch(&self, ctx: &TrapContext) -> TrapResult {
        for entry in &self.handlers {
            let matched = match &entry.matcher {
                Some(matcher) => matcher(ctx),
                None => true, // No matcher means always match
            };
            if matched {
                match (entry.handler)(ctx) {
                    TrapResult::Handled => return TrapResult::Handled,
                    TrapResult::Continue => continue,
                }
            }
        }
        TrapResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn always(_ctx: &TrapContext) -> bool { true }
    fn only_cause_1(ctx: &TrapContext) -> bool { ctx.cause == 1 }
    fn handler1(_ctx: &TrapContext) -> TrapResult { TrapResult::Continue }
    fn handler2(_ctx: &TrapContext) -> TrapResult { TrapResult::Handled }

    #[test_case]
    fn test_priority_and_matcher() -> Result<(), &'static str> {
        let mut disp = TrapDispatcher::new();
        let mut log = alloc::vec::Vec::new();
        // High priority, only cause==1
        disp.register(TrapPriority::High, Some(only_cause_1), |_ctx| { log.push(1); TrapResult::Continue });
        // Low priority, always
        disp.register(TrapPriority::Low, None, |_ctx| { log.push(2); TrapResult::Handled });
        // Normal priority, always
        disp.register(TrapPriority::Normal, Some(always), |_ctx| { log.push(3); TrapResult::Continue });
        let ctx = TrapContext { cause: 1, regs: &[0; 32] };
        disp.dispatch(&ctx);
        // Should call: High (1), Normal (3), Low (2) in order, stop at Handled
        assert_eq!(log, [1, 3, 2]);
        Ok(())
    }
    #[test_case]
    fn test_matcher_none() -> Result<(), &'static str> {
        let mut disp = TrapDispatcher::new();
        disp.register(TrapPriority::Normal, None, |_ctx| TrapResult::Handled);
        let ctx = TrapContext { cause: 0, regs: &[0; 32] };
        assert!(matches!(disp.dispatch(&ctx), TrapResult::Handled));
        Ok(())
    }
}
