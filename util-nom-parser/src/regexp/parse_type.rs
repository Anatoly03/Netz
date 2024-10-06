//! This module contains the production type definition and implementations.
//! A production type is the interpreted type of a field in a 

use super::ProductionType;

#[derive(Debug, PartialEq)]
pub struct ParseType {
    prod_type: ProductionType,
}
