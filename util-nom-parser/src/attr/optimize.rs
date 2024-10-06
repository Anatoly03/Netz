use super::Rule;

use crate::regexp::Dimension;

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

            // A non-scalar nested in a repetition is itself.
            // Assert: `Repetition(Option(X)) = Repetition(X)`
            // Assert: `Repetition(Repetition(X)) = Repetition(X)`
            Rule::Repetition(s) => match *s {
                Rule::Repetition(k) | Rule::Option(k) => Rule::Repetition(Box::new(k.optimize())),
                // Assert: `Repetition(Scope([X])) = Repetition(X)`
                // ^^^^^^^ Will be done automatically with optimize
                // Rule::Scope(vec) if vec.len() == 1 => {
                //     Rule::Repetition(Box::new(vec.into_iter().next().unwrap().optimize()))
                // }
                k => Rule::Repetition(Box::new(k.optimize())),
            },

            // Rule::Whitespace => todo!(),
            // Rule::Keyword(_) => todo!(),
            // Rule::Identifier(_) => todo!(),
            // Rule::TypeReference(_) => todo!(),
            // Rule::Repetition(rule) => todo!(),
            // Rule::Branch(vec) => todo!(),
            _ => self,
        }
    }
}
