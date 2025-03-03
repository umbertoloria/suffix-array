use crate::suffix_array::monitor::ExecutionOutcome;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExecutionOutcomeFileFormat {
    compares_using_rules: usize,
    compares_using_strcmp: usize,
    compares_using_one_cf: usize,
    compares_using_two_cf: usize,
}
impl ExecutionOutcomeFileFormat {
    pub fn new(execution_outcome: &ExecutionOutcome) -> Self {
        Self {
            compares_using_rules: execution_outcome.compares_using_rules,
            compares_using_strcmp: execution_outcome.compares_using_strcmp,
            compares_using_one_cf: execution_outcome.compares_with_one_cf,
            compares_using_two_cf: execution_outcome.compares_with_two_cfs,
        }
    }
}
