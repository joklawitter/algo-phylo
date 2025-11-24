use crate::model::tree::{LabelIndex, LeafLabelMap, Tree, TreeIndex};
use crate::model::vertex::BranchLength;
use crate::parser::byte_parser::ByteParser;
use crate::parser::parsing_error::ParsingError;
use std::collections::HashMap;
use std::fmt;

/// Newick label delimiters: parentheses, comma, colon, semicolon, whitespace
const NEWICK_LABEL_DELIMITERS: &[u8] = b"(),:; \t\n\r";

pub fn parse_newick(parser: &mut ByteParser, num_leaves: usize) -> Result<(Tree, LeafLabelMap), ParsingError> {
    let mut tree = Tree::new(num_leaves);
    let mut leaf_label_map = LeafLabelMap::new(num_leaves);
    let mut resolver = LabelResolver::new_label_to_index(&mut leaf_label_map);

    parser.skip_whitespace();
    parse_root(parser, &mut tree, &mut resolver)?;
    Ok((tree, leaf_label_map))
}

pub fn parse_newick_with_resolver(parser: &mut ByteParser, num_leaves: usize, resolver: &mut LabelResolver) -> Result<Tree, ParsingError> {
    let mut tree = Tree::new(num_leaves);

    parser.skip_whitespace();
    parse_root(parser, &mut tree, resolver)?;

    Ok(tree)
}

fn parse_root(parser: &mut ByteParser, tree: &mut Tree, resolver: &mut LabelResolver) -> Result<(), ParsingError> {
    let (left_index, right_index) = parser_children(parser, tree, resolver)?;

    // Root may have an optional branch length (which we ignore for now)
    if parser.peek() == Some(b':') {
        let _ = parse_branch_length(parser)?;
    }

    // Consume the terminating semicolon
    parser.skip_whitespace();
    if !parser.consume_if(b';') {
        return Err(ParsingError::invalid_newick_string(
            parser,
            format!("Expected ';' at end of tree but found {:?}", parser.peek()),
        ));
    }

    tree.add_root((left_index, right_index));

    Ok(())
}

fn parse_vertex(parser: &mut ByteParser, tree: &mut Tree, resolver: &mut LabelResolver) -> Result<TreeIndex, ParsingError> {
    if parser.peek_is(b'(') {
        parse_internal_vertex(parser, tree, resolver)
    } else {
        parse_leaf(parser, tree, resolver)
    }
}

fn parse_internal_vertex(parser: &mut ByteParser, tree: &mut Tree, resolver: &mut LabelResolver) -> Result<TreeIndex, ParsingError> {
    let (left_index, right_index) = parser_children(parser, tree, resolver)?;
    let branch_length = parse_branch_length(parser)?;
    let index = tree.add_internal_vertex((left_index, right_index), branch_length);
    Ok(index)
}

fn parser_children(parser: &mut ByteParser, tree: &mut Tree, resolver: &mut LabelResolver) -> Result<(TreeIndex, TreeIndex), ParsingError> {
    if !parser.consume_if(b'(') {
        return Err(ParsingError::invalid_newick_string(
            parser,
            format!("Expected '(' before children but found {:?}", parser.peek()),
        ));
    }
    let left_index = parse_vertex(parser, tree, resolver)?;

    if !parser.consume_if(b',') {
        return Err(ParsingError::invalid_newick_string(
            parser,
            format!("Expected ',' between children but found {:?}", parser.peek()),
        ));
    }
    let right_index = parse_vertex(parser, tree, resolver)?;

    if !parser.consume_if(b')') {
        return Err(ParsingError::invalid_newick_string(
            parser,
            format!("Expected ')' after children but found {:?}", parser.peek()),
        ));
    }

    Ok((left_index, right_index))
}

fn parse_leaf(parser: &mut ByteParser, tree: &mut Tree, resolver: &mut LabelResolver) -> Result<TreeIndex, ParsingError> {
    let label = parser.parse_label(NEWICK_LABEL_DELIMITERS)?;
    let label_index = resolver.resolve_label(&*label, parser)?;
    let branch_length = parse_branch_length(parser)?;
    let index = tree.add_leaf(branch_length, label_index);
    Ok(index)
}

fn parse_branch_length(parser: &mut ByteParser) -> Result<Option<BranchLength>, ParsingError> {
    if !parser.consume_if(b':') {
        return Ok(None);
    }

    let mut branch_length_str = String::new();
    while let Some(b) = parser.peek() {
        // Valid characters for a float: digits, '.', '-', '+', 'e', 'E'
        if b.is_ascii_digit() || b == b'.' || b == b'-' || b == b'+' || b == b'e' || b == b'E' {
            branch_length_str.push(b as char);
            parser.next(); // consume it
        } else {
            break; // Hit a delimiter like ',', ')', ';', or whitespace
        }
    }

    let value: f64 = branch_length_str.parse()
        .map_err(|_| ParsingError::invalid_newick_string(parser, format!("Invalid branch length: {}", branch_length_str)))?;
    Ok(Some(BranchLength::new(value)))
}

#[derive(Debug)]
pub enum LabelResolver<'a> {
    // When labels are stored directly in Newick string, use LeafLabelMap directly
    LabelToIndex(&'a mut LeafLabelMap), // "Scarabaeus" -> 17
    // When label keys are stored in Newick string and no direct mapping exists yet
    KeyToLabelToIndex {
        translation: HashMap<String, String>, // "beetle"/"42" -> "Scarabaeus"
        leaf_label_map: &'a mut LeafLabelMap, // "Scarabaeus" -> 17
    },
    // When label keys are stored in Newick string and direct mapping already exists
    KeyToIndex(HashMap<String, LabelIndex>),  // "beetle"/"42" -> 17
}

impl<'a> LabelResolver<'a> {
    /// Resolves a parsed label string to a LabelIndex
    ///
    /// For LabelToIndex: inserts the label directly into LeafLabelMap
    /// For KeyToLabelToIndex: looks up translation, then inserts actual label into LeafLabelMap
    /// For KeyToIndex: direct lookup in pre-computed mapping
    pub fn resolve_label(&mut self, parsed_label: &str, parser: &ByteParser) -> Result<LabelIndex, ParsingError> {
        match self {
            LabelResolver::LabelToIndex(leaf_label_map) => {
                Ok(leaf_label_map.get_or_insert(parsed_label))
            }
            LabelResolver::KeyToLabelToIndex { translation, leaf_label_map } => {
                let actual_label = translation
                    .get(parsed_label)
                    .ok_or_else(|| ParsingError::invalid_newick_string(
                        parser,
                        format!("Label '{}' not found in translation map", parsed_label),
                    ))?;
                Ok(leaf_label_map.get_or_insert(actual_label))
            }
            LabelResolver::KeyToIndex(index_map) => {
                index_map.get(parsed_label)
                    .copied()
                    .ok_or_else(|| ParsingError::invalid_newick_string(
                        parser,
                        format!("Label '{}' not found in index map", parsed_label),
                    ))
            }
        }
    }

    /// Create resolver when labels are directly in Newick (no translation table)
    pub fn new_label_to_index(leaf_map: &'a mut LeafLabelMap) -> Self {
        LabelResolver::LabelToIndex(leaf_map)
    }

    /// Create resolver when we have a translation table but no pre-computed index mapping
    /// Used for first tree with a translation table
    pub fn new_key_to_label_to_index(
        translation: HashMap<String, String>,
        leaf_label_map: &'a mut LeafLabelMap,
    ) -> Self {
        LabelResolver::KeyToLabelToIndex { translation, leaf_label_map }
    }

    /// Create resolver when we have a translation table and pre-computed index mapping
    /// Used for subsequent trees with same translation table
    /// Builds the index mapping by looking up all translations in the LeafLabelMap
    pub fn new_key_to_index(
        translation: &HashMap<String, String>,
        leaf_label_map: &LeafLabelMap,
        parser: &ByteParser,
    ) -> Result<Self, ParsingError> {
        let mut index_map = HashMap::with_capacity(translation.len());

        for (key, actual_label) in translation {
            let index = leaf_label_map.get_index(actual_label)
                .ok_or_else(|| ParsingError::invalid_newick_string(
                    parser,
                    format!("Label '{}' not found in LeafLabelMap", actual_label),
                ))?;
            index_map.insert(key.clone(), index);
        }

        Ok(LabelResolver::KeyToIndex(index_map))
    }
}

impl fmt::Display for LabelResolver<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LabelResolver::LabelToIndex(_) => {
                writeln!(f, "LabelResolver (LabelToIndex)")
            }
            LabelResolver::KeyToLabelToIndex { translation, leaf_label_map: _ } => {
                writeln!(f, "LabelResolver (KeyToLabelToIndex):")?;
                for (key, value) in translation {
                    writeln!(f, "  {} -> {}", key, value)?;
                }
                Ok(())
            }
            LabelResolver::KeyToIndex(index_map) => {
                writeln!(f, "LabelResolver (KeyToIndex):")?;
                for (key, value) in index_map {
                    writeln!(f, "  {} -> {}", key, value)?;
                }
                Ok(())
            }
        }
    }
}