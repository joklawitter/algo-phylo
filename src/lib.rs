pub mod model;
pub mod parser;

use crate::model::tree::{LeafLabelMap, Tree};
use crate::parser::byte_parser::ByteParser;
use crate::parser::nexus;
use std::error::Error;

pub fn parse_nexus_file(path: &str) -> Result<(Vec<Tree>, LeafLabelMap), Box<dyn Error>> {
    // Read entire file into memory
    let contents = std::fs::read(path)?;

    // Create ByteParser
    let mut parser = ByteParser::from_bytes(&contents);

    // Parse NEXUS content
    let (trees, map) = nexus::parse_nexus(&mut parser)?;

    Ok((trees, map))
}