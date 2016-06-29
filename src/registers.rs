pub const REGISTER_COUNT: usize = 24;

#[repr(i16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Register {
	ZERO = 0,
	RA = 1,
	LO = 2,
	HI = 3,
	SP = 4,
	FP = 5,
	A0 = 6,
	A1 = 7,
	A2 = 8,
	A3 = 9,
	A4 = 10,
	A5 = 11,
	T0 = 12,
	T1 = 13,
	T2 = 14,
	T3 = 15,
	T4 = 16,
	T5 = 17,
	S0 = 18,
	S1 = 19,
	S2 = 20,
	S3 = 21,
	S4 = 22,
	S5 = 23,
}
