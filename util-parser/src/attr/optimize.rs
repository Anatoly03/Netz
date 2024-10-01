use super::{Dimension, Rule};

impl Rule {
    /// Returns the dimension (scalar, optional or many) of a
    /// rule.
    fn dimension(&self) -> Dimension {
        match self {
            Rule::Option(rule) => Dimension::Option,
            Rule::Repetition(rule) => Dimension::Many,
            _ => Dimension::Scalar,
        }
    }

    /// Optimize a rule with predefined optimization techniques.
    pub fn optimize(self) -> Rule {
        match self {
            // A scope of one element is the optimized element itself.
            // Assert: `Scope([X]) = X`
            Rule::Scope(vec) if vec.len() == 1 => vec.into_iter().next().unwrap().optimize(),
            // A non-scalar nested in an option is itself.
            // Assert: `Option(Option(X)) = Option(X)`
            // Assert: `Option(Repetition(X)) = Repetition(X)`
            Rule::Option(s) if s.dimension() != Dimension::Scalar => s.optimize(),

            // Rule::Repetition(s) if s.dimension() == Dimension::Many => s.optimize(),

            // Rule::Whitespace => todo!(),
            // Rule::Keyword(_) => todo!(), 
            // Rule::Identifier(_) => todo!(),
            // Rule::TypeReference(_) => todo!(),
            // Rule::Option(rule) => todo!(),
            // Rule::Repetition(rule) => todo!(),
            // Rule::Branch(vec) => todo!(),
            _ => self,
        }
    }
}
