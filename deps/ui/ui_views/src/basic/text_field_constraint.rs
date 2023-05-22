use std::fmt::Debug;

use reflected::{Field, Type};
use ui::ToLabel;

#[derive(Debug)]
pub(crate) enum TextFieldConstraint {
    Integer,
    Float,
}

impl TextFieldConstraint {
    pub(crate) fn from_field(field: &Field) -> Option<Self> {
        if matches!(field.tp, Type::Integer) {
            Self::Integer.into()
        } else if matches!(field.tp, Type::Float) {
            Self::Float.into()
        } else {
            None
        }
    }

    pub(crate) fn filter(&self, string: impl ToLabel) -> String {
        let string = string.to_label();
        let symbols = self.accepted_symbols(&string);
        string.to_string().chars().filter(|c| symbols.contains(*c)).collect()
    }

    fn accept_char(&self, char: char, string: &str) -> bool {
        self.accepted_symbols(string).contains(char)
    }

    fn accepted_symbols(&self, _str: &str) -> &str {
        match self {
            Self::Integer => "-0123456789",
            Self::Float => "-0.123456789",
        }
    }
}

pub(crate) trait AcceptChar {
    fn accept_char(&self, char: char, string: &str) -> bool;
}

impl AcceptChar for Option<TextFieldConstraint> {
    fn accept_char(&self, char: char, string: &str) -> bool {
        match self {
            Some(constraint) => constraint.accept_char(char, string),
            None => true,
        }
    }
}
