use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::Type;

/// The syntactic shapes that values must match to be passed into a command. You can think of this as the type-checking that occurs when you call a function.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyntaxShape {
    /// A specific match to a word or symbol
    Keyword(Vec<u8>, Box<SyntaxShape>),

    /// Any syntactic form is allowed
    Any,

    /// Strings and string-like bare words are allowed
    String,

    /// A dotted path to navigate the table
    CellPath,

    /// A dotted path to navigate the table (including variable)
    FullCellPath,

    /// Only a numeric (integer or decimal) value is allowed
    Number,

    /// A range is allowed (eg, `1..3`)
    Range,

    /// Only an integer value is allowed
    Int,

    /// A filepath is allowed
    Filepath,

    /// A glob pattern is allowed, eg `foo*`
    GlobPattern,

    /// A module path pattern used for imports
    ImportPattern,

    /// A binary literal
    Binary,

    /// A block is allowed, eg `{start this thing}`
    Block(Option<Vec<SyntaxShape>>),

    /// A table is allowed, eg `[[first, second]; [1, 2]]`
    Table,

    /// A table is allowed, eg `[first second]`
    List(Box<SyntaxShape>),

    /// A filesize value is allowed, eg `10kb`
    Filesize,

    /// A duration value is allowed, eg `19day`
    Duration,

    /// A datetime value, eg `2022-02-02` or `2019-10-12T07:20:50.52+00:00`
    DateTime,

    /// An operator
    Operator,

    /// A math expression which expands shorthand forms on the lefthand side, eg `foo > 1`
    /// The shorthand allows us to more easily reach columns inside of the row being passed in
    RowCondition,

    /// A general math expression, eg `1 + 2`
    MathExpression,

    /// A variable name
    Variable,

    /// A variable with optional type, `x` or `x: int`
    VarWithOptType,

    /// A signature for a definition, `[x:int, --foo]`
    Signature,

    /// A general expression, eg `1 + 2` or `foo --bar`
    Expression,

    /// A boolean value
    Boolean,

    /// A record value
    Record,

    /// A custom shape with custom completion logic
    Custom(Box<SyntaxShape>, String),
}

impl SyntaxShape {
    pub fn to_type(&self) -> Type {
        match self {
            SyntaxShape::Any => Type::Unknown,
            SyntaxShape::Block(_) => Type::Block,
            SyntaxShape::Binary => Type::Binary,
            SyntaxShape::CellPath => Type::Unknown,
            SyntaxShape::Custom(custom, _) => custom.to_type(),
            SyntaxShape::DateTime => Type::Date,
            SyntaxShape::Duration => Type::Duration,
            SyntaxShape::Expression => Type::Unknown,
            SyntaxShape::Filepath => Type::String,
            SyntaxShape::Filesize => Type::Filesize,
            SyntaxShape::FullCellPath => Type::Unknown,
            SyntaxShape::GlobPattern => Type::String,
            SyntaxShape::ImportPattern => Type::Unknown,
            SyntaxShape::Int => Type::Int,
            SyntaxShape::List(x) => {
                let contents = x.to_type();
                Type::List(Box::new(contents))
            }
            SyntaxShape::Keyword(_, expr) => expr.to_type(),
            SyntaxShape::MathExpression => Type::Unknown,
            SyntaxShape::Number => Type::Number,
            SyntaxShape::Operator => Type::Unknown,
            SyntaxShape::Range => Type::Unknown,
            SyntaxShape::Record => Type::Record(vec![]), // FIXME: Add actual record type
            SyntaxShape::RowCondition => Type::Bool,
            SyntaxShape::Boolean => Type::Bool,
            SyntaxShape::Signature => Type::Signature,
            SyntaxShape::String => Type::String,
            SyntaxShape::Table => Type::List(Box::new(Type::Unknown)), // FIXME: Tables should have better types
            SyntaxShape::VarWithOptType => Type::Unknown,
            SyntaxShape::Variable => Type::Unknown,
        }
    }
}

impl Display for SyntaxShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxShape::Keyword(kw, shape) => {
                write!(f, "\"{}\" {}", String::from_utf8_lossy(kw), shape)
            }
            SyntaxShape::Any => write!(f, "any"),
            SyntaxShape::String => write!(f, "string"),
            SyntaxShape::CellPath => write!(f, "cellpath"),
            SyntaxShape::FullCellPath => write!(f, "cellpath"),
            SyntaxShape::Number => write!(f, "number"),
            SyntaxShape::Range => write!(f, "range"),
            SyntaxShape::Int => write!(f, "int"),
            SyntaxShape::Filepath => write!(f, "path"),
            SyntaxShape::GlobPattern => write!(f, "glob"),
            SyntaxShape::ImportPattern => write!(f, "import"),
            SyntaxShape::Block(_) => write!(f, "block"),
            SyntaxShape::Binary => write!(f, "binary"),
            SyntaxShape::Table => write!(f, "table"),
            SyntaxShape::List(x) => write!(f, "list<{}>", x),
            SyntaxShape::Record => write!(f, "record"),
            SyntaxShape::Filesize => write!(f, "filesize"),
            SyntaxShape::Duration => write!(f, "duration"),
            SyntaxShape::DateTime => write!(f, "datetime"),
            SyntaxShape::Operator => write!(f, "operator"),
            SyntaxShape::RowCondition => write!(f, "condition"),
            SyntaxShape::MathExpression => write!(f, "variable"),
            SyntaxShape::Variable => write!(f, "var"),
            SyntaxShape::VarWithOptType => write!(f, "vardecl"),
            SyntaxShape::Signature => write!(f, "signature"),
            SyntaxShape::Expression => write!(f, "expression"),
            SyntaxShape::Boolean => write!(f, "bool"),
            SyntaxShape::Custom(x, _) => write!(f, "custom<{}>", x),
        }
    }
}
