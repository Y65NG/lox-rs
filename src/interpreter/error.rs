use crate::lexer::Token;
use crate::error::Error;

pub struct RuntimeError(pub Token, pub String);

impl Error for RuntimeError {}
