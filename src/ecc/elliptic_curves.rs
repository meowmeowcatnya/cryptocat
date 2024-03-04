//General Elliptic Curves
mod field;
use std::clone;

use field::Field;

struct Point<T>{
    x: T,
    y: T
}



struct EllipticCurveType1 <T: Field /* T::char() is assumed to not be 3 or 2*/> {
    //E: y^2=x^3+ax+b
    d: T, // discriminant
    a: T, // 1st curve parameter
    b: T, // 2nd curve parameter
}

impl<T: Field> EllipticCurveType1 <T>{
    fn new(a:  T, b: T) -> Self {
        let mut x = a.pow(3);
        let mut y = b.pow(2);
        x = T::scalar_left_multiplication(4,x);
        y = T::scalar_left_multiplication(27, y);
        Self {
            a, 
            b,
            d:  T::scalar_left_multiplication(-16, x.add(&y)),
        }
    }

    fn testPoint(self, p: Point<T>) -> bool{
        //tests if point satisfies equation
        let ax = self.a.mul(&p.x);
        let x3 = p.x.pow(3);
        let z = ax.add(&x3).add(&self.b);
        return T::equals(p.y.pow(2), &z);
    }
}

struct EllipticCurveType2NS<T: Field /*T::char() is assumed to be 2*/> {
    //E: y^2 + xy = x^3 + ax^2 + b
    a: T,
    b: T,
    // in this case we always have d = b.
}

impl<T:Field> EllipticCurveType2NS<T>{
    fn new(a:T, b:T) -> Self {
        Self {
            a,
            b,
        }
    }

    fn testPoint(self, p: Point<T>) -> bool{
        //todo
        return true
    }
}