use crate::rational_expression::RationalExpression;
use itertools::Itertools;
use primefield::FieldElement;
use std::{fmt, prelude::v1::*};

#[derive(Clone, Debug)]
pub enum Error {
    InvalidTraceLength,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match *self {
            InvalidTraceLength => write!(f, "Invalid trace length (must be power of two)"),
        }
    }
}

// TODO: Merge with ProofParams
#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Constraints {
    channel_seed:   Vec<u8>,
    trace_nrows:    usize,
    trace_ncolumns: usize,
    expressions:    Vec<RationalExpression>,
}

impl Constraints {
    pub fn from_expressions(
        (trace_nrows, trace_ncolumns): (usize, usize),
        channel_seed: Vec<u8>,
        expressions: Vec<RationalExpression>,
    ) -> Result<Self, Error> {
        let _ = FieldElement::root(trace_nrows).ok_or(Error::InvalidTraceLength)?;
        // TODO: Validate expressions
        Ok(Self {
            channel_seed,
            trace_nrows,
            trace_ncolumns,
            expressions,
        })
    }

    pub fn channel_seed(&self) -> &[u8] {
        &self.channel_seed
    }

    pub fn trace_nrows(&self) -> usize {
        self.trace_nrows
    }

    pub fn trace_ncolumns(&self) -> usize {
        self.trace_ncolumns
    }

    pub fn len(&self) -> usize {
        self.expressions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.expressions().is_empty()
    }

    pub fn expressions(&self) -> &[RationalExpression] {
        &self.expressions
    }

    pub fn degree(&self) -> usize {
        self.expressions
            .iter()
            .map(|c| {
                let (numerator_degree, denominator_degree) = c.trace_degree();
                numerator_degree - denominator_degree
            })
            .max()
            .expect("no constraints")
    }

    pub(crate) fn combine(&self, constraint_coefficients: &[FieldElement]) -> RationalExpression {
        use RationalExpression::*;
        assert_eq!(2 * self.len(), constraint_coefficients.len());
        let target_degree = self.degree() * self.trace_nrows() - 1;

        self.expressions
            .iter()
            .zip(constraint_coefficients.iter().tuples())
            .map(
                |(constraint, (coefficient_low, coefficient_high))| -> RationalExpression {
                    let (num, den) = constraint.degree(self.trace_nrows() - 1);
                    let adjustment_degree = target_degree + den - num;
                    let adjustment = Constant(coefficient_low.clone())
                        + Constant(coefficient_high.clone()) * X.pow(adjustment_degree);
                    adjustment * constraint.clone()
                },
            )
            .sum()
    }
}
