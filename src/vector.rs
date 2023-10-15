#![allow(dead_code)]
use auto_impl_ops::auto_ops;
use std::{
    io::{Error, Write},
    marker::PhantomData,
    ops::*,
};

use crate::{objects::Interval, EPS};

type Float = f64;

pub type Vec3 = Float3<VectorT>;
pub type Point3 = Float3<PointT>;
pub type Color = Float3<ColorT>;

pub trait VecLike: Copy + Clone + Default {}

#[derive(Debug, Clone, Copy, Default)]
pub struct VectorT {}

#[derive(Debug, Clone, Copy, Default)]
pub struct PointT {}

#[derive(Debug, Clone, Copy, Default)]
pub struct ColorT {}

impl VecLike for VectorT {}
impl VecLike for PointT {}
impl VecLike for ColorT {}

#[derive(Debug, Default, Clone, Copy)]
pub struct Float3<T: VecLike> {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    _type: PhantomData<T>,
}

#[auto_ops]
impl<T1: VecLike, T2: VecLike> AddAssign<&Float3<T2>> for Float3<T1> {
    fn add_assign(&mut self, rhs: &Float3<T2>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[auto_ops]
impl<T1: VecLike, T2: VecLike> SubAssign<&Float3<T2>> for Float3<T1> {
    fn sub_assign(&mut self, rhs: &Float3<T2>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

#[auto_ops]
impl<T1: VecLike, T2: VecLike> MulAssign<&Float3<T2>> for Float3<T1> {
    fn mul_assign(&mut self, rhs: &Float3<T2>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

#[auto_ops]
impl<T: VecLike> MulAssign<Float> for Float3<T> {
    fn mul_assign(&mut self, rhs: Float) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: VecLike> Mul<&Float3<T>> for Float {
    type Output = Float3<T>;

    fn mul(self, rhs: &Float3<T>) -> Self::Output {
        rhs * self
    }
}

impl<T: VecLike> Mul<Float3<T>> for Float {
    type Output = Float3<T>;

    fn mul(self, rhs: Float3<T>) -> Self::Output {
        rhs * self
    }
}

#[auto_ops]
impl<T1: VecLike, T2: VecLike> DivAssign<&Float3<T2>> for Float3<T1> {
    fn div_assign(&mut self, rhs: &Float3<T2>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

#[auto_ops]
impl<T: VecLike> DivAssign<Float> for Float3<T> {
    fn div_assign(&mut self, rhs: Float) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: VecLike> Div<&Float3<T>> for Float {
    type Output = Float3<T>;

    fn div(self, rhs: &Float3<T>) -> Self::Output {
        rhs / self
    }
}

impl<T: VecLike> Div<Float3<T>> for Float {
    type Output = Float3<T>;

    fn div(self, rhs: Float3<T>) -> Self::Output {
        rhs / self
    }
}

impl<T: VecLike> Float3<T> {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self {
            x,
            y,
            z,
            _type: PhantomData,
        }
    }

    pub fn length(&self) -> Float {
        f64::sqrt(self.length_squared() as Float)
    }

    pub fn length_squared(&self) -> Float {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn dot<V: VecLike>(&self, rhs: &Float3<V>) -> Float {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
            _type: PhantomData,
        }
    }

    pub fn lerp<V: VecLike>(&self, value: &Float3<V>, p: f64) -> Self {
        (1.0 - p) * self + p * value
    }

    pub fn normalized(&self) -> Self {
        self / self.length()
    }

    pub fn normalize(mut self) {
        self /= self.length()
    }

    pub fn random() -> Self {
        Self {
            x: fastrand::f64(),
            y: fastrand::f64(),
            z: fastrand::f64(),
            _type: PhantomData,
        }
    }

    pub fn random_in(min: f64, max: f64) -> Self {
        Self {
            x: fastrand::f64() * (max - min) + min,
            y: fastrand::f64() * (max - min) + min,
            z: fastrand::f64() * (max - min) + min,
            _type: PhantomData,
        }
    }

    pub fn random_in_sphere() -> Self {
        loop {
            let v = Self::random_in(-1.0, 1.0);
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_disc() -> Self {
        loop {
            let mut v = Self::random_in(-1.0, 1.0);
            v.z = 0.0;
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_unit() -> Self {
        Self::random_in_sphere().normalized()
    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let v = Self::random_unit();
        v * normal.dot(&v).signum()
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        self - normal * self.dot(normal) * 2.0
    }

    pub fn refract(&self, normal: &Self, coef: f64) -> Self {
        let r = self;
        let n = *normal * -1.0;
        let cos = Interval(-1.0, 1.0).clamp(-r.dot(&n));

        let perp = coef * (r + n * cos);
        let parallel = n * -f64::sqrt(f64::abs(1.0 - perp.length_squared()));

        perp + parallel
    }

    pub fn is_zero(&self) -> bool {
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }
}

impl From<Vec3> for Point3 {
    fn from(value: Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            _type: PhantomData,
        }
    }
}

impl From<Point3> for Vec3 {
    fn from(value: Point3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            _type: PhantomData,
        }
    }
}

impl Float3<ColorT> {
    pub fn linear_to_gamma(&self) -> Color {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
            _type: self._type,
        }
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        let corrected = self.linear_to_gamma() * 255.999;

        writer.write_fmt(format_args!(
            "{} {} {}\n",
            corrected.x as u8, corrected.y as u8, corrected.z as u8
        ))?;

        Ok(())
    }
}
