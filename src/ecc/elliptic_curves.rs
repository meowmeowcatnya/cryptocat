//General Elliptic Curves
mod field;

use field::Field;

#[derive(PartialEq)]
enum Point<T: Field>{
    Point{x: T,y: T},
    PointAtInfinity
}


struct EllipticCurveType1 <T: Field> {
    //E: y^2=x^3+ax+b
    d: T, // discriminant
    a: T, // 1st curve parameter
    b: T, // 2nd curve parameter
}

impl<T: Field + Eq> EllipticCurveType1 <T> {
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
        match p {
            Point::PointAtInfinity => true, 
            Point::Point { x, y} => {
                let ax = self.a * x;
                let x3 = x.pow(3);
                let z = ax + x3 + self.b;
                y.pow(2) == z

            }
        }
    }

    fn add_point(self, p1: Point<T>, p2: Point<T>) -> Point<T> {
        //neutral rule
        match p1 {
            Point::PointAtInfinity => p2,
            Point::Point { x, y } => {
                let a = x;
                let b = y;
                match p2 {
                    Point::PointAtInfinity => p1,
                    Point::Point { x, y } => {
                        //inverse rule 
                        if a == x && b == T::inv_add(&y) {Point::PointAtInfinity} else {
                                // double rule
                            if p1 == p2 {
                                // x coordinate
                                let x2 = x.pow(2);
                                let num = T::scalar_left_multiplication(3, &x2) + self.a;
                                let den = T::scalar_left_multiplication(2, &y);
                                let frac = num * T::inv_mul(&den);
                                let x3 = frac.pow(2) + T::inv_add(&T::scalar_left_multiplication(2, &x));

                                //y coordinate
                                let diff = x + T::inv_add(&x3);
                                let prod = frac * diff;
                                let y3 = prod + T::inv_add(&y);

                                Point::Point { x: x3, y: y3 }
                            } else {
                                //addition rule
                                // x coordinate 
                                let num = y + T::inv_add(&b);
                                let den = x + T::inv_add(&a);
                                let frac = num * T::inv_mul(&den);
                                let frac2 = frac.pow(2);
                                let x3 = frac2 + T::inv_add(&a) + T::inv_add(&x);  

                                // y coordinate
                                let diff = a + T::inv_add(&x3);
                                let prod = frac * diff;
                                let y3 = prod + T::inv_add(&b);
                                
                                Point::Point { x: x3, y: y3}
                            }
                        }
                    }
                }
            }
        }
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
        match p {
            Point::PointAtInfinity => true,
            Point::Point { x, y } => {
                let y2: T = y.pow(2);
                let xy: T = x * y;
                let lhs: T = y2 + xy;
                let x3: T = x.pow(3);
                let ax2: T = self.a * x.pow(2);
                let rhs: T = x3 + ax2 + self.b;
                lhs == rhs 
            }
        }
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
        match p {
            Point::PointAtInfinity => true, 
            Point::Point { x, y } => {
            let y2: T = y.pow(2);
            let cy: T = y * self.c;
            let lhs: T = y2 + cy;
            let x3: T = x.pow(3);
            let ax: T = self.a * x;
            let rhs = x3 + ax + self.b;
            lhs == rhs
            }
        }
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
        match p {
            Point::PointAtInfinity => true,
            Point::Point { x, y } => {
            let lhs: T = y.pow(2);
            let x3: T = x.pow(3);
            let ax2: T = self.a * x.pow(2);
            let rhs = x3 + ax2 + self.b;
            lhs == rhs
            }
        }
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
        match p {
            Point::PointAtInfinity => true, 
            Point::Point { x, y } => {
            let lhs = y.pow(2);
            let x3 = x.pow(3);
            let ax = self.a * x;
            let rhs = x3 + ax + self.b;
            lhs == rhs
            }
        }
    }
}