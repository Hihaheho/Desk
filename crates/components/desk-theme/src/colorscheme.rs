use std::collections::BTreeMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodeColorTag {
    Variable,
    Comment,
    CommentDelimiter,
    CommentPrefix,
    DsonComment,
    DsonCommentDelimiter,
    DsonCommentPrefix,
    Uuid,
    RealLiteral,
    RationalLiteral,
    IntegerLiteral,
    StringLiteral,
    StringEscape,
    StringDelimiter,
    DsonRealLiteral,
    DsonRationalLiteral,
    DsonIntegerLiteral,
    DsonStringLiteral,
    DsonStringEscape,
    DsonStringDelimiter,
    Arrow,
    FArrow,
    EArrow,
    Continue,
    ControlDelimiter,
    ExprDelimiter,
    Comma,
    DsonComma,
    StmtEnd,
    TyAnno,
    Hole,
    MapDelimiter,
    VecDelimiter,
    TypeDelimiter,
    DsonMapDelimiter,
    DsonVecDelimiter,
    DsonTypeDelimiter,
    ParamsDelimiter,
    Label,
    DsonLabel,
    Attribute,
    DsonAttribute,
    Infer,
    Perform,
    Reference,
    Apply,
    Product,
    Sum,
    DsonProduct,
    DsonSum,
    Minus,
    Let,
    TyLet,
    Function,
    Do,
    NewType,
    Pi,
    Sigma,
    Card,
    NewBrand,
    TyString,
    TyReal,
    TyRational,
    TyInteger,
    DsonTyString,
    DsonTyReal,
    DsonTyRational,
    DsonTyInteger,
    Handle,
    Match,
    Version,
}

// TODO: derive Reflect
#[derive(Serialize, Deserialize, Clone)]
pub struct CodeColorScheme {
    default_color: Color,
    palette: BTreeMap<CodeColorTag, Color>,
}

impl Default for CodeColorScheme {
    fn default() -> Self {
        Self {
            default_color: Color::BLACK,
            palette: BTreeMap::new(),
        }
    }
}

impl CodeColorScheme {
    pub fn set_default(&mut self, color: Color) {
        self.default_color = color;
    }

    pub fn set(&mut self, tag: CodeColorTag, color: Color) {
        self.palette.insert(tag, color);
    }

    pub fn get(&self, tag: CodeColorTag) -> Color {
        self.palette
            .get(&tag)
            .copied()
            .unwrap_or(self.default_color)
    }

    pub fn set_punct(&mut self, color: Color) {
        self.set(CodeColorTag::Arrow, color);
        self.set(CodeColorTag::FArrow, color);
        self.set(CodeColorTag::EArrow, color);
        self.set(CodeColorTag::Comma, color);
        self.set(CodeColorTag::StmtEnd, color);
        self.set(CodeColorTag::TyAnno, color);
        self.set(CodeColorTag::DsonComma, color);
    }

    pub fn set_comment(&mut self, color: Color) {
        self.set(CodeColorTag::Comment, color);
        self.set(CodeColorTag::CommentDelimiter, color);
        self.set(CodeColorTag::CommentPrefix, color);
    }

    pub fn set_string(&mut self, color: Color) {
        self.set(CodeColorTag::StringDelimiter, color);
        self.set(CodeColorTag::StringLiteral, color);
        self.set(CodeColorTag::DsonStringDelimiter, color);
        self.set(CodeColorTag::DsonStringLiteral, color);
        // Do not include `self.set(ColorTag::StringEscape, color);` here
    }

    pub fn set_number(&mut self, color: Color) {
        self.set(CodeColorTag::RealLiteral, color);
        self.set(CodeColorTag::RationalLiteral, color);
        self.set(CodeColorTag::IntegerLiteral, color);
    }

    pub fn set_symbol(&mut self, color: Color) {
        self.set(CodeColorTag::Attribute, color);
        self.set(CodeColorTag::Label, color);
        self.set(CodeColorTag::Pi, color);
        self.set(CodeColorTag::Sigma, color);
        self.set(CodeColorTag::Function, color);
        self.set(CodeColorTag::Perform, color);
        self.set(CodeColorTag::Continue, color);
        self.set(CodeColorTag::Hole, color);
        self.set(CodeColorTag::Infer, color);
    }

    pub fn set_declaration(&mut self, color: Color) {
        self.set(CodeColorTag::NewType, color);
        self.set(CodeColorTag::Card, color);
        self.set(CodeColorTag::NewBrand, color);
        self.set(CodeColorTag::Version, color);
        self.set(CodeColorTag::Version, color);
    }

    pub fn set_control(&mut self, color: Color) {
        self.set(CodeColorTag::Handle, color);
        self.set(CodeColorTag::Match, color);
        self.set(CodeColorTag::Let, color);
        self.set(CodeColorTag::TyLet, color);
        self.set(CodeColorTag::Do, color);
    }

    pub fn set_operator(&mut self, color: Color) {
        self.set(CodeColorTag::Minus, color);
        self.set(CodeColorTag::Apply, color);
        self.set(CodeColorTag::Product, color);
        self.set(CodeColorTag::Sum, color);
        self.set(CodeColorTag::Reference, color);
        self.set(CodeColorTag::DsonProduct, color);
        self.set(CodeColorTag::DsonSum, color);
    }

    pub fn set_delimiter(&mut self, color: Color) {
        self.set(CodeColorTag::ControlDelimiter, color);
        self.set(CodeColorTag::ExprDelimiter, color);
        self.set(CodeColorTag::MapDelimiter, color);
        self.set(CodeColorTag::VecDelimiter, color);
        self.set(CodeColorTag::TypeDelimiter, color);
        self.set(CodeColorTag::ParamsDelimiter, color);
        self.set(CodeColorTag::DsonMapDelimiter, color);
        self.set(CodeColorTag::DsonVecDelimiter, color);
        self.set(CodeColorTag::DsonTypeDelimiter, color);
    }

    pub fn set_type(&mut self, color: Color) {
        self.set(CodeColorTag::TyString, color);
        self.set(CodeColorTag::TyReal, color);
        self.set(CodeColorTag::TyRational, color);
        self.set(CodeColorTag::TyInteger, color);
        self.set(CodeColorTag::DsonTyString, color);
        self.set(CodeColorTag::DsonTyReal, color);
        self.set(CodeColorTag::DsonTyRational, color);
        self.set(CodeColorTag::DsonTyInteger, color);
    }
}
