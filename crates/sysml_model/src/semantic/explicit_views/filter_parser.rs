use super::*;

#[derive(Debug, Clone)]
pub(crate) enum FilterToken {
    At(String),
    Not,
    And,
    Or,
    LParen,
    RParen,
    Unknown(String),
}

pub(crate) fn tokenize_filter(text: &str) -> Vec<FilterToken> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = text.trim().chars().collect();
    let mut index = 0;
    while index < chars.len() {
        match chars[index] {
            ' ' | '\t' | '\r' | '\n' => index += 1,
            '(' => {
                tokens.push(FilterToken::LParen);
                index += 1;
            }
            ')' => {
                tokens.push(FilterToken::RParen);
                index += 1;
            }
            '@' => {
                let start = index;
                index += 1;
                while index < chars.len()
                    && (chars[index].is_alphanumeric()
                        || matches!(chars[index], '_' | ':' | '.' | '\''))
                {
                    index += 1;
                }
                tokens.push(FilterToken::At(chars[start + 1..index].iter().collect()));
            }
            _ => {
                let start = index;
                while index < chars.len()
                    && !chars[index].is_whitespace()
                    && !matches!(chars[index], '(' | ')')
                {
                    index += 1;
                }
                let word: String = chars[start..index].iter().collect();
                let normalized = word.to_lowercase();
                tokens.push(match normalized.as_str() {
                    "not" => FilterToken::Not,
                    "and" => FilterToken::And,
                    "or" => FilterToken::Or,
                    _ => FilterToken::Unknown(word),
                });
            }
        }
    }
    tokens
}

pub(crate) struct FilterParser {
    pub(crate) tokens: Vec<FilterToken>,
    pub(crate) index: usize,
}

impl FilterParser {
    pub(crate) fn parse_expr(&mut self) -> FilterExpr {
        self.parse_or()
    }

    pub(crate) fn parse_or(&mut self) -> FilterExpr {
        let mut expr = self.parse_and();
        while self.matches(|token| matches!(token, FilterToken::Or)) {
            let rhs = self.parse_and();
            expr = FilterExpr::Or(Box::new(expr), Box::new(rhs));
        }
        expr
    }

    pub(crate) fn parse_and(&mut self) -> FilterExpr {
        let mut expr = self.parse_unary();
        while self.matches(|token| matches!(token, FilterToken::And)) {
            let rhs = self.parse_unary();
            expr = FilterExpr::And(Box::new(expr), Box::new(rhs));
        }
        expr
    }

    pub(crate) fn parse_unary(&mut self) -> FilterExpr {
        if self.matches(|token| matches!(token, FilterToken::Not)) {
            return FilterExpr::Not(Box::new(self.parse_unary()));
        }
        self.parse_primary()
    }

    pub(crate) fn parse_primary(&mut self) -> FilterExpr {
        match self.peek().cloned() {
            Some(FilterToken::At(value)) => {
                self.index += 1;
                FilterExpr::Matches(value)
            }
            Some(FilterToken::LParen) => {
                self.index += 1;
                let expr = self.parse_expr();
                if self.matches(|token| matches!(token, FilterToken::RParen)) {
                    expr
                } else {
                    FilterExpr::Unsupported("missing ')' in filter expression".to_string())
                }
            }
            Some(FilterToken::Unknown(text)) => {
                self.index += 1;
                FilterExpr::Unsupported(text)
            }
            _ => FilterExpr::Unsupported("empty filter expression".to_string()),
        }
    }

    pub(crate) fn matches(&mut self, predicate: impl FnOnce(&FilterToken) -> bool) -> bool {
        if let Some(token) = self.peek() {
            if predicate(token) {
                self.index += 1;
                return true;
            }
        }
        false
    }

    pub(crate) fn peek(&self) -> Option<&FilterToken> {
        self.tokens.get(self.index)
    }
}
