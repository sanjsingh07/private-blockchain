#![allow(clippy::integer_arithmetic)]
/// There are 10^9 carats in one SOL
pub const LAMPORTS_PER_GEMA: u64 = 1_000_000_000;

/// Approximately convert fractional native tokens (carats) into native tokens (GEMA)
pub fn carats_to_gema(carats: u64) -> f64 {
    carats as f64 / LAMPORTS_PER_GEMA as f64
}

/// Approximately convert native tokens (GEMA) into fractional native tokens (carats)
pub fn gema_to_carats(nub: f64) -> u64 {
    (nub * LAMPORTS_PER_GEMA as f64) as u64
}

use std::fmt::{Debug, Display, Formatter, Result};
pub struct Gema(pub u64);

impl Gema {
    fn write_in_gema(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "GM {}.{:09}",
            self.0 / LAMPORTS_PER_GEMA,
            self.0 % LAMPORTS_PER_GEMA
        )
    }
}

impl Display for Gema {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_gema(f)
    }
}

impl Debug for Gema {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_gema(f)
    }
}
