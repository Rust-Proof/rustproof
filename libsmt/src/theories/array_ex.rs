//! Module that describes the ArrayEx Theory

use std::fmt;

use backends::backend::SMTNode;

#[derive(Clone, Debug)]
pub enum OpCodes<X, Y, Z>
    where X: fmt::Debug + fmt::Display + Clone,
          Y: fmt::Debug + fmt::Display + Clone,
          Z: fmt::Debug + fmt::Display + Clone
{
    Select,
    Store,
    FreeVar(String),
    Const(Sorts<X, Y>, Box<Z>),
}

impl<X, Y, Z> fmt::Display for OpCodes<X, Y, Z>
where X: fmt::Debug + fmt::Display + Clone,
      Y: fmt::Debug + fmt::Display + Clone,
      Z: fmt::Debug + fmt::Display + Clone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            OpCodes::Select => "select".to_owned(),
            OpCodes::Store => "store".to_owned(),
            OpCodes::FreeVar(ref s) => s.clone(),
            OpCodes::Const(ref arr, ref val) => format!("((as const {}) {})", arr, val),
        };
        write!(f, "{}", s)
    }
}

impl<X, Y, Z> SMTNode for OpCodes<X, Y, Z>
where X: fmt::Debug + fmt::Display + Clone,
      Z: fmt::Debug + fmt::Display + Clone,
      Y: fmt::Debug + fmt::Display + Clone {

    fn is_var(&self) -> bool {
        if let OpCodes::FreeVar(_) = *self {
            true
        } else {
            false
        }
    }

    fn is_const(&self) -> bool {
        match *self {
            OpCodes::Const(_, _) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Sorts<X, Y>
    where X: fmt::Display + fmt::Debug + Clone,
          Y: fmt::Display + fmt::Debug + Clone
{
    Array(Box<X>, Box<Y>),
}

impl<X, Y> fmt::Display for Sorts<X, Y>
    where X: fmt::Display + fmt::Debug + Clone,
          Y: fmt::Display + fmt::Debug + Clone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Sorts::Array(ref x, ref y) => format!("(Array {} {})", x, y),
        };
        write!(f, "{}", s)
    }
}

impl<X, Y> Sorts<X, Y>
where X: fmt::Debug + fmt::Display + Clone,
      Y: fmt::Debug + fmt::Display + Clone
{
    pub fn new(idx: X, ty: Y) -> Sorts<X, Y> {
        Sorts::Array(Box::new(idx), Box::new(ty))
    }
}
