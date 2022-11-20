use chrono::{DateTime, Utc};
use core::fmt::{self, Debug};
use std::time::SystemTime;

use crate::{error::ShellError, views::baseview::BaseView, parselex::parser};

use super::{datadict::DataDict, descriptor::Descriptor};

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
pub enum Primitive {
    String(String),
    Integer(i64),
    Time(SystemTime),
    Size(u64),
    None,
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Primitive::String(s) => write!(f, "{}", s),
            Primitive::Integer(i) => write!(f, "{}", i),
            Primitive::Time(_) => todo!(),
            Primitive::Size(_) => todo!(),
            Primitive::None => todo!(),
        }
    }
}

impl Primitive {
    pub fn format(&self) -> String {
        match self {
            Primitive::String(s) => s.to_string(),
            Primitive::Integer(i) => i.to_string(),
            Primitive::Time(t) => {
                let as_utc: DateTime<Utc> = (*t).into();
                as_utc.date_naive().to_string()
            }
            Primitive::Size(bytes) => {
                let kilobytes = (*bytes as f32) / 1024.0;
                let megabytes = kilobytes / 1024.0;
                let gigabytes = megabytes / 1024.0;

                if gigabytes >= 1.0 {
                    format!("{:.2} GB", gigabytes)
                } else if megabytes >= 1.0 {
                    format!("{:.2} MB", megabytes)
                } else if kilobytes >= 1.0 {
                    format!("{:.2} KB", kilobytes)
                } else {
                    format!("{:.2} bytes", bytes)
                }
            }
            Primitive::None => "".to_string(),
        }
    }
}

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
pub enum Value {
    Object(DataDict),
    List(Vec<Value>),
    Primitive(Primitive),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Object(_) => todo!(),
            Value::List(_) => todo!(),
            Value::Primitive(p) => write!(f, "{}", p),
        }
    }
}

impl From<&parser::Val> for Value {
    fn from(input: &parser::Val) -> Self {
        match input {
            parser::Val::Bool(_) => todo!(),
            parser::Val::String(s) => Value::string(s),
            parser::Val::List(_) => todo!(),
            parser::Val::Num(n) => Value::int(*n),
        }
    }
}

impl From<&parser::Expr> for Value {
    fn from(input: &parser::Expr) -> Self {
        match input {
            parser::Expr::Val(v) => Value::from(v),
            parser::Expr::LambdaExpr(_, _) => todo!(),
            parser::Expr::Command(_, _) => todo!(),
            _ => todo!(),
        }
    }
}

impl Value {
    pub fn data_descriptors(&self) -> Vec<Descriptor> {
        match self {
            Value::Object(o) => o.data_descriptors(),
            Value::List(_l) => todo!(),
            Value::Primitive(_p) => todo!(),
        }
    }

    pub fn get_data_from_key(&self, key: impl Into<String>) -> &Value {
        match self {
            Value::Object(o) => o.get_data_from_key(key.into()),
            Value::List(_l) => todo!(),
            Value::Primitive(_p) => todo!(),
        }
    }

    pub fn get_data(&self, desc: &Descriptor) -> &Value {
        match self {
            Value::Object(o) => o.get_data(desc),
            Value::List(_l) => todo!(),
            Value::Primitive(_p) => todo!(),
        }
    }

    pub fn format(&self) -> String {
        match self {
            Value::Object(_o) => todo!(),
            Value::List(_l) => todo!(),
            Value::Primitive(p) => p.format(),
        }
    }

    pub fn object(dict: DataDict) -> Self {
        Value::Object(dict)
    }

    pub fn list(values: impl Into<Vec<Value>>) -> Self {
        Value::List(values.into())
    }

    pub fn string(string: impl Into<String>) -> Self {
        Value::Primitive(Primitive::String(string.into()))
    }

    pub fn int(int: impl Into<i64>) -> Self {
        Value::Primitive(Primitive::Integer(int.into()))
    }

    pub fn time(time: impl Into<SystemTime>) -> Self {
        Value::Primitive(Primitive::Time(time.into()))
    }

    pub fn size(size: impl Into<u64>) -> Self {
        Value::Primitive(Primitive::Size(size.into()))
    }

    pub fn none() -> Self {
        Value::Primitive(Primitive::None)
    }
}

impl Value {
    pub fn to_int(&self) -> Result<i64, ShellError> {
        match self {
            Value::Primitive(Primitive::Integer(i)) => Ok(*i),
            _ => Err(ShellError::new("Expected an integer")),
        }
    }
}

pub trait ToBaseView {
    fn to_base_view(&self) -> BaseView;
}

impl ToBaseView for Value {
    fn to_base_view(&self) -> BaseView {
        BaseView::new(self)
    }
}

pub trait Type: Debug {}

#[derive(Debug)]
pub struct Any;

impl Type for Any {}
