//! Spec abstract operations per ECMA-262 §7. Each operation is named
//! verbatim from the spec where reasonable. v1 implements the subset
//! exercised by the round-3.d.b opcode handlers.

use crate::value::Value;
use std::rc::Rc;

/// ToBoolean per §7.1.2.
pub fn to_boolean(v: &Value) -> bool {
    match v {
        Value::Undefined | Value::Null => false,
        Value::Boolean(b) => *b,
        Value::Number(n) => !(n.is_nan() || *n == 0.0),
        Value::String(s) => !s.is_empty(),
        Value::BigInt(b) => !b.is_zero(),
        Value::Symbol(_) => true,
        Value::Object(_) => true,
    }
}

/// ToNumber per §7.1.4. v1 supports the primitive cases; Object → primitive
/// → number coercion lands when intrinsics + Symbol.toPrimitive arrive.
pub fn to_number(v: &Value) -> f64 {
    match v {
        Value::Undefined => f64::NAN,
        Value::Null => 0.0,
        Value::Boolean(true) => 1.0,
        Value::Boolean(false) => 0.0,
        Value::Number(n) => *n,
        Value::String(s) => parse_string_to_number(s.as_str()),
        Value::BigInt(b) => b.to_f64(), // ECMA §7.1.4 throws TypeError; we follow Bun's pragmatic lossy coercion
        Value::Symbol(_) => f64::NAN, // ECMA §7.1.4 throws TypeError on Symbol; lossy NaN matches existing BigInt pragmatism
        Value::Object(_) => f64::NAN, // Object -> primitive deferred
    }
}

fn parse_string_to_number(s: &str) -> f64 {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return 0.0;
    }
    if let Some(rest) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        return u64::from_str_radix(rest, 16)
            .map(|n| n as f64)
            .unwrap_or(f64::NAN);
    }
    trimmed.parse::<f64>().unwrap_or(f64::NAN)
}

/// ToString per §7.1.17. v1 supports primitives.
pub fn to_string(v: &Value) -> Rc<String> {
    Rc::new(match v {
        Value::Undefined => "undefined".to_string(),
        Value::Null => "null".to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Number(n) => number_to_string(*n),
        Value::String(s) => return s.clone(),
        Value::BigInt(b) => b.to_decimal(),
        // Ω.5.P19.E1: Symbol stores its canonical `@@sym:<n>:<desc>` form
        // as the inner Rc<String>, which is also the underlying property-
        // storage key. Returning it preserves the round-trip `obj[sym] = v`
        // → `obj.properties["@@sym:..."]` invariant the storage layer
        // depends on. Spec §7.1.17 throws TypeError on Symbol; we follow
        // the same pragmatic relaxation BigInt takes one line up.
        Value::Symbol(s) => return s.clone(),
        Value::Object(_) => "[object Object]".to_string(), // Object ToString deferred
    })
}

/// Number::toString per §6.1.6.1.20. v1 uses Rust's default f64 formatter
/// with special-cases for integer numbers + NaN + Infinity per spec.
pub fn number_to_string(n: f64) -> String {
    if n.is_nan() {
        return "NaN".to_string();
    }
    if n == f64::INFINITY {
        return "Infinity".to_string();
    }
    if n == f64::NEG_INFINITY {
        return "-Infinity".to_string();
    }
    if n == 0.0 {
        return "0".to_string();
    }
    // Ω.5.P61.E17: number-to-string per ECMA §6.1.6.1.20.
    // - Integers below 2^53 use the exact i64 representation.
    // - Integers between 2^53 and 10^21 use {:.0} (no exponential).
    // - Values >= 10^21 use exponential notation per spec step 9.
    // Pre-E17 cruftless used `n as i64` for everything < 1e21 which
    // overflowed past ~9e18; test262 Object.getOwnPropertyDescriptor
    // 2-16/17 surfaced this via numeric keys past 9e18.
    if n.fract() == 0.0 && n.abs() < 1e21 {
        if n.abs() < (1u64 << 53) as f64 {
            return format!("{}", n as i64);
        }
        return format!("{:.0}", n);
    }
    // Small-magnitude branch: ECMA §6.1.6.1.20 step uses exponential
    // notation when the decimal exponent is < -6, i.e., |n| < 1e-6.
    // Rust's default Display gives "0.0000001" for 1e-7; spec wants "1e-7".
    if n.abs() < 1e-6 {
        let s = format!("{:e}", n);
        let mut out = String::new();
        let bytes = s.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let c = bytes[i] as char;
            out.push(c);
            if c == 'e'
                && i + 1 < bytes.len()
                && bytes[i + 1] as char != '-'
                && bytes[i + 1] as char != '+'
            {
                out.push('+');
            }
            i += 1;
        }
        return out;
    }
    if n.abs() >= 1e21 {
        // ECMA-style exponential: 1e+21, 1.5e+22, -1e+21, etc.
        // Rust's {:e} gives 1e21; need 1e+21 prefix.
        let s = format!("{:e}", n);
        // Patch sign: Rust gives 'e21', spec wants 'e+21' for positive.
        let mut out = String::new();
        let mut i = 0;
        let bytes = s.as_bytes();
        while i < bytes.len() {
            let c = bytes[i] as char;
            out.push(c);
            if c == 'e'
                && i + 1 < bytes.len()
                && bytes[i + 1] as char != '-'
                && bytes[i + 1] as char != '+'
            {
                out.push('+');
            }
            i += 1;
        }
        return out;
    }
    format!("{}", n)
}

/// SameValue per ECMA §7.2.10 — like strict equality but NaN equals NaN
/// and +0 does NOT equal −0. Used by Object.is.
pub fn same_value(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            if x.is_nan() && y.is_nan() {
                return true;
            }
            // +0 / -0 distinguish via sign bit.
            if *x == 0.0 && *y == 0.0 {
                return x.is_sign_positive() == y.is_sign_positive();
            }
            x == y
        }
        _ => is_strictly_equal(a, b),
    }
}

/// SameValueZero per ECMA §7.2.11 — NaN equals NaN, +0 equals −0.
/// Used by Array.prototype.includes and Set/Map key equality.
pub fn same_value_zero(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            if x.is_nan() && y.is_nan() {
                return true;
            }
            x == y
        }
        _ => is_strictly_equal(a, b),
    }
}

/// Strict equality per §7.2.15.
pub fn is_strictly_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Undefined, Value::Undefined) => true,
        (Value::Null, Value::Null) => true,
        (Value::Boolean(x), Value::Boolean(y)) => x == y,
        (Value::Number(x), Value::Number(y)) => {
            // NaN !== NaN per IEEE 754 and spec
            if x.is_nan() || y.is_nan() {
                return false;
            }
            x == y
        }
        (Value::String(x), Value::String(y)) => x.as_str() == y.as_str(),
        (Value::BigInt(x), Value::BigInt(y)) => x == y,
        // Ω.5.P19.E1: SameValue on Symbols compares the canonical
        // `@@sym:` string by content. Symbol() with each call carries a
        // distinct counter, so two literal `Symbol('x')` calls never
        // compare equal. Symbol.for(k) intentionally produces a stable
        // `@@sym:<k>` form for any given k, so `Symbol.for('a') === Symbol.for('a')`
        // holds via content equality.
        (Value::Symbol(x), Value::Symbol(y)) => x.as_str() == y.as_str(),
        (Value::Object(x), Value::Object(y)) => x == y,
        _ => false,
    }
}

/// Loose equality per §7.2.13. v1 handles the primitive cases; full
/// type-coercion table including Object-to-primitive lands later.
pub fn is_loosely_equal(a: &Value, b: &Value) -> bool {
    // Same-type fast path: defer to strict equality.
    if std::mem::discriminant(a) == std::mem::discriminant(b) {
        return is_strictly_equal(a, b);
    }
    match (a, b) {
        (Value::Null, Value::Undefined) | (Value::Undefined, Value::Null) => true,
        (Value::Number(x), Value::String(s)) | (Value::String(s), Value::Number(x)) => {
            let y = parse_string_to_number(s.as_str());
            !x.is_nan() && !y.is_nan() && *x == y
        }
        // ECMA §7.2.13 BigInt/Number: equal iff BigInt numerically == n.
        (Value::BigInt(b), Value::Number(n)) | (Value::Number(n), Value::BigInt(b)) => {
            if n.is_nan() || n.is_infinite() || n.fract() != 0.0 {
                return false;
            }
            matches!(b.cmp_f64(*n), Some(std::cmp::Ordering::Equal))
        }
        // BigInt/String: parse the string as a BigInt and compare.
        (Value::BigInt(b), Value::String(s)) | (Value::String(s), Value::BigInt(b)) => {
            match crate::bigint::JsBigInt::from_decimal(s.as_str()) {
                Some(parsed) => b.cmp(&parsed) == std::cmp::Ordering::Equal,
                None => false,
            }
        }
        // Boolean -> Number, then re-compare loosely.
        (Value::Boolean(b), other) | (other, Value::Boolean(b)) => {
            let nb = if *b { 1.0 } else { 0.0 };
            is_loosely_equal(&Value::Number(nb), other)
        }
        _ => false,
    }
}

/// Abstract Relational Comparison per §7.2.14, returning Ordering.
/// Used by <, >, <=, >= opcodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelOrder {
    Less,
    Greater,
    Equal,
    Undefined,
}

pub fn abstract_relational_compare(x: &Value, y: &Value) -> RelOrder {
    use std::cmp::Ordering::*;
    // v1 simplified: ToPrimitive → if both String, lex compare; else ToNumber.
    if let (Value::String(a), Value::String(b)) = (x, y) {
        return match a.as_str().cmp(b.as_str()) {
            Less => RelOrder::Less,
            Greater => RelOrder::Greater,
            Equal => RelOrder::Equal,
        };
    }
    // BigInt-aware relational compare per ECMA §7.2.13.
    let ord_to_rel = |o: std::cmp::Ordering| match o {
        Less => RelOrder::Less,
        Greater => RelOrder::Greater,
        Equal => RelOrder::Equal,
    };
    match (x, y) {
        (Value::BigInt(a), Value::BigInt(b)) => return ord_to_rel(a.cmp(b)),
        (Value::BigInt(a), Value::Number(n)) => {
            return match a.cmp_f64(*n) {
                Some(o) => ord_to_rel(o),
                None => RelOrder::Undefined,
            };
        }
        (Value::Number(n), Value::BigInt(b)) => {
            return match b.cmp_f64(*n) {
                Some(o) => ord_to_rel(o.reverse()),
                None => RelOrder::Undefined,
            };
        }
        _ => {}
    }
    let nx = to_number(x);
    let ny = to_number(y);
    if nx.is_nan() || ny.is_nan() {
        return RelOrder::Undefined;
    }
    if nx < ny {
        RelOrder::Less
    } else if nx > ny {
        RelOrder::Greater
    } else {
        RelOrder::Equal
    }
}

/// EXT 78: ECMA-262 §21.2.1.1 BigInt() constructor body.
/// Combines ToPrimitive(value, "number") with the spec's primitive
/// dispatch table — Number arguments route through NumberToBigInt
/// (RangeError when not integral), other primitives through ToBigInt
/// (§7.1.13). Object inputs unbox via ToPrimitive so BigInt wrappers
/// and user @@toPrimitive run.
pub fn to_bigint(
    rt: &mut crate::interp::Runtime,
    v: &Value,
) -> Result<Value, crate::interp::RuntimeError> {
    use crate::bigint::JsBigInt;
    use crate::interp::RuntimeError;
    let prim = match v {
        Value::Object(_) => rt.to_primitive(v, "number")?,
        _ => v.clone(),
    };
    match prim {
        Value::BigInt(b) => Ok(Value::BigInt(b)),
        Value::Boolean(b) => Ok(Value::BigInt(Rc::new(if b {
            JsBigInt::one()
        } else {
            JsBigInt::zero()
        }))),
        Value::String(s) => match JsBigInt::from_decimal(s.trim()) {
            Some(b) => Ok(Value::BigInt(Rc::new(b))),
            None => Err(RuntimeError::SyntaxError(format!(
                "Cannot convert {:?} to a BigInt",
                s.as_str()
            ))),
        },
        // NumberToBigInt per §21.2.1.1.1: RangeError for non-integral
        // Numbers (NaN, ±Infinity, fractional). Integral Numbers convert.
        Value::Number(n) => {
            if !n.is_finite() || n.fract() != 0.0 {
                return Err(RuntimeError::RangeError(format!(
                    "The number {} cannot be converted to a BigInt because it is not an integer",
                    n
                )));
            }
            Ok(Value::BigInt(Rc::new(JsBigInt::from_i64(n as i64))))
        }
        Value::Undefined => Err(RuntimeError::TypeError(
            "Cannot convert undefined to a BigInt".into(),
        )),
        Value::Null => Err(RuntimeError::TypeError(
            "Cannot convert null to a BigInt".into(),
        )),
        Value::Symbol(_) => Err(RuntimeError::TypeError(
            "Cannot convert a Symbol value to a BigInt".into(),
        )),
        Value::Object(_) => Err(RuntimeError::TypeError(
            "Cannot convert object to a BigInt".into(),
        )),
    }
}

/// Apply `+` semantics per §13.15. ToPrimitive-coerces operands; if either
/// is a String, concatenate; else arithmetic add.
pub fn op_add(x: &Value, y: &Value) -> Value {
    if matches!(x, Value::String(_)) || matches!(y, Value::String(_)) {
        let xs = to_string(x);
        let ys = to_string(y);
        let mut concat = String::with_capacity(xs.len() + ys.len());
        concat.push_str(&xs);
        concat.push_str(&ys);
        return Value::String(Rc::new(concat));
    }
    if let (Value::BigInt(a), Value::BigInt(b)) = (x, y) {
        return Value::BigInt(Rc::new(a.add(b)));
    }
    Value::Number(to_number(x) + to_number(y))
}
