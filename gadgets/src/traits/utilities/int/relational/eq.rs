// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::utilities::{boolean::Boolean, eq::EvaluateEqGadget, int::*, number::Number};
use snarkvm_fields::PrimeField;
use snarkvm_r1cs::{errors::SynthesisError, ConstraintSystem};

macro_rules! eq_gadget_impl {
    ($($gadget: ident)*) => ($(
        impl<F: PrimeField> EvaluateEqGadget<F> for $gadget {
            fn evaluate_equal<CS: ConstraintSystem<F>>(
                &self,
                mut cs: CS,
                other: &Self
            ) -> Result<Boolean, SynthesisError> {
                let mut result = Boolean::constant(true);
                for (i, (a, b)) in self.bits.iter().zip(&other.bits).enumerate() {
                    let equal = a.evaluate_equal(
                        &mut cs.ns(|| format!("{} evaluate equality for {}-th bit", <$gadget as Number>::SIZE, i)),
                        b,
                    )?;

                    result = Boolean::and(
                        &mut cs.ns(|| format!("{} and result for {}-th bit", <$gadget as Number>::SIZE, i)),
                        &equal,
                        &result,
                    )?;
                }

                Ok(result)
            }
        }

        impl PartialEq for $gadget {
            fn eq(&self, other: &Self) -> bool {
                !self.value.is_none() && self.value == other.value
            }
        }

        impl Eq for $gadget {}
    )*)
}

eq_gadget_impl!(Int8 Int16 Int32 Int64 Int128);
