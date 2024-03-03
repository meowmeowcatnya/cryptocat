use std::collections::HashSet;
use num_bigint::BigUint;

struct FieldPoint(BigUint, BigUint);
const PointAtInfinity: i8 = -1;

enum Point {
    FieldPoint,
    PointAtInfinity,
}

struct Curve{
    a: u8, //first curve coefficient
    b: u8, //second curve coefficent 
    //TODO - curves over general fields
}

impl Curve {
    fn test(P: Point) -> bool { //tests if Point is on the curve
        todo!("Curve is not done implementing");
    }
}

struct Pol {
    degree: u32,
    coefficients: HashSet<u32> //since we're only working over binary fields, storing the non-zero coefficient's indices is enough
}

impl Pol {
    fn new(degree: m, coefficients: HashSet<u32>) -> Self {
        Self {
            degree,
            coefficients,
        }
    }
    //TODO - polynomials over general Fields
}

struct Koblitz {
    m: u32, // extension degree
    f: Pol, // reduction polynomial of degree m
    E: Curve, // curve
    n: BigUint, // base point order
    h: u32, // cofactor
    P: Point, // base point
}

const KoblitzNIST1: Koblitz = Koblitz {
    m: 163,
    f: Pol {
        degree: 163, 
        coefficients: HashSet::from([0, 3, 6, 7, 163])
    },
    E: Curve {
        a: 1,
        b: 1,
    },
    n: BigUint::from(0x_00000004_00000000_00000000_00020108_A2E0CC0D_99F8A5EF),
    h: 2,
    P: Point {
        FieldPoint(BigUnit::from(0x_00000003_F0EBA162_86A2D57E_A0991168_D4994637_E8343E36), BigUint::from(0x_00000000_D51FBC6C_71A0094F_A2CDD545_B11C5C0C_797324F1))
    },
};

/* const KOBLITZ_NIST_2: Koblitz = 2;
const KOBLITZ_NIST_3: Koblitz = 3;
const KOBLITZ_NIST_4: Koblitz = 4;
const KOBLITZ_NIST_5: Koblitz = 5;
*/

enum KoblitzNIST{
    KoblitzNIST1,
    /*KOBLITZ_NIST_2,
    KOBLITZ_NIST_3,
    KOBLITZ_NIST_4,
    KOBLITZ_NIST_5,*/
}

fn main(){

}