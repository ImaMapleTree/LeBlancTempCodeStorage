pub(crate) const NOP: u16 = 0;
pub(crate) const NOTIMPL: u16 = 1;
pub(crate) const ICONST0: u16 = 2;
pub(crate) const ICONST1: u16 = 3;
pub(crate) const ICONST2: u16 = 4;
pub(crate) const ICONST3: u16 = 5;
pub(crate) const ICONST4: u16 = 6;
pub(crate) const ICONST5: u16 = 7;
pub(crate) const ICONST10: u16 = 8;
pub(crate) const LCONST0: u16 = 9;
pub(crate) const LCONST1: u16 = 10;
pub(crate) const LCONST2: u16 = 11;
pub(crate) const FCONST0: u16 = 12;
pub(crate) const FCONST1: u16 = 13;
pub(crate) const FCONST2: u16 = 14;
pub(crate) const DCONST0: u16 = 15;
pub(crate) const DCONST1: u16 = 16;
pub(crate) const DCONST2: u16 = 17;

pub(crate) const IADDSET: u16 = 19;
pub(crate) const IINC: u16 = 20;
pub(crate) const IADD: u16 = 21;
pub(crate) const ISUB: u16 = 22;
pub(crate) const IMUL: u16 = 23;
pub(crate) const IDIV: u16 = 24;
pub(crate) const IMOD: u16 = 25;
pub(crate) const INOT: u16 = 26;

pub(crate) const LOADCONST: u16 = 60;
pub(crate) const LOADCONST2: u16 = 61;
pub(crate) const LOADVAR: u16 = 70;
pub(crate) const LOADFUNC: u16 = 80;
pub(crate) const STOREVAR: u16 = 81;
pub(crate) const STRCL: u16 = 90;
pub(crate) const CALLNORMAL: u16 = 100;
pub(crate) const CALLBUILTIN: u16 = 110;
pub(crate) const EQ: u16 = 120;
pub(crate) const NE: u16 = 130;
pub(crate) const GE: u16 = 140;
pub(crate) const GT: u16 = 150;
pub(crate) const LE: u16 = 160;
pub(crate) const LT: u16 = 170;
pub(crate) const IFTRUE: u16 = 178;
pub(crate) const IFFALSE: u16 = 179;
pub(crate) const IFEQ: u16 = 180;
pub(crate) const IFNE: u16 = 190;
pub(crate) const IFGE: u16 = 200;
pub(crate) const IFGT: u16 = 210;
pub(crate) const IFLE: u16 = 220;
pub(crate) const IFLT: u16 = 230;
pub(crate) const GOTO: u16 = 240;

pub(crate) const IRETURN: u16 = 249;
pub(crate) const RETURN: u16 = 250;
pub(crate) const NORETURN: u16 = 251;
pub(crate) const PRINT: u16 = 800;














//^.*pub\(crate\) const (.*):(.*)
//\t\t$1 => \"$1\",
pub fn debug_instruct(i: u16) -> String {
    String::from(match i {
		NOP => "NOP",
		NOTIMPL => "NOTIMPL",
		ICONST0 => "ICONST0",
		ICONST1 => "ICONST1",
		ICONST2 => "ICONST2",
		ICONST3 => "ICONST3",
		ICONST4 => "ICONST4",
		ICONST5 => "ICONST5",
		ICONST10 => "ICONST10",
		LCONST0 => "LCONST0",
		LCONST1 => "LCONST1",
		LCONST2 => "LCONST2",
		FCONST0 => "FCONST0",
		FCONST1 => "FCONST1",
		FCONST2 => "FCONST2",
		DCONST0 => "DCONST0",
		DCONST1 => "DCONST1",
		DCONST2 => "DCONST2",

		IADDSET => "IADDSET",
		IINC => "IINC",
		IADD => "IADD",
		ISUB => "ISUB",
		IMUL => "IMUL",
		IDIV => "IDIV",
		IMOD => "IMOD",
		INOT => "INOT",

		LOADCONST => "LOADCONST",
		LOADCONST2 => "LOADCONST2",
		LOADVAR => "LOADVAR",
		LOADFUNC => "LOADFUNC",
		STOREVAR => "STOREVAR",
		STRCL => "STRCL",
		CALLNORMAL => "CALLNORMAL",
		CALLBUILTIN => "CALLBUILTIN",
		EQ => "EQ",
		NE => "NE",
		GE => "GE",
		GT => "GT",
		LE => "LE",
		LT => "LT",
		IFTRUE => "IFTRUE",
		IFFALSE => "IFFALSE",
		IFEQ => "IFEQ",
		IFNE => "IFNE",
		IFGE => "IFGE",
		IFGT => "IFGT",
		IFLE => "IFLE",
		IFLT => "IFLT",
		GOTO => "GOTO",

		IRETURN => "IRETURN",
		RETURN => "RETURN",
		NORETURN => "NORETURN",
		PRINT => "PRINT",
        other => { println!("ERRORED INSTRUCT: {}", other); ""}
    })
}