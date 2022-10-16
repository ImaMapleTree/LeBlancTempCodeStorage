

#[macro_export]
macro_rules! make_ast {
    ($l:expr, $r:expr, $name:ident, $data:expr) => {
        $name::new(Location::new(($l, $r)), $data)
    }
}

#[macro_export]
macro_rules! lazystore {
    () => (
        LazyStore::default()
    );
    ($($x:expr),+ $(,)?) => {
        {
            let v = <[_]>::into_vec(
                Box::new([$($x),+])
            );
            LazyStore::from(v)
        }
    };
}

#[macro_export]
macro_rules! bytes {
    ($($x:expr),+ $(,)?) => {
        {
            [$($x as u16),+]
        }
    }
}

#[macro_export]
macro_rules! unhex_instruct {
    ($line:expr, $instruct:ident) => {
        {
            use crate::leblanc::core::interpreter::instructions2::Instruction2::*;
            $instruct($line, [])
        }
    };
    ($line:expr, $instruct:ident, $($hex:expr),+ $(,)?) => {
        {
            use crate::leblanc::core::interpreter::instructions2::Instruction2::*;
            $instruct($line, [$(scrape_arg(&mut $hex)),+])
        }
    }
}