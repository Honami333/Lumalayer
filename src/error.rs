use std::fmt::Display;

use crate::lexer::{Span, Spanned};

pub type LumaResult<T> = Result<T, LumaError>;


#[derive(Debug, Clone, PartialEq)]
pub enum LumaError {
    UnterminatedString { span: Span },
    UnexpectedChar { ch: char, span: Span },
    TokenNotFound { spanned: Spanned },
    UnexpectedToken { spanned: Spanned },
    InvalidNumber { span: Span },
    InvalidColor { span: Span },
    InvalidColorLength { len: usize, span: Span },
    UnexpectedEof { span: Span  },
    ASTParseError { span: Span },
    ColorOutOfRange { value: usize, span: Span },
    OpacityOutOfRange { value: usize, span: Span } ,
    DuplicateProperty { span: Span },
    MissingText { span: Span },
    MissingImage { span: Span },
    MissingHitCondition { span: Span },
    DuplicateHitBlock { span: Span },
    EmptyUse { span: Span },
    InvalidPath { span: Span }
}

impl Display for LumaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnterminatedString { span } => write!(f, "unterminated string literal at {}..{}", span.start, span.end),
            Self::UnexpectedChar { ch, span } => write!(f, "unexpected character '{}' at {}..{}", ch, span.start, span.end),
            Self::InvalidNumber { span } => write!(f, "invalid number literal at {}..{}", span.start, span.end),
            Self::InvalidColor { span } => write!(f, "invalid color literal at {}..{}", span.start, span.end),
            Self::InvalidColorLength { len, span } => write!(f, "invalid color length {}, expected 6 or 8 at {}..{}", len, span.start, span.end),
            Self::TokenNotFound { spanned } => write!(f, "token {:?} not found at {}..{}", spanned.token, spanned.span.start, spanned.span.end),
            Self::UnexpectedEof { span } => write!(f, "unexpected end of input at {}..{}", span.start, span.end),
            Self::UnexpectedToken { spanned } => write!(f, "unexpected token {:?} at {}..{}", spanned.token, spanned.span.start, spanned.span.end),
            Self::ASTParseError { span } => write!(f, "failed to parse AST node at {}..{}", span.start, span.end),
            Self::ColorOutOfRange { value, span } => write!(f, "color component {} out of range 0..255 at {}..{}", value, span.start, span.end),
            Self::OpacityOutOfRange { value, span } => write!(f, "opacity {} out of range 0..255 at {}..{}", value, span.start, span.end),
            Self::DuplicateProperty { span } => write!(f, "property already defined at {}..{}", span.start, span.end),
            Self::MissingText { span } => write!(f, "label has no text at {}..{}", span.start, span.end),
            Self::MissingImage { span } => write!(f, "image has no handle at {}..{}", span.start, span.end),
            Self::MissingHitCondition { span } => write!(f, "hit entry has no condition before ':' at {}..{}", span.start, span.end),
            Self::DuplicateHitBlock { span } => write!(f, "hit block of this type already defined at {}..{}", span.start, span.end),
            Self::EmptyUse { span } => write!(f, "use statement has no path at {}..{}", span.start, span.end),
            Self::InvalidPath { span } => write!(f, "invalid path at {}..{}", span.start, span.end),
        }
    }
}