// Copyright 2023 
// released under BSD 3-Clause License
// author: Gary Hu

mod smt;
mod types;

pub use crate::sim::interpreter::Simulator;
pub use smt::{
    check_assuming, check_assuming_end, get_smt_value, ModelCheckResult, PropertyCheckResult, SmtModelChecker,
    SmtModelCheckerOptions, SmtSolverCmd, TransitionSystemEncoding, UnrollSmtEncoding,
    BITWUZLA_CMD, YICES2_CMD,
};
pub use types::{parse_big_uint_from_bit_string, Witness, WitnessArray, WitnessValue};