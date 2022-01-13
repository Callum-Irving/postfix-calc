use rug::Float;
use std::collections::HashMap;

use lazy_static::lazy_static;

// TODO: Synchronize this throughout crate
const PRECISION: u32 = 63;

lazy_static! {
    pub static ref LITERALS: HashMap<&'static str, Float> = {
        HashMap::from([
            (
                "pi",
                Float::with_val(
                    PRECISION,
                    3.1415926535897932384626433832795028841971693993751058209749445923,
                ),
            ),
            (
                "e",
                Float::with_val(
                    PRECISION,
                    2.7182818284590452353602874713526624977572470936999595749669676277,
                ),
            ),
        ])
    };
    pub static ref UNARY_FNS: HashMap<&'static str, fn(Float) -> Float> = {
        let mut map: HashMap<&'static str, fn(Float) -> Float> = HashMap::new();
        map.insert("sin", funcs::sin);
        map.insert("cos", funcs::cos);
        map.insert("tan", funcs::tan);
        map
    };
    pub static ref BINARY_FNS: HashMap<&'static str, fn(Float, Float) -> Float> = {
        let mut map: HashMap<&'static str, fn(Float, Float) -> Float> = HashMap::new();
        map.insert("+", funcs::add);
        map.insert("-", funcs::subtract);
        map.insert("*", funcs::multiply);
        map.insert("/", funcs::divide);
        map.insert("pow", funcs::pow);
        map.insert("log", funcs::log);
        map
    };
}

mod funcs {
    use rug::ops::Pow;
    use rug::Float;

    // UNARY
    pub fn sin(x: Float) -> Float {
        x.sin()
    }

    pub fn cos(x: Float) -> Float {
        x.cos()
    }

    pub fn tan(x: Float) -> Float {
        x.tan()
    }

    // BINARY
    pub fn add(x: Float, y: Float) -> Float {
        x + y
    }

    pub fn subtract(x: Float, y: Float) -> Float {
        x - y
    }

    pub fn multiply(x: Float, y: Float) -> Float {
        x * y
    }

    pub fn divide(x: Float, y: Float) -> Float {
        x / y
    }

    pub fn pow(base: Float, exp: Float) -> Float {
        base.pow(exp)
    }

    pub fn log(x: Float, base: Float) -> Float {
        base.log10() / x.log10()
    }
}

pub enum BuiltinFn {
    Unary(fn(Float) -> Float),
    Binary(fn(Float, Float) -> Float),
}

pub fn get_fn(identifier: &str) -> Option<BuiltinFn> {
    let unary = UNARY_FNS.get(identifier);
    if unary.is_some() {
        return Some(BuiltinFn::Unary(*unary.unwrap()));
    }
    let binary = BINARY_FNS.get(identifier);
    if binary.is_some() {
        return Some(BuiltinFn::Binary(*binary.unwrap()));
    }
    None
}
