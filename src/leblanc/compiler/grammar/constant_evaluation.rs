use crate::leblanc::compiler::grammar::ast::Const;

pub fn add_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth) = left {
        left = if truth { Const::Whole(1) }
        else { Const::Whole(0) }
    };
    match left {
        Const::Boolean(_) => Err(()),
        Const::String(str) => {
            if let Const::String(val) = right {
                Ok(Const::String(str + &val))
            } else {
                Err(())
            }
        }
        Const::Whole(num) => {
            match right {
                Const::Whole(other) => Ok(Const::Whole(num + other)),
                Const::Float(other) => Ok(Const::Float(num as f64 + other)),
                _ => Err(())
            }
        }

        Const::Float(num) => {
            match right {
                Const::Whole(other) => Ok(Const::Float(num + other as f64)),
                Const::Float(other) => Ok(Const::Float(num + other)),
                _ => Err(())
            }
        }
    }
}

pub fn sub_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth) = left {
        left = if truth { Const::Whole(1) }
        else { Const::Whole(0) }
    };
    match left {
        Const::Boolean(_) => Err(()),
        Const::String(str) => {
            if let Const::String(val) = right {
                Ok(Const::String(str.replacen(&val, "", 1)))
            } else {
                Err(())
            }
        }
        Const::Whole(num) => {
            match right {
                Const::Whole(other) => Ok(Const::Whole(num - other)),
                Const::Float(other) => Ok(Const::Float(num as f64 - other)),
                _ => Err(())
            }
        }

        Const::Float(num) => {
            match right {
                Const::Whole(other) => Ok(Const::Float(num - other as f64)),
                Const::Float(other) => Ok(Const::Float(num - other)),
                _ => Err(())
            }
        }
    }
}

pub fn mul_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth) = left {
        left = if truth { Const::Whole(1) }
        else { Const::Whole(0) }
    };
    match left {
        Const::Boolean(_) => Err(()),
        Const::String(str) => {
            if let Const::Whole(val) = right {
                Ok(Const::String(str.repeat(val as usize)))
            } else {
                Err(())
            }
        }
        Const::Whole(num) => {
            match right {
                Const::Whole(other) => Ok(Const::Whole(num * other)),
                Const::Float(other) => Ok(Const::Float(num as f64 * other)),
                _ => Err(())
            }
        }

        Const::Float(num) => {
            match right {
                Const::Whole(other) => Ok(Const::Float(num * other as f64)),
                Const::Float(other) => Ok(Const::Float(num * other)),
                _ => Err(())
            }
        }
    }
}

pub fn div_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth) = left {
        left = if truth { Const::Whole(1) }
        else { Const::Whole(0) }
    };
    match left {
        Const::Boolean(_) => Err(()),
        Const::String(_) => Err(()),
        Const::Whole(num) => {
            match right {
                Const::Whole(other) => Ok(Const::Whole(num / other)),
                Const::Float(other) => Ok(Const::Float(num as f64 / other)),
                _ => Err(())
            }
        }

        Const::Float(num) => {
            match right {
                Const::Whole(other) => Ok(Const::Float(num / other as f64)),
                Const::Float(other) => Ok(Const::Float(num / other)),
                _ => Err(())
            }
        }
    }
}

pub fn pow_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth) = left {
        left = if truth { Const::Whole(1) }
        else { Const::Whole(0) }
    };
    match left {
        Const::Boolean(_) => Err(()),
        Const::String(_) => Err(()),
        Const::Whole(num) => {
            match right {
                Const::Whole(other) => Ok(Const::Whole(num.pow(other as u32))),
                Const::Float(other) => Ok(Const::Float((num as f64).powf(other as f64))),
                _ => Err(())
            }
        }

        Const::Float(num) => {
            match right {
                Const::Whole(other) => Ok(Const::Float(num.powf(other as f64))),
                Const::Float(other) => Ok(Const::Float(num.powf(other))),
                _ => Err(())
            }
        }
    }
}

pub fn mod_constants(left: Const, right: Const) -> Result<Const, ()> {
    let mut left = left;
    if let Const::Boolean(truth) = left {
        left = if truth { Const::Whole(1) }
        else { Const::Whole(0) }
    };
    match left {
        Const::Boolean(_) => Err(()),
        Const::String(_) => Err(()),
        Const::Whole(num) => {
            match right {
                Const::Whole(other) => Ok(Const::Whole(num % other)),
                Const::Float(other) => Ok(Const::Float(num as f64 % other)),
                _ => Err(())
            }
        }

        Const::Float(num) => {
            match right {
                Const::Whole(other) => Ok(Const::Float(num % other as f64)),
                Const::Float(other) => Ok(Const::Float(num % other)),
                _ => Err(())
            }
        }
    }
}