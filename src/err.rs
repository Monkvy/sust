

/// Create an error with the type of [Kind::Undefined] & the given message.
#[track_caller]
pub fn new(msg: String) -> Error {
    let l = std::panic::Location::caller();
    Error { kind: Kind::Undefined, msg: format!("\"{}\" at file: \"{}\", line: {}", msg, l.file(), l.line()) }
}


#[derive(Debug, PartialEq)]
pub enum Kind {
    Undefined,
}
impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


pub struct Error { 
    pub kind: Kind,
    pub msg: String
}
impl std::error::Error for Error { }
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind.to_string(), self.msg)
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
