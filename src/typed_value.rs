//! Types for working with Ruby's VALUE type, but they are more strongly typed.

use crate::{
    value::{qfalse, qnil, qtrue, ReprValue},
    Enumerator, Float, Integer, RArray, RClass, RComplex, RFile, RHash, RMatch, RModule, RObject,
    RRational, RRegexp, RString, RStruct, RTypedData, Range, Symbol, Value,
};

/// A strongly typed Ruby value.
#[derive(Copy, Clone, Debug)]
pub enum TypedValue {
    /// A Ruby Integer.
    Integer(Integer),
    /// A Ruby Float.
    Float(Float),
    /// A Ruby Complex.
    Complex(RComplex),
    /// A Ruby Rational.
    Rational(RRational),
    /// A Ruby String.
    String(RString),
    /// A Ruby Symbol.
    Symbol(Symbol),
    /// A Ruby Range.
    Range(Range),
    /// A Ruby Array.
    Array(RArray),
    /// A Ruby Hash.
    Hash(RHash),
    /// A Ruby Struct.
    Struct(RStruct),
    /// A Ruby TypedData.
    TypedData(RTypedData),
    /// A Ruby Object.
    Object(RObject),
    /// A Ruby Class.
    Class(RClass),
    /// A Ruby Module.
    Module(RModule),
    /// A Ruby File.
    File(RFile),
    /// A Ruby Match.
    Match(RMatch),
    /// A Ruby Enumerator.
    Enumerator(Enumerator),
    /// A Ruby Regexp.
    Regexp(RRegexp),
    /// A Ruby Value.
    Value(Value),
    /// A Ruby True.
    True,
    /// A Ruby False.
    False,
    /// A Ruby Nil.
    Nil,
}

impl TypedValue {
    /// Creates a new `TypedValue` from a `Value`.
    pub fn from_value(val: Value) -> Self {
        let rb_value = val.as_rb_value();

        match val.rb_type() {
            rb_sys::ruby_value_type::RUBY_T_NONE => {
                panic!("Attempting to access garbage collected Object")
            }
            rb_sys::ruby_value_type::RUBY_T_OBJECT => {
                Self::Object(unsafe { RObject::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_CLASS => {
                Self::Class(unsafe { RClass::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_MODULE => {
                Self::Module(unsafe { RModule::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_FLOAT => {
                Self::Float(unsafe { Float::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_STRING => {
                Self::String(unsafe { RString::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_REGEXP => {
                Self::Regexp(unsafe { RRegexp::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_ARRAY => {
                Self::Array(unsafe { RArray::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_HASH => {
                Self::Hash(unsafe { RHash::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_STRUCT => {
                Self::Struct(unsafe { RStruct::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_BIGNUM => {
                Self::Integer(unsafe { Integer::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_FILE => {
                Self::File(unsafe { RFile::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_DATA => {
                Self::TypedData(unsafe { RTypedData::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_MATCH => {
                Self::Match(unsafe { RMatch::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_COMPLEX => {
                Self::Complex(unsafe { RComplex::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_RATIONAL => {
                Self::Rational(unsafe { RRational::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_NIL => Self::Nil,
            rb_sys::ruby_value_type::RUBY_T_TRUE => Self::True,
            rb_sys::ruby_value_type::RUBY_T_FALSE => Self::False,
            rb_sys::ruby_value_type::RUBY_T_SYMBOL => {
                Self::Symbol(unsafe { Symbol::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_FIXNUM => {
                Self::Integer(unsafe { Integer::from_rb_value_unchecked(rb_value) })
            }
            rb_sys::ruby_value_type::RUBY_T_UNDEF => Self::Value(val),
            rb_sys::ruby_value_type::RUBY_T_IMEMO => Self::Value(val),
            rb_sys::ruby_value_type::RUBY_T_NODE => Self::Value(val),
            rb_sys::ruby_value_type::RUBY_T_ICLASS => Self::Value(val),
            rb_sys::ruby_value_type::RUBY_T_ZOMBIE => Self::Value(val),
            rb_sys::ruby_value_type::RUBY_T_MOVED => Self::Value(val),
            rb_sys::ruby_value_type::RUBY_T_MASK => Self::Value(val),
        }
    }

    fn as_value(&self) -> Value {
        match self {
            Self::Integer(i) => i.as_value(),
            Self::Float(f) => f.as_value(),
            Self::Complex(c) => c.as_value(),
            Self::Rational(r) => r.as_value(),
            Self::String(s) => s.as_value(),
            Self::Symbol(s) => s.as_value(),
            Self::Range(r) => r.as_value(),
            Self::Array(a) => a.as_value(),
            Self::Hash(h) => h.as_value(),
            Self::Struct(s) => s.as_value(),
            Self::TypedData(t) => t.as_value(),
            Self::Object(o) => o.as_value(),
            Self::Class(c) => c.as_value(),
            Self::Module(m) => m.as_value(),
            Self::File(f) => f.as_value(),
            Self::Match(m) => m.as_value(),
            Self::Enumerator(e) => e.as_value(),
            Self::Regexp(r) => r.as_value(),
            Self::Value(v) => *v,
            Self::True => qtrue().as_value(),
            Self::False => qfalse().as_value(),
            Self::Nil => qnil().as_value(),
        }
    }
}
