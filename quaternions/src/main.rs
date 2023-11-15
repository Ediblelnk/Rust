use std::ops;

fn main() {
    let q = Quaternion {
        r: 1.0,
        i: 1.0,
        j: 1.0,
        k: 1.0,
    };

    println!("q: {:#?}", q);
    println!(
        "-q: {:#?}",
        q * Quaternion {
            r: -1.0,
            i: 0.0,
            j: 0.0,
            k: 0.0
        }
    );
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Quaternion {
    r: f64,
    i: f64,
    j: f64,
    k: f64,
}

impl ops::Add for Quaternion {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
        }
    }
}

impl ops::AddAssign for Quaternion {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Mul for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
            i: self.r * rhs.i + self.i * rhs.r + self.j * rhs.k - self.k * rhs.j,
            j: self.r * rhs.j - self.i * rhs.k + self.j * rhs.r + self.k * rhs.i,
            k: self.r * rhs.k + self.i * rhs.j - self.j * rhs.i + self.k * rhs.r,
        }
    }
}

impl ops::MulAssign for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::Sub for Quaternion {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let neg_rhs = rhs
            * Quaternion {
                r: -1.0,
                i: 0.0,
                j: 0.0,
                k: 0.0,
            };

        self + neg_rhs
    }
}
