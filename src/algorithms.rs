//! Routing between operators.

use std::fmt::{Display, Formatter};

use lazy_static::lazy_static;

// use lazy_static::lazy_static;
use crate::{OperatorId, Preset};

/// The destination of an operator
///
/// An operator ID can be converted to an output:
///
/// ```
/// use synthahol_dx7::Output;
/// assert_eq!(Output::from(2), Some(Output::Op3));
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Output {
    // Operators
    Op1 = 0,
    Op2,
    Op3,
    Op4,
    Op5,
    Op6,

    /// Amplifier output
    Amplifier,
}

impl Output {
    pub fn is_operator(&self) -> bool {
        self != &Output::Amplifier
    }

    pub fn from(operator_id: OperatorId) -> Option<Output> {
        use Output::*;
        match operator_id {
            0 => Some(Op1),
            1 => Some(Op2),
            2 => Some(Op3),
            3 => Some(Op4),
            4 => Some(Op5),
            5 => Some(Op6),
            _ => None,
        }
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Output::*;
        let msg = match self {
            Op1 => "Operator 1",
            Op2 => "Operator 2",
            Op3 => "Operator 3",
            Op4 => "Operator 4",
            Op5 => "Operator 5",
            Op6 => "Operator 6",
            Amplifier => "Amplifier",
        };
        f.write_str(msg)
    }
}

pub type AlgorithmId = usize;

/// Routing between the operators and amplifier
pub struct Algorithm {
    routing_by_operator: [Vec<Output>; Preset::OPERATOR_COUNT],
}

impl Algorithm {
    pub const fn new(operators: [Vec<Output>; Preset::OPERATOR_COUNT]) -> Self {
        Self {
            routing_by_operator: operators,
        }
    }

    /// Returns `true` if the operator exists and is a carrier
    pub fn is_carrier(&self, operator_id: OperatorId) -> bool {
        let routing = self.routing(operator_id);
        routing == Some(&vec![Output::Amplifier])
    }

    /// Returns `true` if the operator exists and feeds back into itself.
    pub fn is_feedback(&self, operator_id: OperatorId) -> bool {
        let output = Output::from(operator_id);
        let routing = self.routing(operator_id);
        output
            .and_then(|output| routing.map(|routing| routing.contains(&output)))
            .unwrap_or_default()
    }

    pub fn routing(&self, operator_id: OperatorId) -> Option<&Vec<Output>> {
        self.routing_by_operator.get(operator_id as usize)
    }
}

lazy_static! {
static ref ALGORITHMS: [Algorithm; Algorithms::COUNT] = {
    use Output::*;
    [
        Algorithm::new( [
            vec![Amplifier],
            vec![Op1],
            vec![Amplifier],
            vec![Op3],
            vec![Op4],
            vec![Op5, Op6],
        ]), // 1
        Algorithm::new( [
            vec![Amplifier],
            vec![Op1, Op2],
            vec![Amplifier],
            vec![Op3],
            vec![Op4],
            vec![Op5],
        ]),
        Algorithm::new( [
            vec![Amplifier],
            vec![Op1],
            vec![Op2],
            vec![Amplifier],
            vec![Op4],
            vec![Op5, Op6],
        ]),
        Algorithm::new( [
            vec![Amplifier],
            vec![Op1],
            vec![Op2],
            vec![Amplifier],
            vec![Op4],
            vec![Op5, Amplifier],
        ]),
        Algorithm::new( [
            vec![Amplifier],
            vec![Op1],
            vec![Amplifier],
            vec![Op3],
            vec![Amplifier],
            vec![Op5, Op6],
        ]), // 5
Algorithm::new(        [
            vec![Amplifier],
            vec![Op1],
            vec![Amplifier],
            vec![Op3],
            vec![Amplifier],
            vec![Op5, Amplifier],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Op1],
            vec![Amplifier],
            vec![Op3],
            vec![Op3],
            vec![Op5, Op6],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Op1],
            vec![Amplifier],
            vec![Op3, Op4],
            vec![Op3],
            vec![Op5],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Op1, Op2],
            vec![Amplifier],
            vec![Op3],
            vec![Op3],
            vec![Op5],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Op1],
            vec![Op2, Op3],
            vec![Amplifier],
            vec![Op4],
            vec![Op4],
        ]), // 10
Algorithm::new(        [
            vec![Amplifier],
            vec![Op1],
            vec![Op2],
            vec![Amplifier],
            vec![Op4],
            vec![Op4, Op6],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Op1, Op2],
            vec![Amplifier],
            vec![Op3],
            vec![Op3],
            vec![Op3],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Op1],
            vec![Amplifier],
            vec![Op3],
            vec![Op3],
            vec![Op3, Op6],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Op1],
            vec![Amplifier],
            vec![Op3],
            vec![Op4],
            vec![Op4, Op6],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Op1, Op2],
            vec![Amplifier],
            vec![Op3],
            vec![Op4],
            vec![Op4],
        ]), // 15
Algorithm::new(        [
            vec![Amplifier],
            vec![Op1],
            vec![Op1],
            vec![Op3],
            vec![Op1],
            vec![Op5, Op6],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Op1, Op2],
            vec![Op1],
            vec![Op3],
            vec![Op1],
            vec![Op5],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Op1],
            vec![Op1, Op3],
            vec![Op1],
            vec![Op4],
            vec![Op5],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Op1],
            vec![Op2],
            vec![Amplifier],
            vec![Amplifier],
            vec![Op4, Op5, Op6],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Amplifier],
            vec![Op1, Op2, Op3],
            vec![Amplifier],
            vec![Op4],
            vec![Op4],
        ]), // 20
Algorithm::new(        [
            vec![Amplifier],
            vec![Amplifier],
            vec![Op1, Op2, Op3],
            vec![Amplifier],
            vec![Amplifier],
            vec![Op4, Op5],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Op1],
            vec![Amplifier],
            vec![Amplifier],
            vec![Amplifier],
            vec![Op3, Op4, Op5, Op6],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Amplifier],
            vec![Op2],
            vec![Amplifier],
            vec![Amplifier],
            vec![Op4, Op5, Op6],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Amplifier],
            vec![Amplifier],
            vec![Amplifier],
            vec![Amplifier],
            vec![Op3, Op4, Op5, Op6],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Amplifier],
            vec![Amplifier],
            vec![Amplifier],
            vec![Amplifier],
            vec![Op4, Op5, Op6],
        ]), // 25
Algorithm::new(        [
            vec![Amplifier],
            vec![Amplifier],
            vec![Op2],
            vec![Amplifier],
            vec![Op4],
            vec![Op4, Op6],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Amplifier],
            vec![Op2, Op3],
            vec![Amplifier],
            vec![Op4],
            vec![Op4],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Op1],
            vec![Amplifier],
            vec![Op3],
            vec![Op4, Op5],
            vec![Amplifier],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Amplifier],
            vec![Amplifier],
            vec![Op3],
            vec![Amplifier],
            vec![Op5, Op6],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Amplifier],
            vec![Amplifier],
            vec![Op3],
            vec![Op4, Op5],
            vec![Amplifier],
        ]), // 30
Algorithm::new(        [
            vec![Amplifier],
            vec![Amplifier],
            vec![Amplifier],
            vec![Amplifier],
            vec![Amplifier],
            vec![Op5, Op6],
        ]),
Algorithm::new(        [
            vec![Amplifier],
            vec![Amplifier],
            vec![Amplifier],
            vec![Amplifier],
            vec![Amplifier],
            vec![Amplifier, Op6],
        ]), // 32
    ]
};
}

pub struct Algorithms;

impl Algorithms {
    const COUNT: usize = 32;

    pub fn all() -> &'static [Algorithm; Algorithms::COUNT] {
        &ALGORITHMS
    }

    pub fn get(id: AlgorithmId) -> Option<&'static Algorithm> {
        ALGORITHMS.get(id)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn carrier() {
        let algorithm = Algorithms::get(0).unwrap();
        assert!(algorithm.is_carrier(0));
        assert!(!algorithm.is_carrier(1));
        assert!(algorithm.is_carrier(2));
        assert!(!algorithm.is_carrier(3));
        assert!(!algorithm.is_carrier(5));
        assert!(!algorithm.is_carrier(6));
    }

    #[test]
    fn feedback() {
        let algorithm = Algorithms::get(0).unwrap();
        assert!(!algorithm.is_feedback(0));
        assert!(algorithm.is_feedback(5));
    }

    #[test]
    fn routing() {
        // Every operator must have an output and not have duplicates
        for (algorithm_index, algorithm) in Algorithms::all().iter().enumerate() {
            for operator_id in 0..ALGORITHMS[0].routing_by_operator.len() {
                let routing = algorithm
                    .routing(operator_id as OperatorId)
                    .expect("every operator has a routing");
                let unique: HashSet<&Output> = routing.into_iter().collect();
                assert_eq!(
                    routing.len(),
                    unique.len(),
                    "Algorithm index {algorithm_index} contains duplicates"
                );
                assert!(
                    !routing.is_empty(),
                    "Algorithm index {algorithm_index} does not have an output for operator ID {operator_id}"
                );
            }
        }
    }
}
