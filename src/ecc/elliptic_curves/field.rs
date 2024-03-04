pub trait Field: {
    fn inv_mul(self) -> Self;
    fn inv_add(self) -> Self;
    fn neutral_mul() -> Self;
    fn neutral_add() -> Self;
    fn pow(&self, b: i32) -> Self;
    fn scalar_left_multiplication(a: i32, b: Self) -> Self;
    fn add(self, b: &Self) -> Self;
    fn mul(self, b: &Self) -> Self;
    fn char() -> u32;
    fn equals(self, b: &Self) -> bool;
}