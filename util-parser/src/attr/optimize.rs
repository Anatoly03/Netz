use super::Rule;

impl Rule {
    /// Optimize a rule with predefined optimization techniques.
    pub fn optimize(self) -> Rule {
        match self {
            // A scope of one element is the optimized element itself.
            Rule::Scope(vec) if vec.len() == 1 => vec
                .into_iter()
                .next()
                .unwrap()
                .optimize(),
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
