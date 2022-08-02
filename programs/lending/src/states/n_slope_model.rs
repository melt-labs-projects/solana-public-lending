use anchor_lang::prelude::*;

use crate::errors::LendingError;
use crate::instructions::utils;

#[account]
pub struct NSlopeModel {

    pub slopes: Vec<[u64; 3]>

}

impl NSlopeModel {

    pub fn space(slopes: u64) -> usize {
        return 8 + 4 + (slopes as usize * 24);
    }

    pub fn validate(&self) -> Result<()> {
        for i in 0..(self.slopes.len() - 1) {
            let current = self.slopes[i];
            let next = self.slopes[i + 1];
            let x_gap = next[0] - current[0];
            assert_eq!(current[1] + utils::safe_fraction_mul(x_gap, current[2], utils::ONE)?, next[1]);
        }
        Ok(())
    }

    pub fn interest_rate(&self, utilization: u64) -> Result<u64> {
        for slope in self.slopes.iter().rev() {
            if utilization >= slope[0] {
                return Ok(slope[1] + utils::safe_fraction_mul(utilization - slope[0], slope[2], utils::ONE)?);
            }
        }
        err!(LendingError::NumericalError)
    }

}