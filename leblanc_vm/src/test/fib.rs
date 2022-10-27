use crate::lbvm::instructions::*;

pub fn fib_bytecode() -> Vec<u8> {
    let bc = [
        LOADVAR, 0,
        ICONST1,
        IFLE, 6,
            LOADVAR, 0,
            IRETURN,
        // fib(a-2)
        LOADVAR, 0,
        ICONST2,
        ISUB,
        CALLNORMAL, 0,

        // fib(a-1)
        LOADVAR, 0,
        ICONST1,
        ISUB,
        CALLNORMAL, 0,
        IADD,
        IRETURN
    ];
    //bc.to_vec()

    bc.iter().flat_map(|v| {
        let v = *v;
        vec![(v & 0x00FF) as u8, ((v & 0xFF00) >> 8) as u8]
    }).collect()
}

/*
if a <= 0: return a;
return fib(a-2) + fib(a-1)



 */