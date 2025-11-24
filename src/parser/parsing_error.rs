use crate::parser::byte_parser::ByteParser;
use std::error::Error;
use std::fmt;

/// Error types that can occur during NEXUS and NEWICK parsing
#[derive(PartialEq, Debug, Clone)]
pub enum ParsingErrorType {
    UnexpectedEOF,
    MissingNexusHeader,
    InvalidBlockName,
    InvalidTaxaBlock(String),
    InvalidTreesBlock(String),
    UnclosedComment,
    InvalidNewickString(String),
    InvalidFormatting,
    // ... more as needed
}

/// Parsing error with contextual information (position and surrounding bytes)
#[derive(Debug)]
pub struct ParsingError {
    kind: ParsingErrorType,
    position: usize,
    context: String,
}

impl ParsingError {
    /// Create a ParsingError from an error type and parser state
    pub fn from_parser(kind: ParsingErrorType, parser: &ByteParser) -> Self {
        Self {
            kind,
            position: parser.position(),
            context: parser.get_context_as_string(50),
        }
    }

    /// Convenience constructor for UnexpectedEOF
    pub fn unexpected_eof(parser: &ByteParser) -> Self {
        Self::from_parser(ParsingErrorType::UnexpectedEOF, parser)
    }

    /// Convenience constructor for MissingNexusHeader
    pub fn missing_nexus_header(parser: &ByteParser) -> Self {
        Self::from_parser(ParsingErrorType::MissingNexusHeader, parser)
    }

    /// Convenience constructor for InvalidBlockName
    pub fn invalid_block_name(parser: &ByteParser) -> Self {
        Self::from_parser(ParsingErrorType::InvalidBlockName, parser)
    }

    /// Convenience constructor for InvalidTaxaBlock
    pub fn invalid_taxa_block(parser: &ByteParser, msg: String) -> Self {
        Self::from_parser(ParsingErrorType::InvalidTaxaBlock(msg), parser)
    }

    /// Convenience constructor for InvalidTreesBlock
    pub fn invalid_trees_block(parser: &ByteParser, msg: String) -> Self {
        Self::from_parser(ParsingErrorType::InvalidTreesBlock(msg), parser)
    }

    /// Convenience constructor for UnclosedComment
    pub fn unclosed_comment(parser: &ByteParser) -> Self {
        Self::from_parser(ParsingErrorType::UnclosedComment, parser)
    }

    /// Convenience constructor for InvalidNewickString
    pub fn invalid_newick_string(parser: &ByteParser, msg: String) -> Self {
        Self::from_parser(ParsingErrorType::InvalidNewickString(msg), parser)
    }

    /// Convenience constructor for InvalidFormatting
    pub fn invalid_formatting(parser: &ByteParser) -> Self {
        Self::from_parser(ParsingErrorType::InvalidFormatting, parser)
    }

    /// Get the error kind
    pub fn kind(&self) -> &ParsingErrorType {
        &self.kind
    }

    /// Get the position where the error occurred
    pub fn position(&self) -> usize {
        self.position
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write the main error message
        match &self.kind {
            ParsingErrorType::MissingNexusHeader => write!(f, "File does not start with #NEXUS header")?,
            ParsingErrorType::InvalidTaxaBlock(msg) => write!(f, "Invalid TAXA block format - {msg}")?,
            ParsingErrorType::InvalidTreesBlock(msg) => write!(f, "Invalid TREES block format - {msg}")?,
            ParsingErrorType::UnclosedComment => write!(f, "Unclosed comment")?,
            ParsingErrorType::InvalidBlockName => write!(f, "Invalid block name")?,
            ParsingErrorType::InvalidNewickString(msg) => write!(f, "Invalid newick string: {}", msg)?,
            ParsingErrorType::UnexpectedEOF => write!(f, "Unexpected end of file")?,
            ParsingErrorType::InvalidFormatting => write!(f, "Invalid formatting")?,
        }

        // Add position information
        write!(f, " at position {}", self.position)?;

        // Add context if available
        if !self.context.is_empty() {
            write!(f, "\n  Context (next {} bytes): {}", self.context.len(), self.context)?;
        }

        Ok(())
    }
}

impl Error for ParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}