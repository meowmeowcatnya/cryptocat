use std::ops;

pub trait Field: Sized + ops::Add<Self, Output=Self> + ops::Mul<Self, Output=Self> + Eq + Copy{
    fn inv_mul(&self) -> Self;
    fn inv_add(&self) -> Self;
    fn neutral_mul() -> Self;
    fn neutral_add() -> Self;
    
    fn pow(&self, b: i32) -> Self {
        let mut x: Self = Self::neutral_mul();
        for _i in 0..b.abs() {
            x =  x * (*self)
        }
        if b >= 0 {x} else {Self::inv_mul(&self)} 
    }

    fn scalar_left_multiplication(a: i32, b: &Self) -> Self {
        let mut x: Self = Self::neutral_add();
        for _i in 0..a.abs() {
            x = x + (*b);
        }
        if a >= 0 {x} else {Self::inv_add(&b)}
    }
    fn char() -> u32;
}