use crate::leblanc::compiler::parser::ast::Const;
use crate::leblanc::core::native_types::LeBlancType;

pub fn add_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth) = left {
        left = if truth { Const::Whole(1, Some(LeBlancType::Int)) }
        else { Const::Whole(0, Some(LeBlancType::Int)) }
    };
    match left {
        Const::String(str) => {
            if let Const::String(val) = right {
                Ok(Const::String(str + &val))
            } else {
                Err(())
            }
        }
        Const::Whole(num, _) => {
            match right {
                Const::Whole(other, _) => Ok(Const::Whole(num + other, None)),
                Const::Float(other, _) => Ok(Const::Float(num as f64 + other, None)),
                _ => Err(())
            }
        }

        Const::Float(num, _) => {
            match right {
                Const::Whole(other, _) => Ok(Const::Float(num + other as f64, None)),
                Const::Float(other, _) => Ok(Const::Float(num + other, None)),
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub fn sub_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth) = left {
        left = if truth { Const::Whole(1, Some(LeBlancType::Int)) }
        else { Const::Whole(0, Some(LeBlancType::Int)) }
    };
    match left {
        Const::String(str) => {
            if let Const::String(val) = right {
                Ok(Const::String(str.replacen(&val, "", 1)))
            } else {
                Err(())
            }
        }
        Const::Whole(num, _) => {
            match right {
                Const::Whole(other, _) => Ok(Const::Whole(num - other, None)),
                Const::Float(other, _) => Ok(Const::Float(num as f64 - other, None)),
                _ => Err(())
            }
        }

        Const::Float(num, _) => {
            match right {
                Const::Whole(other, _) => Ok(Const::Float(num - other as f64, None)),
                Const::Float(other, _) => Ok(Const::Float(num - other, None)),
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub fn mul_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth) = left {
        left = if truth { Const::Whole(1, Some(LeBlancType::Int)) }
        else { Const::Whole(0, Some(LeBlancType::Int)) }
    };
    match left {
        Const::String(str) => {
            if let Const::Whole(val, _) = right {
                Ok(Const::String(str.repeat(val as usize)))
            } else {
                Err(())
            }
        }
        Const::Whole(num, _) => {
            match right {
                Const::Whole(other, _) => Ok(Const::Whole(num * other, None)),
                Const::Float(other, _) => Ok(Const::Float(num as f64 * other, None)),
                _ => Err(())
            }
        }

        Const::Float(num, _) => {
            match right {
                Const::Whole(other, _) => Ok(Const::Float(num * other as f64, None)),
                Const::Float(other, _) => Ok(Const::Float(num * other, None)),
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub fn div_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth) = left {
        left = if truth { Const::Whole(1, Some(LeBlancType::Int)) }
        else { Const::Whole(0, Some(LeBlancType::Int)) }
    };
    match left {
        Const::Whole(num, _) => {
            match right {
                Const::Whole(other, _) => Ok(Const::Whole(num / other, None)),
                Const::Float(other, _) => Ok(Const::Float(num as f64 / other, None)),
                _ => Err(())
            }
        }

        Const::Float(num, _) => {
            match right {
                Const::Whole(other, _) => Ok(Const::Float(num / other as f64, None)),
                Const::Float(other, _) => Ok(Const::Float(num / other, None)),
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub fn pow_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth) = left {
        left = if truth { Const::Whole(1, Some(LeBlancType::Int)) }
        else { Const::Whole(0, Some(LeBlancType::Int)) }
    };
    match left {
        Const::Whole(num, _) => {
            match right {
                Const::Whole(other, _) => Ok(Const::Whole(num.pow(other as u32), None)),
                Const::Float(other, _) => Ok(Const::Float((num as f64).powf(other as f64), None)),
                _ => Err(())
            }
        }

        Const::Float(num, _) => {
            match right {
                Const::Whole(other, _) => Ok(Const::Float(num.powf(other as f64), None)),
                Const::Float(other, _) => Ok(Const::Float(num.powf(other), None)),
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub fn mod_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth) = left {
        left = if truth { Const::Whole(1, Some(LeBlancType::Int)) }
        else { Const::Whole(0, Some(LeBlancType::Int)) }
    };
    match left {
        Const::Whole(num, _) => {
            match right {
                Const::Whole(other, _) => Ok(Const::Whole(num % other, None)),
                Const::Float(other, _) => Ok(Const::Float(num as f64 % other, None)),
                _ => Err(())
            }
        }

        Const::Float(num, _) => {
            match right {
                Const::Whole(other, _) => Ok(Const::Float(num % other as f64, None)),
                Const::Float(other, _) => Ok(Const::Float(num % other, None)),
                _ => Err(())
            }
        }
        _ => Err(())
    }
}