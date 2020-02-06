use std::fmt;
use std::any::Any;

//////////////////////////////////////////////////////////////////////
/// Token Common
//////////////////////////////////////////////////////////////////////

pub trait Token {
    fn to_string(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

impl fmt::Display for dyn Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for dyn Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
