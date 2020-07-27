use ::std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use std::ops::{Deref, Rem};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

trait NormalizeTrait {
    fn normalize<Tx: Into<Self> + Copy, Ty: Into<Self> + Copy>(&mut self, lo: Tx, hi: Ty)
    where
        Self: Sized + PartialOrd<Tx> + PartialOrd<Ty>;
}

impl NormalizeTrait for f32 {
    fn normalize<Tx: Into<Self> + Copy, Ty: Into<Self> + Copy>(&mut self, lo: Tx, hi: Ty)
    where
        Self: Sized + PartialOrd<Tx> + PartialOrd<Ty>,
    {
        *self = self.rem_euclid(2e0 * hi.into());

        if *self > hi {
            *self -= 2e0 * hi.into();
        }

        if *self < lo {
            *self += 2e0 * hi.into();
        }
    }
}

impl From<[f32; 3]> for Vector {
    fn from(other: [f32; 3]) -> Self {
        Self {
            x: other[0] as f32,
            y: other[1] as f32,
            z: other[2] as f32,
        }
    }
}

impl From<[f32; 2]> for Vector {
    fn from(other: [f32; 2]) -> Self {
        Self {
            x: other[0] as f32,
            y: other[1] as f32,
            z: 0e0,
        }
    }
}

impl Into<[f32; 3]> for &Vector {
    fn into(self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }
}

impl Into<[f32; 2]> for &Vector {
    fn into(self) -> [f32; 2] {
        [self.x as f32, self.y as f32]
    }
}

impl Vector {
    pub fn new<Tx: Into<f32> + Copy, Ty: Into<f32> + Copy, Tz: Into<f32> + Copy>(
        x: Tx,
        y: Ty,
        z: Tz,
    ) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn len_sqr(&self) -> f32 {
        f32::powi(self.x, 2) + f32::powi(self.y, 2) + f32::powi(self.z, 2)
    }

    pub fn len(&self) -> f32 {
        self.len_sqr().sqrt()
    }

    pub fn nullify_x(self) -> Self {
        Self {
            x: 0e0,
            y: self.y,
            z: self.z,
        }
    }

    pub fn nullify_y(self) -> Self {
        Self {
            x: self.x,
            y: 0e0,
            z: self.z,
        }
    }

    pub fn nullify_z(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: 0e0,
        }
    }

    pub fn unit(&self) -> Self {
        self.clone() / self.len()
    }

    pub fn normalize(&mut self, vec_lo: &Self, vec_hi: &Self) -> &mut Self {
        self.x.normalize(vec_lo.x, vec_hi.x);
        self.y.normalize(vec_lo.y, vec_hi.y);
        self.z.normalize(vec_lo.z, vec_hi.z);

        self
    }

    pub fn normalize_yaw_cs(&mut self) -> &mut Self {
        let lo = -180f32;
        let hi = 180f32;

        self.y.normalize(lo, hi);

        self
    }

    pub fn clamp(&mut self, vec_lo: &Self, vec_hi: &Self) -> &mut Self {
        self.x = self.x.clamp(vec_lo.x, vec_hi.x);
        self.y = self.y.clamp(vec_lo.y, vec_hi.y);
        self.z = self.z.clamp(vec_lo.z, vec_hi.z);

        self
    }

    pub fn clamp_csgo(&mut self) -> &mut Self {
        let vec_lo = Self::new(-180e0, -90e0, 0e0);
        let vec_hi = Self::new(180e0, 90e0, 0e0);

        self.clamp(&vec_lo, &vec_hi);

        self
    }

    pub fn to_degrees(&mut self) -> &mut Self {
        self.as_mut().iter_mut().for_each(|x| *x = x.to_degrees());

        self
    }

    pub fn to_radians(&mut self) -> &mut Self {
        self.as_mut().iter_mut().for_each(|x| *x = x.to_radians());

        self
    }

    pub fn swap_system(&mut self) -> &mut Self {
        self.y = -self.y;
        std::mem::swap(&mut self.x, &mut self.y);
        self
    }
}

impl AsRef<[f32]> for Vector {
    fn as_ref(&self) -> &[f32] {
        unsafe { std::slice::from_raw_parts(self as *const Vector as *const f32, 3) }
    }
}

impl AsMut<[f32]> for Vector {
    fn as_mut(&mut self) -> &mut [f32] {
        unsafe { std::slice::from_raw_parts_mut(self as *mut Vector as *mut f32, 3) }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl<T: Into<f32> + Copy> Add<T> for Vector {
    type Output = Vector;

    fn add(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x + rhs.into(),
            y: self.y + rhs.into(),
            z: self.z + rhs.into(),
        }
    }
}

impl<T: Into<f32> + Copy> AddAssign<T> for Vector {
    fn add_assign(&mut self, rhs: T) {
        self.x = self.x + rhs.into();
        self.y = self.y + rhs.into();
        self.z = self.z + rhs.into();
    }
}

impl<T: Into<f32> + Copy> Sub<T> for Vector {
    type Output = Vector;

    fn sub(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x - rhs.into(),
            y: self.y - rhs.into(),
            z: self.z - rhs.into(),
        }
    }
}

impl<T: Into<f32> + Copy> SubAssign<T> for Vector {
    fn sub_assign(&mut self, rhs: T) {
        self.x = self.x - rhs.into();
        self.y = self.y - rhs.into();
        self.z = self.z - rhs.into();
    }
}

impl Mul<Vector> for Vector {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T: Into<f32> + Copy> Mul<T> for Vector {
    type Output = Vector;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs.into(),
            y: self.y * rhs.into(),
            z: self.z * rhs.into(),
        }
    }
}

impl<T: Into<f32> + Copy> MulAssign<T> for Vector {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs.into();
        self.y = self.y * rhs.into();
        self.z = self.z * rhs.into();
    }
}

impl<T: Into<f32> + Copy> Div<T> for Vector {
    type Output = Vector;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x / rhs.into(),
            y: self.y / rhs.into(),
            z: self.z / rhs.into(),
        }
    }
}

impl<T: Into<f32> + Copy> DivAssign<T> for Vector {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs.into();
        self.y = self.y / rhs.into();
        self.z = self.z / rhs.into();
    }
}
