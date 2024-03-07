//General Elliptic Curves
mod field;

use field::Field;

struct Point<T>{
    x: T,
    y: T
}



struct EllipticCurveType1 <T: Field> {
    //E: y^2=x^3+ax+b
    d: T, // discriminant
    a: T, // 1st curve parameter
    b: T, // 2nd curve parameter
}

impl<T: Field> EllipticCurveType1 <T> {
    fn new(a:  T, b: T) -> Self {
        if T::char() == 3 || T::char() == 2 {panic!("The field's characteristic must not be 2 or 3");}
        let mut x = a.pow(3);
        let mut y = b.pow(2);
        x = T::scalar_left_multiplication(4,&x);
        y = T::scalar_left_multiplication(27, &y);
        Self {
            a, 
            b,
            d:  T::scalar_left_multiplication(-16, &(x+y)),
        }
    }

    fn test_point(self, p: Point<T>) -> bool{
        //tests if point satisfies equation
        let ax = self.a * p.x;
        let x3 = &p.x.pow(3);
        let z = ax + *x3 + self.b;
        p.y.pow(2) == z
    }
}

struct EllipticCurveType2NS<T: Field> {
    //E: y^2 + xy = x^3 + ax^2 + b
    a: T,
    b: T,
    d: T,
    // in this case we always have d = b.
}

impl<T:Field> EllipticCurveType2NS<T>{
    fn new(a:T, b:T) -> Self {
        if T::char() != 2 {panic!("The field's characteristic must be 2")}
        Self {
            a,
            b,
            d: b,
        }
    }

    fn test_point(self, p: Point<T>) -> bool {
        let y2: T = p.y.pow(2);
        let xy: T = p.x * p.y;
        let lhs: T = y2 + xy;
        let x3: T = p.x.pow(3);
        let ax2: T = self.a * p.x.pow(2);
        let rhs: T = x3 + ax2 + self.b;
        lhs == rhs 
    }
}

struct EllipticCurveType2S<T: Field> {
    //y^2 + cy = x^3 + ax + b 
    a: T,
    b: T, 
    c: T,
    d: T,
}

impl<T:Field> EllipticCurveType2S<T> {
    fn new(a: T, b: T, c: T) -> Self {
        if T::char() != 2 {panic!("The field's characteristic must be 2")}
        Self {
            a,
            b,
            c,
            d: c.pow(4),
        }
    }

    fn test_point(self, p: Point<T>) -> bool {
        let y2: T = p.y.pow(2);
        let cy: T = p.y * self.c;
        let lhs: T = y2 + cy;
        let x3: T = p.x.pow(3);
        let ax: T = self.a * p.x;
        let rhs = x3 + ax + self.b;
        lhs == rhs
    }
}

struct EllipticCurveType3NS<T: Field> {
    // y^2 = x^3 + ax^2 + b
    a: T,
    b: T,
    d: T,
}

impl<T: Field> EllipticCurveType3NS<T> {
    fn new(a: T, b: T) -> Self {
        if T::char() != 3 {panic!("The field's characteristic must be 3")}
        Self {
            a,
            b,
            d: T::inv_mul(&(a.pow(3) * b)),
        }
    }

    fn test_point(self, p: Point<T>) -> bool {
        let lhs: T = p.y.pow(2);
        let x3: T = p.x.pow(3);
        let ax2: T = self.a * p.x.pow(2);
        let rhs = x3 + ax2 + self.b;
        lhs == rhs
    }
}

struct EllipticCurveType3S<T: Field> {
    // y^2 = x^3 + ax + b;
    a: T,
    b: T,
    d: T,
}

impl<T: Field> EllipticCurveType3S<T> {
    fn new(a: T, b: T) -> Self {
        if T::char() != 3 {panic!("the field's characteristic must be 3")}
        Self {
            a,
            b,
            d: T::inv_mul(&a.pow(3)),
        }
    }

    fn test_point(self, p: Point<T>) -> bool {
        let lhs = p.y.pow(2);
        let x3 = p.x.pow(3);
        let ax = self.a * p.x;
        let rhs = x3 + ax + self.b;
        lhs == rhs
    }
}