use crate::*;

use crate::error::{LumaResult, LumaError};
use crate::lexer::{HitKw, LumaLexer, VariantKw};
use crate::lexer::PropKw;
use crate::lexer::Span;
use crate::lexer::Spanned;
use crate::lexer::Token;
use crate::lexer::WidgetKw;

use wibr::Make;
use wibr::New;




pub fn main() {
    let data = std::fs::read_to_string("ui.luma").unwrap();

    let mut lexer = LumaLexer::new();
    lexer.tokenize(&data);

    let mut tokens = lexer.tokens.iter().peekable();

    let mut errors = Vec::new();
    let mut layers = Vec::new();
    let mut imports = Vec::new();

    while let Some(spanned) = tokens.next() {
        let mut last_span = spanned.span;

        if spanned.token == Token::Use {
            let mut import = Vec::new();
            import.push((*spanned).clone());
            let mut path_sep_is_last = false;
            let mut token_is_last = true;

            match tokens.peek() {
                Some(imp) => {
                    let imp = (*imp).clone();
                    match imp.token {
                        Token::Crate => import.push(imp),
                        Token::Super => import.push(imp),
                        _ => {
                            errors.push(LumaError::InvalidPath { span: imp.span });
                            continue;
                        }
                    };
                    tokens.next();
                },
                None => errors.push(LumaError::EmptyUse { span: spanned.span })
            };

            while let Some(semicolon) = tokens.peek() && semicolon.token != Token::Semicolon {
                let path = (*tokens.next().unwrap()).clone();
                match path.token {
                    Token::LBrace | Token::RBrace | Token::Comma => {
                        token_is_last = false;
                        path_sep_is_last = false;
                    },
                    Token::Ident(_)  => {
                        match token_is_last {
                            true => {
                                errors.push(LumaError::InvalidPath { span: path.span });
                                break;
                            },
                            false => token_is_last = true
                        };
                        path_sep_is_last = false;
                    },
                    Token::PathSep => {
                        match path_sep_is_last {
                            true => {
                                errors.push(LumaError::InvalidPath { span: path.span });
                                break;
                            },
                            false => path_sep_is_last = true
                        };
                        token_is_last = false;
                    }
                    _ => {
                        errors.push(LumaError::UnexpectedToken { spanned: path });
                        continue;
                    }
                };
                import.push(path);
            };

            if path_sep_is_last {
                errors.push(LumaError::InvalidPath { span: import.last().unwrap().span });
                continue;
            };
            
            imports.push(import);
            take_log!(Token::Semicolon, last_span, tokens, errors);
            continue;
        };

        if spanned.token == Token::Widget(WidgetKw::Layer) {
            let Some(ident) = take_log!(Token::Ident(_), last_span, tokens, errors) else { continue; };
            if !open_brace!(last_span, tokens, errors) { continue; };

            let mut node = Vec::new();
            let mut label = Vec::new();
            let mut container = Vec::new();
            let mut image = Vec::new();
            let mut hit = ASTHitLayer::new();
            
            while let Some(rbrace) = tokens.peek() && rbrace.token != Token::RBrace {
                if take!(Token::Widget(WidgetKw::Node), last_span, tokens).is_ok() {
                    let Some(ident) = take_log!(Token::Ident(_), last_span, tokens, errors) else { continue; };

                    if !open_brace!(last_span, tokens, errors) { continue; };

                    let mut size = None;
                    let mut scale = None;
                    let mut transform = None;
                    let mut parent = None;
                    while let Some(rbrace) = tokens.peek() && rbrace.token != Token::RBrace {
                        if let Some(spanned) = tokens.peek() {
                            match spanned.token {
                                Token::Prop(PropKw::Size) => duplicate_log!(size, parse_size, last_span, tokens, errors),
                                Token::Prop(PropKw::Scale) => duplicate_log!(scale, parse_scale, last_span, tokens, errors),
                                Token::Prop(PropKw::Transform) => duplicate_log!(transform, parse_transform, last_span, tokens, errors),
                                Token::Prop(PropKw::Parent) => duplicate_log!(parent, parse_parent, last_span, tokens, errors),
                                _ => {
                                    errors.push(LumaError::UnexpectedToken { spanned: (*spanned).clone() });
                                    tokens.next();
                                }
                            };
                        };
                    };

                    if !close_brace!(last_span, tokens, errors) { continue; };

                    node.push(ASTNode::new(
                        ident,
                        size.unwrap_or_default(),
                        scale.unwrap_or_default(),
                        transform.unwrap_or_default(),
                        parent
                    ));
                    continue;
                };

                if take!(Token::Widget(WidgetKw::Container), last_span, tokens).is_ok() {
                    let Some(ident) = take_log!(Token::Ident(_), last_span, tokens, errors) else { continue; };

                    if !open_brace!(last_span, tokens, errors) { continue; };

                    let mut size = None;
                    let mut scale = None;
                    let mut transform = None;
                    let mut parent = None;
                    let mut rotate = None;
                    let mut padding = None;
                    let mut clip = None;
                    let mut color = None;
                    let mut visible = None;
                    let mut opacity = None;
                    while let Some(rbrace) = tokens.peek() && rbrace.token != Token::RBrace {
                        if let Some(spanned) = tokens.peek() {
                            match spanned.token {
                                Token::Prop(PropKw::Size) => duplicate_log!(size, parse_size, last_span, tokens, errors),
                                Token::Prop(PropKw::Scale) => duplicate_log!(scale, parse_scale, last_span, tokens, errors),
                                Token::Prop(PropKw::Transform) => duplicate_log!(transform, parse_transform, last_span, tokens, errors),
                                Token::Prop(PropKw::Parent) => duplicate_log!(parent, parse_parent, last_span, tokens, errors),
                                Token::Prop(PropKw::Rotate) => duplicate_log!(rotate, parse_rotate, last_span, tokens, errors),
                                Token::Prop(PropKw::Padding) => duplicate_log!(padding, parse_padding, last_span, tokens, errors),
                                Token::Prop(PropKw::Clip) => duplicate_log!(clip, parse_clip, last_span, tokens, errors),
                                Token::Prop(PropKw::Color) => duplicate_log!(color, parse_color, last_span, tokens, errors),
                                Token::Prop(PropKw::Visible) => duplicate_log!(visible, parse_visible, last_span, tokens, errors),
                                Token::Prop(PropKw::Opacity) => duplicate_log!(opacity, parse_opacity, last_span, tokens, errors),
                                _ => {
                                    errors.push(LumaError::UnexpectedToken { spanned: (*spanned).clone() });
                                    tokens.next();
                                }
                            };
                        };
                    };

                    if !close_brace!(last_span, tokens, errors) { continue; };

                    let mut container_style = ASTContainerStyle::default();
                    if let Some(padding) = padding { error_log!(container_style.padding_spanned_parse(padding), errors); };
                    if let Some(clip) = clip { error_log!(container_style.clip_spanned_parse(clip), errors); };

                    let mut visual_mode = ASTVisualMode::default();
                    if let Some(visible) = visible { error_log!(visual_mode.visible_spanned_parse(visible), errors); };
                    if let Some(opacity) = opacity { error_log!(visual_mode.opacity_spanned_parse(opacity), errors); };

                    container.push(ASTContainer::new(
                        ASTNode::new(
                            ident,
                            size.unwrap_or_default(),
                            scale.unwrap_or_default(),
                            transform.unwrap_or_default(),
                            parent
                        ),
                        rotate.unwrap_or_default(),
                        container_style,
                        color.unwrap_or_default(),
                        visual_mode
                    ));
                    continue;
                };

                if let (
                    lb,
                    dlb
                ) = (
                    take!(Token::Widget(WidgetKw::Label), last_span, tokens),
                    take!(Token::Widget(WidgetKw::DynamicLabel), last_span, tokens)
                ) && (lb.is_ok() || dlb.is_ok()) {
                    let is_dynamic = match (lb.is_ok(), dlb.is_ok()) {
                        (true, false) => false,
                        (false, true) => true,
                        _ => continue
                    };

                    let Some(ident) = take_log!(Token::Ident(_), last_span, tokens, errors) else { continue; };

                    if !open_brace!(last_span, tokens, errors) { continue; };

                    let mut size = None;
                    let mut scale = None;
                    let mut transform = None;
                    let mut parent = None;
                    let mut rotate = None;
                    let mut color = None;
                    let mut visible = None;
                    let mut opacity = None;
                    let mut text_ident: Option<Spanned> = None;
                    let mut text = None;
                    let mut font_size = None;
                    let mut text_color = None;
                    let mut line_height = None;
                    let mut letter_spacing = None;
                    let mut align = None;
                    while let Some(rbrace) = tokens.peek() && rbrace.token != Token::RBrace {
                        if let Some(spanned) = tokens.peek() {
                            match spanned.token {
                                Token::Prop(PropKw::Size) => duplicate_log!(size, parse_size, last_span, tokens, errors),
                                Token::Prop(PropKw::Scale) => duplicate_log!(scale, parse_scale, last_span, tokens, errors),
                                Token::Prop(PropKw::Transform) => duplicate_log!(transform, parse_transform, last_span, tokens, errors),
                                Token::Prop(PropKw::Parent) => duplicate_log!(parent, parse_parent, last_span, tokens, errors),
                                Token::Prop(PropKw::Rotate) => duplicate_log!(rotate, parse_rotate, last_span, tokens, errors),
                                Token::Prop(PropKw::Color) => duplicate_log!(color, parse_color, last_span, tokens, errors),
                                Token::Prop(PropKw::Visible) => duplicate_log!(visible, parse_visible, last_span, tokens, errors),
                                Token::Prop(PropKw::Opacity) => duplicate_log!(opacity, parse_opacity, last_span, tokens, errors),
                                Token::Prop(PropKw::Text) => duplicate_log!(text, parse_text, last_span, tokens, errors),
                                Token::Widget(WidgetKw::TextStyle) => {
                                    if let Some(ident) = &text_ident {
                                        tokens.next();
                                        errors.push(LumaError::DuplicateProperty { span: ident.span });
                                        continue;
                                    };
                                    take_log!(Token::Widget(WidgetKw::TextStyle), last_span, tokens, errors);
                                    text_ident = take_log!(Token::Ident(_), last_span, tokens, errors);

                                    if !open_brace!(last_span, tokens, errors) { continue; };

                                    while let Some(rbrace) = tokens.peek() && rbrace.token != Token::RBrace {
                                        if let Some(spanned) = tokens.peek() {
                                            match spanned.token {
                                                Token::Prop(PropKw::Text) => duplicate_log!(text, parse_text, last_span, tokens, errors),
                                                Token::Prop(PropKw::FontSize) => duplicate_log!(font_size, parse_font_size, last_span, tokens, errors),
                                                Token::Prop(PropKw::Color) => duplicate_log!(text_color, parse_color, last_span, tokens, errors),
                                                Token::Prop(PropKw::LineHeight) => duplicate_log!(line_height, parse_line_height, last_span, tokens, errors),
                                                Token::Prop(PropKw::LetterSpacing) => duplicate_log!(letter_spacing, parse_letter_spacing, last_span, tokens, errors),
                                                Token::Prop(PropKw::Align) => duplicate_log!(align, parse_align, last_span, tokens, errors),
                                                _ => {
                                                    errors.push(LumaError::UnexpectedToken { spanned: (*spanned).clone() });
                                                    tokens.next();
                                                }
                                            };
                                        };
                                    };

                                    if !close_brace!(last_span, tokens, errors) { continue; };
                                },
                                _ => {
                                    errors.push(LumaError::UnexpectedToken { spanned: (*spanned).clone() });
                                    tokens.next();
                                }
                            };
                        };
                    };

                    if !close_brace!(last_span, tokens, errors) { continue; };

                    let Some(text) = text else {
                        errors.push(LumaError::MissingText { span: ident.span });
                        continue;
                    };

                    let text_style = ASTTextStyle::new(
                        text_ident,
                        text,
                        font_size.unwrap_or_default(),
                        text_color.unwrap_or_default(),
                        line_height.unwrap_or_default(),
                        letter_spacing.unwrap_or_default(),
                        align.unwrap_or(Spanned::new(Token::Variant(VariantKw::default()), Span::default()))
                    );

                    let mut visual_mode = ASTVisualMode::default();
                    if let Some(visible) = visible { error_log!(visual_mode.visible_spanned_parse(visible), errors); };
                    if let Some(opacity) = opacity { error_log!(visual_mode.opacity_spanned_parse(opacity), errors); };

                    label.push(ASTLabel::new(
                        ASTNode::new(
                            ident,
                            size.unwrap_or_default(),
                            scale.unwrap_or_default(),
                            transform.unwrap_or_default(),
                            parent
                        ),
                        rotate.unwrap_or_default(),
                        text_style,
                        color.unwrap_or_default(),
                        visual_mode,
                        is_dynamic
                    ));
                    continue;
                };

                if take!(Token::Widget(WidgetKw::Image), last_span, tokens).is_ok() {
                    let Some(ident) = take_log!(Token::Ident(_), last_span, tokens, errors) else { continue; };

                    if !open_brace!(last_span, tokens, errors) { continue; };

                    let mut size = None;
                    let mut scale = None;
                    let mut transform = None;
                    let mut parent = None;
                    let mut visible = None;
                    let mut opacity = None;
                    let mut image_handle = None;
                    while let Some(rbrace) = tokens.peek() && rbrace.token != Token::RBrace {
                        if let Some(spanned) = tokens.peek() {
                            match spanned.token {
                                Token::Prop(PropKw::Size) => duplicate_log!(size, parse_size, last_span, tokens, errors),
                                Token::Prop(PropKw::Scale) => duplicate_log!(scale, parse_scale, last_span, tokens, errors),
                                Token::Prop(PropKw::Transform) => duplicate_log!(transform, parse_transform, last_span, tokens, errors),
                                Token::Prop(PropKw::Parent) => duplicate_log!(parent, parse_parent, last_span, tokens, errors),
                                Token::Prop(PropKw::ImageHandle) => duplicate_log!(image_handle, parse_image_handle, last_span, tokens, errors),
                                Token::Prop(PropKw::Visible) => duplicate_log!(visible, parse_visible, last_span, tokens, errors),
                                Token::Prop(PropKw::Opacity) => duplicate_log!(opacity, parse_opacity, last_span, tokens, errors),
                                _ => {
                                    errors.push(LumaError::UnexpectedToken { spanned: (*spanned).clone() });
                                    tokens.next();
                                }
                            };
                        };
                    };

                    if !close_brace!(last_span, tokens, errors) { continue; };

                    let Some(image_handle) = image_handle else {
                        errors.push(LumaError::MissingImage { span: ident.span });
                        continue;
                    };

                    let mut visual_mode = ASTVisualMode::default();
                    if let Some(visible) = visible { error_log!(visual_mode.visible_spanned_parse(visible), errors); };
                    if let Some(opacity) = opacity { error_log!(visual_mode.opacity_spanned_parse(opacity), errors); };

                    image.push(ASTImage::new(
                        ASTNode::new(
                            ident,
                            size.unwrap_or_default(),
                            scale.unwrap_or_default(),
                            transform.unwrap_or_default(),
                            parent
                        ),
                        image_handle,
                        visual_mode
                    ));
                    continue;
                };

                if let Ok(hit_kw) = take!(Token::Hit(_), last_span, tokens) {
                    let Some(_) = take_log!(Token::Lt, last_span, tokens, errors) else { continue; };
                    let Some(ty) = take_log!(Token::Ident(_), last_span, tokens, errors) else { continue; };
                    let Some(_) = take_log!(Token::Gt, last_span, tokens, errors) else { continue; };

                    if !open_brace!(last_span, tokens, errors) { continue; };

                    let mut block = ASTHitBlock::new(ty);
                    while let Some(rbrace) = tokens.peek() && rbrace.token != Token::RBrace {
                        let mut hit_token = Vec::new();
                        while let Some(fat_arrow) = tokens.peek() && fat_arrow.token != Token::FatArrow {
                            let spanned = (*tokens.next().unwrap()).clone();
                            hit_token.push(spanned);
                        };
                        let Some(_) = take_log!(Token::FatArrow, last_span, tokens, errors) else { continue; };
                        let Some(ident) = take_log!(Token::Ident(_), last_span, tokens, errors) else { continue; };
                        take_comma!(last_span, tokens, errors);
                        if hit_token.is_empty() {
                            errors.push(LumaError::MissingHitCondition { span: ident.span });
                            break;
                        };
                        let entry = ASTHitEntry::new(hit_token, ident);
                        block.entries.push(entry);
                    };

                    if !close_brace!(last_span, tokens, errors) { continue; };

                    match hit_kw.token {
                        Token::Hit(HitKw::HitVec) if hit.vec.is_none() => hit.vec = Some(block),
                        Token::Hit(HitKw::HitMap) if hit.map.is_none() => hit.map = Some(block),
                        _ => errors.push(LumaError::DuplicateHitBlock { span: hit_kw.span })
                    };

                    continue;
                };

                if let Some(spanned) = tokens.next() { errors.push(LumaError::UnexpectedToken { spanned: spanned.to_owned() }); };
            }

            if !close_brace!(last_span, tokens, errors) { continue; };

            layers.push(ASTLayer::new(ident, node, label, container, image, hit));
            continue;
        };

        errors.push(LumaError::UnexpectedToken { spanned: spanned.to_owned() });
    };

    // for spaned in lexer.tokens { println!("{:?}", spaned); };
    for error in lexer.errors { println!("lexer: {}", error); };
    for error in errors { println!("parser: {}", error); };
    
    for import in imports { println!("{:?}", import); };
    for layer in layers { println!("{:?}", layer); };
}

#[macro_export]
macro_rules! duplicate_log {
    ($property:expr, $parser:ident, $span:expr, $tokens:expr, $errors:expr) => {{
        match $property.is_none() {
            true => {
                $property = $parser!($span, $tokens, $errors);
                take_comma!($span, $tokens, $errors);
            },
            false => {
                $tokens.next();
                $errors.push(LumaError::DuplicateProperty { span: $span })
            }
        };
    }};
}

#[macro_export]
macro_rules! take_comma {
    ($span:expr, $tokens:expr, $errors:expr) => {
        let mut clone = $tokens.clone();
        if let Some(clone_spanned) = clone.peek() && clone_spanned.token == Token::Comma { clone.next(); };
        match clone.peek() {
            Some(Spanned { token: Token::Prop(_), ..}) => { _ = take_log!(Token::Comma, $span, $tokens, $errors); },
            _ => { _ = take!(Token::Comma, $span, $tokens); }
        };
    };
}

#[macro_export]
macro_rules! parse_node {
    ($ident:expr, $span:expr, $tokens:expr, $errors:expr) => {{
        match (
            parse_size!($span, $tokens, $errors),
            take_log!(Token::Comma, $span, $tokens, $errors).is_some(),
            parse_scale!($span, $tokens, $errors),
            take_log!(Token::Comma, $span, $tokens, $errors).is_some(),
            parse_transform!($span, $tokens, $errors),
        ) {
            (Some(size), true, Some(scale), true, Some(transform)) => {
                let parent = match take!(Token::Comma, $span, $tokens).is_ok() {
                    true => {
                        let parent = parse_parent!($span, $tokens, $errors);
                        _ = take!(Token::Comma, $span, $tokens);
                        parent
                    },
                    false => None
                };
                Some(ASTNode::new($ident, size, scale, transform, parent))
            },
            _ => None
        }
    }}
}


#[macro_export]
macro_rules! parse_rotate {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        match (
            take_log!(Token::Prop(PropKw::Rotate), $span, $tokens, $errors).is_some(),
            take_log!(Token::Colon, $span, $tokens, $errors).is_some(),
            take_log!(Token::LParen, $span, $tokens, $errors).is_some(),
            take_log!(Token::Float(_), $span, $tokens, $errors),
            take_log!(Token::Comma, $span, $tokens, $errors).is_some(),
            take_log!(Token::Variant(_), $span, $tokens, $errors),
            take_log!(Token::RParen, $span, $tokens, $errors).is_some()
        ) {
            (true, true, true, Some(rotate), true, Some(anchor), true) => error_log!(ASTRotate::spanned_parse(rotate, anchor, $span), $errors),
            _ => None
        }
    }}
}

#[macro_export]
macro_rules! parse_color {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        match (
            take_log!(Token::Prop(PropKw::Color), $span, $tokens, $errors).is_some(),
            take_log!(Token::Colon, $span, $tokens, $errors).is_some(),
            take_log!(Token::LParen, $span, $tokens, $errors).is_some(),
            take_log!(Token::Uint(_), $span, $tokens, $errors),
            take_log!(Token::Comma, $span, $tokens, $errors).is_some(),
            take_log!(Token::Uint(_), $span, $tokens, $errors),
            take_log!(Token::Comma, $span, $tokens, $errors).is_some(),
            take_log!(Token::Uint(_), $span, $tokens, $errors),
            take_log!(Token::Comma, $span, $tokens, $errors).is_some(),
            take_log!(Token::Uint(_), $span, $tokens, $errors),
            take_log!(Token::RParen, $span, $tokens, $errors).is_some()
        ) {
            (true, true, true, Some(r), true, Some(g), true, Some(b), true, Some(a), true) => error_log!(ASTColor::spanned_parse((r, g, b, a), $span), $errors),
            _ => None
        }
    }}
}

#[macro_export]
macro_rules! parse_visible {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        match (
            take_log!(Token::Prop(PropKw::Visible), $span, $tokens, $errors).is_some(),
            take_log!(Token::Colon, $span, $tokens, $errors).is_some(),
            take_log!(Token::Bool(_), $span, $tokens, $errors),
        ) {
            (true, true, visible) => visible,
            _ => None
        }
    }}
}

#[macro_export]
macro_rules! parse_font_size {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        match (
            take_log!(Token::Prop(PropKw::FontSize), $span, $tokens, $errors).is_some(),
            take_log!(Token::Colon, $span, $tokens, $errors).is_some(),
            take_log!(Token::Uint(_), $span, $tokens, $errors),
        ) {
            (true, true, Some(font_size)) => error_log!(ASTFontSize::spanned_parse(font_size, $span), $errors),
            _ => None
        }
    }}
}

#[macro_export]
macro_rules! parse_image_handle {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        match (
            take_log!(Token::Prop(PropKw::ImageHandle), $span, $tokens, $errors).is_some(),
            take_log!(Token::Colon, $span, $tokens, $errors).is_some(),
            take_log!(Token::Ident(_), $span, $tokens, $errors),
        ) {
            (true, true, image_handle) => image_handle,
            _ => None
        }
    }}
}

#[macro_export]
macro_rules! parse_line_height {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        match (
            take_log!(Token::Prop(PropKw::LineHeight), $span, $tokens, $errors).is_some(),
            take_log!(Token::Colon, $span, $tokens, $errors).is_some(),
            take_log!(Token::Float(_), $span, $tokens, $errors),
        ) {
            (true, true, Some(line_height)) => error_log!(ASTLineHeight::spanned_parse(line_height, $span), $errors),
            _ => None
        }
    }}
}

#[macro_export]
macro_rules! parse_letter_spacing {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        match (
            take_log!(Token::Prop(PropKw::LetterSpacing), $span, $tokens, $errors).is_some(),
            take_log!(Token::Colon, $span, $tokens, $errors).is_some(),
            take_log!(Token::Float(_), $span, $tokens, $errors),
        ) {
            (true, true, Some(letter_spacing)) => error_log!(ASTLetterSpacing::spanned_parse(letter_spacing, $span), $errors),
            _ => None
        }
    }}
}

#[macro_export]
macro_rules! parse_align {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        match (
            take_log!(Token::Prop(PropKw::Align), $span, $tokens, $errors).is_some(),
            take_log!(Token::Colon, $span, $tokens, $errors).is_some(),
            take_log!(Token::Variant(_), $span, $tokens, $errors),
        ) {
            (true, true, align) => align,
            _ => None
        }
    }}
}

#[macro_export]
macro_rules! parse_padding {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        match (
            take_log!(Token::Prop(PropKw::Padding), $span, $tokens, $errors).is_some(),
            take_log!(Token::Colon, $span, $tokens, $errors).is_some(),
            take_log!(Token::Float(_), $span, $tokens, $errors),
        ) {
            (true, true, padding) => padding,
            _ => None
        }
    }}
}

#[macro_export]
macro_rules! parse_text {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        match (
            take_log!(Token::Prop(PropKw::Text), $span, $tokens, $errors).is_some(),
            take_log!(Token::Colon, $span, $tokens, $errors).is_some(),
            take_log!(Token::Str(_), $span, $tokens, $errors),
        ) {
            (true, true, text) => text,
            _ => None
        }
    }}
}

#[macro_export]
macro_rules! parse_clip {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        match (
            take_log!(Token::Prop(PropKw::Clip), $span, $tokens, $errors).is_some(),
            take_log!(Token::Colon, $span, $tokens, $errors).is_some(),
            take_log!(Token::Bool(_), $span, $tokens, $errors),
        ) {
            (true, true, clip) => clip,
            _ => None
        }
    }}
}

#[macro_export]
macro_rules! parse_opacity {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        match (
            take_log!(Token::Prop(PropKw::Opacity), $span, $tokens, $errors).is_some(),
            take_log!(Token::Colon, $span, $tokens, $errors).is_some(),
            take_log!(Token::Uint(_), $span, $tokens, $errors),
        ) {
            (true, true, opacity) => opacity,
            _ => None
        }
    }}
}


#[macro_export]
macro_rules! parse_size {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        match (
            take_log!(Token::Prop(PropKw::Size), $span, $tokens, $errors).is_some(),
            take_log!(Token::Colon, $span, $tokens, $errors).is_some(),
            take_log!(Token::LParen, $span, $tokens, $errors).is_some(),
            take_log!(Token::Float(_), $span, $tokens, $errors),
            take_log!(Token::Comma, $span, $tokens, $errors).is_some(),
            take_log!(Token::Float(_), $span, $tokens, $errors),
            take_log!(Token::RParen, $span, $tokens, $errors).is_some()
        ) {
            (true, true, true, Some(widht), true, Some(height), true) => error_log!(ASTSize::spanned_parse((widht, height), $span), $errors),
            _ => None
        }
    }}
}

#[macro_export]
macro_rules! parse_scale {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        match (
            take_log!(Token::Prop(PropKw::Scale), $span, $tokens, $errors).is_some(),
            take_log!(Token::Colon, $span, $tokens, $errors).is_some(),
            take_log!(Token::LParen, $span, $tokens, $errors).is_some(),
            take_log!(Token::Float(_), $span, $tokens, $errors),
            take_log!(Token::Comma, $span, $tokens, $errors).is_some(),
            take_log!(Token::Float(_), $span, $tokens, $errors),
            take_log!(Token::RParen, $span, $tokens, $errors).is_some(),
        ) {
            (true, true, true, Some(x), true, Some(y), true) => error_log!(ASTScale::spanned_parse((x, y), $span), $errors),
            _ => None
        }
    }}
}

#[macro_export]
macro_rules! parse_transform {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        match (
            take_log!(Token::Prop(PropKw::Transform), $span, $tokens, $errors).is_some(),
            take_log!(Token::Colon, $span, $tokens, $errors).is_some(),
            take_log!(Token::LParen, $span, $tokens, $errors).is_some(),
            take_log!(Token::Float(_), $span, $tokens, $errors),
            take_log!(Token::Comma, $span, $tokens, $errors).is_some(),
            take_log!(Token::Float(_), $span, $tokens, $errors),
            take_log!(Token::Comma, $span, $tokens, $errors).is_some(),
            take_log!(Token::Float(_), $span, $tokens, $errors),
            take_log!(Token::RParen, $span, $tokens, $errors).is_some(),
        ) {
            (true, true, true, Some(x), true, Some(y), true, Some(z), true) => error_log!(ASTTransform::spanned_parse((x, y, z), $span), $errors),
            _ => None
        }
    }}
}

#[macro_export]
macro_rules! parse_parent {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        match take!(Token::Prop(PropKw::Parent), $span, $tokens).is_ok() {
            true => match (
                take_log!(Token::Colon, $span, $tokens, $errors).is_some(),
                take_log!(Token::Ident(_), $span, $tokens, $errors)
            ) {
                (true, ident) => ident,
                _ => None
            },
            false => None
        }
    }}
}

#[macro_export]
macro_rules! take {
    ($target:pat, $span:expr, $tokens:expr) => {{
        let tokens = &mut $tokens;
        match tokens.peek() {
            Some(spanned) => {
                match spanned.token {
                    $target => {
                        let spanned = (*spanned).clone();
                        tokens.next();
                        let span = &mut $span;
                        span.set(spanned.span);
                        Ok(spanned)
                    },
                    _ => Err(LumaError::UnexpectedToken { spanned: (*spanned).clone() }),
                }
            },
            None => Err(LumaError::UnexpectedEof { span: $span })
        }
    }};
}

#[macro_export]
macro_rules! open_brace {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        take_log!(Token::LBrace, $span, $tokens, $errors).is_some()
    }}
}

#[macro_export]
macro_rules! close_brace {
    ($span:expr, $tokens:expr, $errors:expr) => {{
        let result = take_log!(Token::RBrace, $span, $tokens, $errors).is_some();
        _ = take!(Token::Semicolon, $span, $tokens);
        result
    }}
}

#[macro_export]
macro_rules! take_log {
    ($target:pat, $span:expr, $tokens:expr, $errors:expr) => {{
        error_log!(take!($target, $span, $tokens), $errors)
    }};
}

#[macro_export]
macro_rules! error_log {
    ($value:expr, $errors:expr) => {
        match $value {
            Ok(value) => Some(value),
            Err(err) => {
                $errors.push(err);
                None
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, New)]
pub struct ASTLayer {
    pub ident: Spanned,
    pub node: Vec<ASTNode>,
    pub label: Vec<ASTLabel>,
    pub container: Vec<ASTContainer>,
    pub image: Vec<ASTImage>,
    pub hit: ASTHitLayer
}

#[derive(Debug, Clone, PartialEq, Make)]
pub struct ASTHitLayer {
    #[None] pub vec: Option<ASTHitBlock>,
    #[None] pub map: Option<ASTHitBlock>
}

#[derive(Debug, Clone, PartialEq, Make)]
pub struct ASTHitBlock {
    pub ty: Spanned,
    #[Some(Vec::new())] pub entries: Vec<ASTHitEntry>,
}

#[derive(Debug, Clone, PartialEq, New)]
pub struct ASTHitEntry {
    pub hit: Vec<Spanned>,
    pub ident: Spanned
}

#[derive(Debug, Clone, PartialEq, New)]
pub struct ASTImage {
    pub node: ASTNode,
    pub image_handle: Spanned,
    pub visual_mode: ASTVisualMode,
}

#[derive(Debug, Clone, PartialEq, New)]
pub struct ASTLabel {
    pub node: ASTNode,
    pub rotate: ASTRotate,
    pub text_style: ASTTextStyle,
    pub color: ASTColor,
    pub visual_mode: ASTVisualMode,
    pub is_dynamic: bool
}

#[derive(Debug, Clone, PartialEq, New)]
pub struct ASTTextStyle {
    pub ident: Option<Spanned>,
    pub text: Spanned,
    pub font_size: ASTFontSize,
    pub color: ASTColor,
    pub line_height: ASTLineHeight,
    pub letter_spacing: ASTLetterSpacing,
    pub align: Spanned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ASTFontSize {
    #[default] Default,
    New(usize)
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ASTLineHeight {
    #[default] Default,
    New(f32)
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ASTLetterSpacing {
    #[default] Default,
    New(f32)
}

#[derive(Debug, Clone, Copy, PartialEq, New)]
pub struct ASTTransform {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Default for ASTTransform {
    fn default() -> Self {
        let transform = agnosticlayer::base_data::Transform::default();
        Self::new(transform.x, transform.y, transform.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, New)]
pub struct ASTSize {
    pub width: f32,
    pub height: f32
}

impl Default for ASTSize {
    fn default() -> Self {
        let size = agnosticlayer::base_data::Size::default();
        Self::new(size.width, size.height)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, New)]
pub struct ASTScale {
    pub x: f32,
    pub y: f32
}

impl Default for ASTScale {
    fn default() -> Self {
        let scale = agnosticlayer::base_data::Scale::default();
        Self::new(scale.x, scale.y)
    }
}

#[derive(Debug, Clone, PartialEq, New)]
pub struct ASTNode {
    pub ident: Spanned,
    pub size: ASTSize,
    pub scale: ASTScale,
    pub transform: ASTTransform,
    pub parent: Option<Spanned>
}

#[derive(Debug, Clone, PartialEq, New)]
pub struct ASTContainer {
    pub node: ASTNode,
    pub rotate: ASTRotate,
    pub container_style: ASTContainerStyle,
    pub color: ASTColor,
    pub visual_mode: ASTVisualMode
}

#[derive(Debug, Clone, PartialEq, New)]
pub struct ASTRotate {
    pub rotate: f32,
    pub anchor: Spanned
}

impl Default for ASTRotate {
    fn default() -> Self {
        let rotate = agnosticlayer::base_data::Rotate::default();
        Self::new(rotate.rotate, Spanned::new(Token::Variant(VariantKw::default()), Span::default()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, New)]
pub struct ASTColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Default for ASTColor {
    fn default() -> Self {
        let color = agnosticlayer::base_data::Color::default();
        Self::new(color.r, color.g, color.b, color.a)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, New)]
pub struct ASTVisualMode {
    pub visible: bool,
    pub opacity: u8
}

impl Default for ASTVisualMode {
    fn default() -> Self {
        let visual_mode = agnosticlayer::base_data::VisualMode::default();
        Self::new(visual_mode.visible, visual_mode.opacity)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, New)]
pub struct ASTContainerStyle {
    pub padding: f32,
    pub clip: bool
}

impl Default for ASTContainerStyle {
    fn default() -> Self {
        let container_style = agnosticlayer::container::ContainerStyle::default();
        Self::new(container_style.padding, container_style.clip)
    }
}


impl ASTSize {
    pub fn spanned_parse(size: (Spanned, Spanned), span: Span) -> LumaResult<Self> {
        match (size.0.token, size.1.token) {
            (Token::Float(width), Token::Float(height)) => Ok(Self::new(width, height)),
            _ => Err(LumaError::ASTParseError { span })
        }
    }
}


impl ASTFontSize {
    pub fn spanned_parse(size: Spanned, span: Span) -> LumaResult<Self> {
        match size.token {
            Token::Uint(size) => Ok(Self::New(size)),
            _ => Err(LumaError::ASTParseError { span })
        }
    }
}

impl ASTLineHeight {
    pub fn spanned_parse(height: Spanned, span: Span) -> LumaResult<Self> {
        match height.token {
            Token::Float(height) => Ok(Self::New(height)),
            _ => Err(LumaError::ASTParseError { span })
        }
    }
}

impl ASTLetterSpacing {
    pub fn spanned_parse(spacing: Spanned, span: Span) -> LumaResult<Self> {
        match spacing.token {
            Token::Float(spacing) => Ok(Self::New(spacing)),
            _ => Err(LumaError::ASTParseError { span })
        }
    }
}

impl ASTTransform {
    pub fn spanned_parse(transform: (Spanned, Spanned, Spanned), span: Span) -> LumaResult<Self> {
        match (transform.0.token, transform.1.token, transform.2.token) {
            (Token::Float(x), Token::Float(y), Token::Float(z)) => Ok(Self::new(x, y, z)),
            _ => Err(LumaError::ASTParseError { span })
        }
    }
}

impl ASTScale {
    pub fn spanned_parse(scale: (Spanned, Spanned), span: Span) -> LumaResult<Self> {
        match (scale.0.token, scale.1.token) {
            (Token::Float(x), Token::Float(y)) => Ok(Self::new(x, y)),
            _ => Err(LumaError::ASTParseError { span })
        }
    }
}

impl ASTRotate {
    pub fn spanned_parse(rotate: Spanned, anchor: Spanned, span: Span) -> LumaResult<Self> {
        match rotate.token {
            Token::Float(rotate) => Ok(Self::new(rotate, anchor)),
            _ => Err(LumaError::ASTParseError { span })
        }
    }
}

impl ASTColor {
    pub fn spanned_parse(color: (Spanned, Spanned, Spanned, Spanned), span: Span) -> LumaResult<Self> {
        match (color.0.token, color.1.token, color.2.token, color.3.token) {
            (Token::Uint(r), Token::Uint(g), Token::Uint(b), Token::Uint(a)) => {
                let r = u8::try_from(r).map_err(|_| LumaError::ColorOutOfRange { value: r, span: color.0.span })?;
                let g = u8::try_from(g).map_err(|_| LumaError::ColorOutOfRange { value: g, span: color.1.span })?;
                let b = u8::try_from(b).map_err(|_| LumaError::ColorOutOfRange { value: b, span: color.2.span })?;
                let a = u8::try_from(a).map_err(|_| LumaError::ColorOutOfRange { value: a, span: color.3.span })?;
                Ok(Self::new(r, g, b, a))
            }
            _ => Err(LumaError::ASTParseError { span })
        }
    }
}

impl ASTVisualMode {
    pub fn visible_spanned_parse(&mut self, visible: Spanned) -> LumaResult<()> {
        match visible.token {
            Token::Bool(visible) => {
                self.visible = visible;
                Ok(())
            },
            _ => Err(LumaError::ASTParseError { span: visible.span })
        }
    }

    pub fn opacity_spanned_parse(&mut self, opacity: Spanned) -> LumaResult<()> {
        match opacity.token {
            Token::Uint(op) => {
                let opacity = u8::try_from(op).map_err(|_| LumaError::OpacityOutOfRange { value: op, span: opacity.span })?;
                self.opacity = opacity;
                Ok(())
            },
            _ => Err(LumaError::ASTParseError { span: opacity.span })
        }
    }
}

impl ASTContainerStyle {
    pub fn padding_spanned_parse(&mut self, padding: Spanned) -> LumaResult<()> {
        match padding.token {
            Token::Float(padding) => {
                self.padding = padding;
                Ok(())
            },
            _ => Err(LumaError::ASTParseError { span: padding.span })
        }
    }

    pub fn clip_spanned_parse(&mut self, clip: Spanned) -> LumaResult<()> {
        match clip.token {
            Token::Bool(clip) => {
                self.clip = clip;
                Ok(())
            },
            _ => Err(LumaError::ASTParseError { span: clip.span })
        }
    }
}