

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableType {
    Undefined,
    Null,
    Boolean,
    Number,
    String,
    Array,
    Object,
}