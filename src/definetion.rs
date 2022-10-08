use std::collections::HashMap;
pub enum TokenType {
    Eof,
    Name,
    DataType,
    Ignore,
    Variable,
    StmtEnd,
    Equal,
    None,
}
