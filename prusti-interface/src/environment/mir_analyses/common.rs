// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use rustc::mir;
use std::collections::HashMap;

pub struct AnalysisResult<T: Default> {
    /// The state before the basic block.
    pub(super) before_block: HashMap<mir::BasicBlock, T>,
    /// The state after the statement.
    pub(super) after_statement: HashMap<mir::Location, T>,
    /// The state at return.
    pub(super) at_return: T,
}

impl<T: Default> AnalysisResult<T> {
    pub fn new() -> Self {
        Self {
            before_block: HashMap::new(),
            after_statement: HashMap::new(),
            at_return: T::default(),
        }
    }
    /// Get the initialization set before the first statement of the
    /// basic block.
    pub fn get_before_block(&self, bb: mir::BasicBlock) -> &T {
        self.before_block
            .get(&bb)
            .expect(&format!("Missing initialization info for block {:?}", bb))
    }
    /// Get the initialization set after the statement.
    /// If `location.statement_index` is equal to the number of statements,
    /// returns the initialization set after the terminator.
    pub fn get_after_statement(&self, location: mir::Location) -> &T {
        self.after_statement.get(&location).expect(&format!(
            "Missing initialization info for location {:?}",
            location
        ))
    }
    /// Get the initilization set at return.
    pub fn get_at_return(&self) -> &T {
        &self.at_return
    }
}

/// A work item used in the fixpoint computation's work queue.
pub(super) enum WorkItem {
    /// Need to apply the effects of the statement.
    ApplyStatementEffects(mir::Location),
    /// Need to apply the effects of the terminator.
    ApplyTerminatorEffects(mir::BasicBlock),
    /// Need to merge the incoming effects of multiple edges.
    MergeEffects(mir::BasicBlock),
}
