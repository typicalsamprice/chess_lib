#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GenType {
    Captures,
    Evasions,
    NonEvasions,
    Quiet,
    QuietChecks
}