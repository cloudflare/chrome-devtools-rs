mod bigint;
mod boolean;
mod function;
mod number;
mod object;
mod string;
mod symbol;

use bigint::BigIntData;
use boolean::BooleanData;
use function::FunctionData;
use number::NumberData;
use object::ObjectData;
use string::StringData;
use symbol::SymbolData;

use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum JsPrimitive {
    Object(ObjectData),
    Number(NumberData),
    BigInt(BigIntData),
    Boolean(BooleanData),
    #[serde(rename = "string")]
    JsString(StringData),
    Symbol(SymbolData),
    Undefined,
    Function(FunctionData),
}

impl fmt::Display for JsPrimitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            JsPrimitive::Object(object) => write!(f, "{}", object),
            JsPrimitive::Boolean(boolean) => write!(f, "{}", boolean),
            JsPrimitive::JsString(string) => write!(f, "{}", string),
            JsPrimitive::Undefined => write!(f, "undefined"),
            JsPrimitive::Function(function) => write!(f, "{}", function),
            JsPrimitive::Number(number) => write!(f, "{}", number),
            JsPrimitive::Symbol(symbol) => write!(f, "{}", symbol),
            JsPrimitive::BigInt(bigint) => write!(f, "{}", bigint),
        }
    }
}
