//! # halo2_proofs

#![cfg_attr(docsrs, feature(doc_cfg))]
// Build without warnings on stable 1.51 and later.
#![allow(unknown_lints)]
// Disable old lint warnings until our MSRV is at least 1.51.
#![allow(renamed_and_removed_lints)]
// Use the old lint name to build without warnings until our MSRV is at least 1.51.
#![allow(clippy::unknown_clippy_lints)]
// The actual lints we want to disable.
#![allow(
    clippy::op_ref,
    clippy::assign_op_pattern,
    clippy::too_many_arguments,
    clippy::suspicious_arithmetic_impl,
    clippy::many_single_char_names,
    clippy::same_item_push,
    clippy::upper_case_acronyms
)]
#![deny(broken_intra_doc_links)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
// Remove this once we update pasta_curves
#![allow(unused_imports)]
#![allow(clippy::derive_partial_eq_without_eq)]

pub mod arithmetic;
pub mod circuit;
use std::time::{Duration, Instant};

pub use halo2curves;
mod multicore;
pub mod plonk;
pub mod poly;
pub mod transcript;

pub mod dev;
mod helpers;
pub use helpers::SerdeFormat;

#[derive(Clone, Debug)]
/// A helper struct for configuring profiling
pub struct Timer {
    name: String,
    start: Instant,
    current: Instant,
    log: Vec<(String, Duration)>,
}

impl Timer {
    /// Creates a new timer
    pub fn new(name: &str) -> Timer {
        Timer {
            name: name.to_string(),
            start: Instant::now(),
            current: Instant::now(),
            log: vec![],
        }
    }

    /// Record the current time stamp
    pub fn checkpoint(&mut self, name: &str) {
        let elapsed = self.current.elapsed();
        self.log.push((name.to_string(), elapsed));
        self.current = Instant::now();
    }

    /// Print the profiling results
    pub fn finalize(&self) {
        let total = self.start.elapsed();
        println!("\n----------------------------------------");
        println!(
            "{} Total: {}.{:03}",
            self.name,
            total.as_secs(),
            total.subsec_millis()
        );
        for (name, elapsed) in &self.log {
            println!(
                "  {}: {}.{:03} ({:.1}%)",
                name,
                elapsed.as_secs(),
                elapsed.subsec_millis(),
                100.0 * elapsed.as_secs_f64() / total.as_secs_f64()
            );
        }
        println!("----------------------------------------\n");
    }
}
