//! This module contains the production type definition and implementations.
//! A production type is the interpreted type of a field in a 

use super::{ProductionType, Dimension};

#[derive(Debug, PartialEq)]
pub struct ParseType {
    prod_type: ProductionType,
    dimension: Dimension,
}

impl ParseType {
    pub fn new(dimension: Dimension, prod_type: ProductionType) -> Self {
        Self { prod_type, dimension }
    }
}

impl ToString for ParseType {
    fn to_string(&self) -> String {
        match self.dimension {
            Dimension::Scalar => self.prod_type.to_string(),
            Dimension::Option => ["Option<", self.prod_type.to_string().as_str(), ">"].concat(),
            Dimension::Many => ["Vec<", self.prod_type.to_string().as_str(), ">"].concat(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coverage() {
        assert_eq!(ParseType::new(Dimension::Scalar, ProductionType::Bool).to_string(), "bool");
        assert_eq!(ParseType::new(Dimension::Option, ProductionType::Int(false, 1)).to_string(), "Option<u8>");
        assert_eq!(ParseType::new(Dimension::Many, ProductionType::Float(4)).to_string(), "Vec<f32>");
    }
}
