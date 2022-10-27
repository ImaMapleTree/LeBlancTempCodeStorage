use crate::leblanc::compiler::parser::ast::Const;
use crate::leblanc::core::native_types::LeBlancType;

pub fn add_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth, location) = left {
        left = if truth { Const::Whole(1, Some(LeBlancType::Int), location) }
        else { Const::Whole(0, Some(LeBlancType::Int), location) }
    };
    match left {
        Const::String(str, _location) => {
            if let Const::String(val, location) = right {
                Ok(Const::String(str + &val, location))
            } else {
                Err(())
            }
        }
        Const::Whole(num, _, _location) => {
            match right {
                Const::Whole(other, _, location) => Ok(Const::Whole(num + other, None, location)),
                Const::Float(other, _, location) => Ok(Const::Float(num as f64 + other, None, location)),
                _ => Err(())
            }
        }

        Const::Float(num, _, _location) => {
            match right {
                Const::Whole(other, _, location) => Ok(Const::Float(num + other as f64, None, location)),
                Const::Float(other, _, location) => Ok(Const::Float(num + other, None, location)),
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub fn sub_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth, location) = left {
        left = if truth { Const::Whole(1, Some(LeBlancType::Int), location) }
        else { Const::Whole(0, Some(LeBlancType::Int), location) }
    };
    match left {
        Const::String(str, _location) => {
            if let Const::String(val, location) = right {
                Ok(Const::String(str.replacen(&val, "", 1), location))
            } else {
                Err(())
            }
        }
        Const::Whole(num, _, _location) => {
            match right {
                Const::Whole(other, _, location) => Ok(Const::Whole(num - other, None, location)),
                Const::Float(other, _, location) => Ok(Const::Float(num as f64 - other, None, location)),
                _ => Err(())
            }
        }

        Const::Float(num, _, _location) => {
            match right {
                Const::Whole(other, _, location) => Ok(Const::Float(num - other as f64, None, location)),
                Const::Float(other, _, location) => Ok(Const::Float(num - other, None, location)),
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub fn mul_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth, location) = left {
        left = if truth { Const::Whole(1, Some(LeBlancType::Int), location) }
        else { Const::Whole(0, Some(LeBlancType::Int), location) }
    };
    match left {
        Const::String(str, _location) => {
            if let Const::Whole(val, _, location) = right {
                Ok(Const::String(str.repeat(val as usize), location))
            } else {
                Err(())
            }
        }
        Const::Whole(num, _, _location) => {
            match right {
                Const::Whole(other, _, location) => Ok(Const::Whole(num * other, None, location)),
                Const::Float(other, _, location) => Ok(Const::Float(num as f64 * other, None, location)),
                _ => Err(())
            }
        }

        Const::Float(num, _, _location) => {
            match right {
                Const::Whole(other, _, location) => Ok(Const::Float(num * other as f64, None, location)),
                Const::Float(other, _, location) => Ok(Const::Float(num * other, None, location)),
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub fn div_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth, location) = left {
        left = if truth { Const::Whole(1, Some(LeBlancType::Int), location) }
        else { Const::Whole(0, Some(LeBlancType::Int), location) }
    };
    match left {
        Const::Whole(num, _, _location) => {
            match right {
                Const::Whole(other, _, location) => Ok(Const::Whole(num / other, None, location)),
                Const::Float(other, _, location) => Ok(Const::Float(num as f64 / other, None, location)),
                _ => Err(())
            }
        }

        Const::Float(num, _, _location) => {
            match right {
                Const::Whole(other, _, location) => Ok(Const::Float(num / other as f64, None, location)),
                Const::Float(other, _, location) => Ok(Const::Float(num / other, None, location)),
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub fn pow_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth, location) = left {
        left = if truth { Const::Whole(1, Some(LeBlancType::Int), location) }
        else { Const::Whole(0, Some(LeBlancType::Int), location) }
    };
    match left {
        Const::Whole(num, _, _location) => {
            match right {
                Const::Whole(other, _, location) => Ok(Const::Whole(num.pow(other as u32), None, location)),
                Const::Float(other, _, location) => Ok(Const::Float((num as f64).powf(other as f64), None, location)),
                _ => Err(())
            }
        }

        Const::Float(num, _, _location) => {
            match right {
                Const::Whole(other, _, location) => Ok(Const::Float(num.powf(other as f64), None, location)),
                Const::Float(other, _, location) => Ok(Const::Float(num.powf(other), None, location)),
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub fn mod_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth, location) = left {
        left = if truth { Const::Whole(1, Some(LeBlancType::Int), location) }
        else { Const::Whole(0, Some(LeBlancType::Int), location) }
    };
    match left {
        Const::Whole(num, _, _location) => {
            match right {
                Const::Whole(other, _, location) => Ok(Const::Whole(num % other, None, location)),
                Const::Float(other, _, location) => Ok(Const::Float(num as f64 % other, None, location)),
                _ => Err(())
            }
        }

        Const::Float(num, _, _location) => {
            match right {
                Const::Whole(other, _, location) => Ok(Const::Float(num % other as f64, None, location)),
                Const::Float(other, _, location) => Ok(Const::Float(num % other, None, location)),
                _ => Err(())
            }
        }
        _ => Err(())
    }
}