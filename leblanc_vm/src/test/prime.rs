use crate::lbvm::instructions::*;

pub fn prime_bytecode() -> Vec<u8> {
    let bc = [
        ICONST2,                // Push "2" to stack
        STOREVAR, 1,            // Pops "2" from stack and stores at var 1
        LOADVAR, 1,             // Pushes value of var 1 onto stack
        LOADVAR, 0,             // Pushes value of var 0 onto stack
        ICONST2,                // Push "2" to stack
        IDIV,                   // Divides TOS2 / TOS1 and pushes result to stack
        IFLE, 28,               // Compares TOS2 <= TOS1, if false jumps to arg
            LOADVAR, 0,         // Pushes value of var 0 to stack
            LOADVAR, 1,         // Pushes value of var 1 to stack
            IMOD,               // TOS2 % TOS1, pushes result to stack
            INOT,               // NOT value of TOS1
            IFTRUE, 4,          // checks if TOS1 is true
                ICONST0,        // Pushes "0" to stack
                IRETURN,        // Returns TOS
            IINC, 1,            // Increments variable 1 by 1
            GOTO, 6,            // Goes to frame pointer
        ICONST1,                // Pushes "1" to stack
        IRETURN,
    ];
    //bc.to_vec()

    let c = bc.iter().flat_map(|v| {
        let v = *v;
        vec![(v & 0x00FF) as u8, ((v & 0xFF00) >> 8) as u8]
    }).collect();
    return c;
}