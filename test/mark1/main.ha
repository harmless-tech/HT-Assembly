//#class Main - Not in milestone1!
//#include structs.has - Not in milestone1!
//#include templates.hat - Not in milestone1!

/*
 * Three registers for milestone1. op1, op2, return.
 * TODO (Later Milestone) Bitwise operations.
 * TODO Types rename to shorten??
 * Types: char, string, int8, int16, int32 (int), int64, i128, uint8, uint16, uint32 (uint), uint64, uint128, float32, float64 (float), byte (uint8), boolean, object, void.
 * Types (shortened): chr, str, i8, i16, i32 (int), i64, i128, u8, u16, u32 (uint), u64, u128, f32, f64 (float), byte (u8), bool, obj, void.
 * TODO Allow for arrays, tuples, matrices, and other data structures. (Milestone 3??)
 * Cannot load data into return register.
 * Optional semi-colon at end of some lines.
 */

// NAME: :: A position tag.

// alloc NAME TYPE DEFAULT_DATA(OPTIONAL)
// setvar NAME DATA
// regvar NAME :: Uses the return register to set the var.

// setreg REG TYPE DEFAULT_DATA(OPTIONAL)
// varreg NAME REG :: Loads a variable into a op register.
// cpyreg REG :: Moves return register to another register.

// op OP :: Does a operation on the two operation registers and puts the computed value into the return register.
/* TODO Operations with two different types are not supported.
 * Operations:
 *   Logic: NOT, AND, OR, XOR
 *   Arith: MOD, MUL, DIV, PLUS, MINUS
 *   Rel: EQUAL, NOT_EQUAL, GREATER, LESS, GREATER_EQUAL, LESS_EQUAL
 *   TODO BitWise: (Not in milestone 1!!)
 */

// pushjmp TAG NAME(OPTIONAL) :: Jumps to a tag if the return register or the optional variable is true.
// popjmp :: Returns to the position +1 before the jump happened.
// loop TAG NAME(OPTIONAL) :: Loops to a tag if the return register or the optional variable is true.

// cast REG NEW_TYPE :: Attempts to cast value and puts it into the return register. TODO may not work in milestone1.
// native NATIVE_NAME (OPTIONAL ARGS) :: Calls a native method. (May not work in milestone1)
// return NAME(OPTIONAL) :: Returns a value. (Not in milestone1) (May merge with exit)
// exit CONSTANT_INT :: Ends the script with a exit code.

// createframe !!Later milestone!!
// discardframe !!Later milestone!!
// assert !!Later milestone!!

start:
    alloc hello str "Hello, World!";
    set hello "Howdy!";

    setreg 0 int 100;
    setreg r1 int 101; // Can do either just a number or rNUM, for registers.
    op +;
    regvar hello;

part2:
    alloc hello3 str "Hello, World!"
    set hello3 "Howdy!"

    setreg 0 int 100
    setreg 1 int 101
    op +
    regvar hello3