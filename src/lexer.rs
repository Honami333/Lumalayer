use std::error::Error;

use wibr::{Make, New};

use crate::error::{LumaError, LumaResult};

pub const LPAREN: char = '(';
pub const RPAREN: char = ')';
pub const LBRACE: char = '{';
pub const RBRACE: char = '}';
pub const LBRACKET: char = '[';
pub const RBRACKET: char = ']';
pub const LT: char = '<';
pub const GT: char = '>';

pub const COLON: char = ':';
pub const SEMICOLON: char = ';';
pub const COMMA: char = ',';
pub const DOT: char = '.';

pub const PLUS: char = '+';
pub const MINUS: char = '-';
pub const STAR: char = '*';
pub const SLASH: char = '/';

pub const EQ: char = '=';
pub const QUOTE: char = '"';
pub const HASH: char = '#';
pub const UNDERSCORE: char = '_';
pub const NEWLINE: char = '\n';


#[derive(Debug, Clone, PartialEq, Make)]
pub struct LumaLexer {
    #[Some(Vec::new())] pub errors: Vec<LumaError>,
    #[Some(Vec::new())] pub tokens: Vec<Spanned>
}

impl LumaLexer {
    pub fn tokenize(&mut self, data: &str) {
        let mut chars = data.char_indices().peekable();

        while let Some((i, ch)) = chars.next() {
            if ch.is_whitespace() { continue; };

            let token = match ch {
                LPAREN => Some(Token::LParen),
                RPAREN => Some(Token::RParen),
                LBRACE => Some(Token::LBrace),
                RBRACE => Some(Token::RBrace),
                LBRACKET => Some(Token::LBracket),
                RBRACKET => Some(Token::RBracket),
                LT => Some(Token::Lt),
                GT => Some(Token::Gt),
                SEMICOLON => Some(Token::Semicolon),
                COMMA => Some(Token::Comma),
                DOT => Some(Token::Dot),
                PLUS => Some(Token::Plus),
                MINUS => Some(Token::Minus),
                STAR => Some(Token::Star),
                _ => None
            };

            if let Some(token) = token {
                self.tokens.push(Spanned::new(token, Span::new(i, i + 1)));
                continue;
            };

            if ch == COLON {
                let (token, end) = match chars.peek() {
                    Some((_, COLON)) => {
                        chars.next();
                        (Token::PathSep, 2)
                    },
                    _ => (Token::Colon, 1)
                };
                self.tokens.push(Spanned::new(token, Span::new(i, i + end)));
                continue;
            };

            if let Some((_, gt)) = chars.peek() && ch == EQ && *gt == GT {
                chars.next();
                self.tokens.push(Spanned::new(Token::FatArrow, Span::new(i, i + 2)));
                continue;
            };

            if ch == QUOTE {
                let end = string(&mut chars, i + 1);
                match end {
                    Ok(e) => self.tokens.push(Spanned::new(Token::Str(data[i + 1..e].to_string()), Span::new(i, e + 1))),
                    Err(err) => self.errors.push(err),
                };
                continue;
            };
            
            if ch.is_ascii_alphabetic() || ch == UNDERSCORE {
                let end = word(&mut chars, i);
                self.tokens.push(Spanned::new(Token::from_word(&data[i..end]), Span::new(i, end)));
                continue;
            };

            if ch.is_ascii_digit() {
                let end = number(&mut chars, i);
                let string = &data[i..end];
                let span = Span::new(i, end);
                let token = match (string.parse::<usize>(), string.parse::<f32>()) {
                    (Ok(u), _) => Token::Uint(u),
                    (_, Ok(float)) => Token::Float(float),
                    _ => {
                        self.errors.push(LumaError::InvalidNumber { span });
                        continue;
                    }
                };
                self.tokens.push(Spanned::new(token, span));
                continue;
            };

            if ch == SLASH {
                if let Some((_, SLASH)) = chars.peek() {
                    while let Some((_, ch)) = chars.next() && ch != NEWLINE {};
                    continue;
                };
                self.tokens.push(Spanned::new(Token::Slash, Span::new(i, i + 1)));
                continue;
            };

            if ch == HASH {
                if chars.peek().is_none() {
                    self.errors.push(LumaError::InvalidColorLength { len: 0, span: Span::new(i, i + 1) });
                    continue;
                };
                let end = color(&mut chars, i + 1);
                let color = &data[i + 1..end];
                let span = Span::new(i, end);
                match color.len() {
                    6 => {
                        match parse_rgb(color, span) {
                            Ok((r, g, b)) => color_to_tokens(&mut self.tokens, (r, g, b, 255), span),
                            Err(err) => self.errors.push(err),
                        };
                    },
                    8 => {
                        match parse_rgba(color, span) {
                            Ok((r, g, b, a)) => color_to_tokens(&mut self.tokens, (r, g, b, a), span),
                            Err(err) => self.errors.push(err),
                        };
                    },
                    _ => self.errors.push(LumaError::InvalidColorLength { len: color.len(), span })
                };
                continue;
            };

            self.errors.push(LumaError::UnexpectedChar { ch, span: Span::new(i, i + 1) });
        };
    }

    pub fn clear(&mut self) {
        self.tokens.clear();
        self.errors.clear();
    }
}

fn parse_rgb(color: &str, span: Span) -> LumaResult<(u8, u8, u8)> {
    match
    (
        u8::from_str_radix(&color[0..2], 16),
        u8::from_str_radix(&color[2..4], 16),
        u8::from_str_radix(&color[4..6], 16)
    ) {
        (Ok(r), Ok(g), Ok(b)) => Ok((r, g, b)),
        _ => Err(LumaError::InvalidColor { span })
    }
}

fn parse_rgba(color: &str, span: Span) -> LumaResult<(u8, u8, u8, u8)> {
    let (r, g, b) = parse_rgb(color, span)?;
    match u8::from_str_radix(&color[6..8], 16) {
        Ok(a) => Ok((r, g, b, a)),
        Err(_) => Err(LumaError::InvalidColor { span })
    }
}


fn color_to_tokens(tokens: &mut Vec<Spanned>, (r, g, b, a): (u8, u8, u8, u8), span: Span) {
    tokens.push(Spanned::new(Token::LParen, span));
    tokens.push(Spanned::new(Token::Uint(r as usize), span));
    tokens.push(Spanned::new(Token::Comma, span));
    tokens.push(Spanned::new(Token::Uint(g as usize), span));
    tokens.push(Spanned::new(Token::Comma, span));
    tokens.push(Spanned::new(Token::Uint(b as usize), span));
    tokens.push(Spanned::new(Token::Comma, span));
    tokens.push(Spanned::new(Token::Uint(a as usize), span));
    tokens.push(Spanned::new(Token::RParen, span));
}

fn color(chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>, start: usize) -> usize {
    let mut end = start;
    while let Some((i, ch)) = chars.peek().copied() && ch.is_ascii_alphanumeric() {
        end += i - end;
        chars.next();
        if end - start == 7 { break; };
    };
    end + 1
}

fn word(chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>, start: usize) -> usize {
    let mut end = start;
    while let Some((i, ch)) = chars.peek().copied() && (ch.is_ascii_alphanumeric() || ch == UNDERSCORE) {
        end += i - end;
        chars.next();
    };
    end + 1
}

fn string(chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>, start: usize) -> LumaResult<usize> {
    let mut end = start;
    loop {
        match chars.peek().copied() {
            Some((i, ch)) => {
                chars.next();
                if ch == QUOTE {
                    end = i;
                    break;
                };
            },
            None => return Err(LumaError::UnterminatedString { span: Span::new(start, end) })
        };
    };
    Ok(end)
}

fn number(chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>, start: usize) -> usize {
    let mut end = start;
    let mut dot_finded = false;
    while let Some((i, ch)) = chars.peek().copied() && (ch.is_ascii_digit() || ch == DOT) {
        if ch == DOT {
            let mut clone = chars.clone();
            clone.next();
            match clone.peek() {
                Some((_, ch)) if !ch.is_ascii_digit() => break,
                None => break,
                _ => {}
            };
        };

        match (dot_finded, ch == DOT) {
            (false, true) => dot_finded = true,
            (true, true) => break,
            _ => {}
        };
        
        end += i - end;
        chars.next();
    };
    end + 1
}



impl Error for LumaError {}

#[derive(Debug, Clone, Copy, PartialEq, New)]
pub struct Span { pub start: usize, pub end: usize }

impl Span {
    pub fn set(&mut self, other: Self) { *self = other; }
}

impl Default for Span {
    fn default() -> Self { Span::new(0, 0) }
}

#[derive(Debug, Clone, PartialEq, New)]
pub struct Spanned { pub token: Token, pub span: Span }

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Ключевые слова
    Use,
    Crate,
    Super,
    Widget(WidgetKw),
    Hit(HitKw),
    Prop(PropKw),
    Variant(VariantKw),

    // Значения
    Ident(String),
    Str(String),
    Bool(bool),
    Uint(usize),
    Float(f32),

    // Пунктуация
    FatArrow,
    LParen, RParen,
    LBrace, RBrace,
    LBracket, RBracket,
    Lt, Gt,
    Colon, PathSep,
    Plus, Minus, Star, Slash,
    Semicolon, Comma, Dot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetKw {
    Node, Layer, Label, DynamicLabel, Container, Image,
    TextStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitKw { HitVec, HitMap }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropKw {
    Size, Scale, Transform, Parent, Rotate, Color,
    Text, FontSize, LineHeight, LetterSpacing, Align,
    Padding, Clip, ImageHandle, Visible, Opacity
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VariantKw {
    #[default] TopLeft, TopCenter, TopRight,
    CenterLeft, CenterCenter, CenterRight,
    BottomLeft, BottomCenter, BottomRight
}

impl WidgetKw {
    pub fn from_word(s: &str) -> Option<Self> {
        Some(match s {
            "Node" => Self::Node,
            "Layer" => Self::Layer,
            "Label" => Self::Label,
            "DynamicLabel" => Self::DynamicLabel,
            "Container" => Self::Container,
            "Image" => Self::Image,
            "TextStyle" => Self::TextStyle,
            _ => return None,
        })
    }
}

impl HitKw {
    pub fn from_word(s: &str) -> Option<Self> {
        Some(match s {
            "HitVec" => Self::HitVec,
            "HitMap" => Self::HitMap,
            _ => return None,
        })
    }
}

impl PropKw {
    pub fn from_word(s: &str) -> Option<Self> {
        Some(match s {
            "size" => Self::Size,
            "scale" => Self::Scale,
            "transform" => Self::Transform,
            "parent" => Self::Parent,
            "rotate" => Self::Rotate,
            "color" => Self::Color,
            "text" => Self::Text,
            "font_size" => Self::FontSize,
            "line_height" => Self::LineHeight,
            "letter_spacing" => Self::LetterSpacing,
            "align" => Self::Align,
            "padding" => Self::Padding,
            "clip" => Self::Clip,
            "image_handle" => Self::ImageHandle,
            "visible" => Self::Visible,
            "opacity" => Self::Opacity,
            _ => return None,
        })
    }
}

impl VariantKw {
    pub fn from_word(s: &str) -> Option<Self> {
        Some(match s {
            "TopLeft" => Self::TopLeft,
            "TopCenter" => Self::TopCenter,
            "TopRight" => Self::TopRight,
            "CenterLeft" => Self::CenterLeft,
            "CenterCenter" => Self::CenterCenter,
            "CenterRight" => Self::CenterRight,
            "BottomLeft" => Self::BottomLeft,
            "BottomCenter" => Self::BottomCenter,
            "BottomRight" => Self::BottomRight,
            _ => return None,
        })
    }
}

impl Token {
    pub fn from_word(s: &str) -> Self {
        match s {
            "use" => return Self::Use,
            "crate" => return Self::Crate,
            "super" => return Self::Super,
            "true" => return Self::Bool(true),
            "false" => return Self::Bool(false),
            _ => {}
        };
        if let Some(k) = WidgetKw::from_word(s) { return Self::Widget(k); }
        if let Some(k) = HitKw::from_word(s) { return Self::Hit(k); }
        if let Some(k) = PropKw::from_word(s) { return Self::Prop(k); }
        if let Some(k) = VariantKw::from_word(s) { return Self::Variant(k); }
        Self::Ident(s.to_string())
    }
}