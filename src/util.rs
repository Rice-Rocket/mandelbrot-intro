#![allow(unused)]

use std::ops::{Add, Div, Mul, Sub};

use image::{ImageBuffer, Pixel, PixelWithColorType, Rgb};

pub type MandelbrotImage = ImageBuffer<Rgb<u8>, Vec<u8>>;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

    #[inline]
    pub const fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    #[inline]
    pub const fn grayscale(l: f32) -> Color {
        Color { r: l, g: l, b: l }
    }

    #[inline]
    pub fn cos(self) -> Color {
        Color {
            r: self.r.cos(),
            g: self.g.cos(),
            b: self.b.cos(),
        }
    }

    #[inline]
    fn clamp(self, low: f32, high: f32) -> Color {
        Color {
            r: self.r.clamp(low, high),
            g: self.g.clamp(low, high),
            b: self.b.clamp(low, high),
        }
    }
}

impl Add<Color> for Color {
    type Output = Color;

    #[inline]
    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    #[inline]
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    #[inline]
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: rhs.r * self,
            g: rhs.g * self,
            b: rhs.b * self,
        }
    }
}

impl From<Color> for Rgb<u8> {
    fn from(v: Color) -> Self {
        let c = v.clamp(0.0, 1.0);
        Rgb([
            (c.r * 255.0) as u8,
            (c.g * 255.0) as u8,
            (c.b * 255.0) as u8,
        ])
    }
}

/// A point in 2D space.
#[derive(Clone, Copy, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    /// Creates a new instance of `Point` given an x and y position.
    #[inline]
    pub const fn new(x: T, y: T) -> Point<T> {
        Self { x, y }
    }

    /// Maps a function over the x and y parts of this point.
    #[inline]
    pub fn map<F: Fn(T) -> U, U>(self, f: F) -> Point<U> {
        Point::<U> {
            x: f(self.x),
            y: f(self.y),
        }
    }
}

impl Point<u32> {
    /// Converts a point from pixel coordinates to uv coordinates given the size of the image.
    #[inline]
    pub fn to_uv(self, n: u32) -> Point<f32> {
        self.map(|v| v as f32 / n as f32)
    }
}

impl<T: Default> Default for Point<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

/// A complex number of type `f32`.
pub type ComplexF32 = Complex<f32>;

/// A complex number.
#[derive(Clone, Copy, Debug)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T> {
    /// Creates a new instance of `Complex` with the given real and imaginary parts.
    #[inline]
    pub const fn new(re: T, im: T) -> Complex<T> {
        Self { re, im }
    }
    
    /// Maps a function over the real and imaginary parts of a complex number.
    #[inline]
    pub fn map<F: Fn(T) -> U, U>(self, f: F) -> Complex<U> {
        Complex::<U> {
            re: f(self.re),
            im: f(self.im),
        }
    }

    /// Zips two complex numbers together.
    #[inline]
    pub fn zip<U>(self, rhs: Complex<U>) -> Complex<(T, U)> {
        Complex::<(T, U)> {
            re: (self.re, rhs.re),
            im: (self.im, rhs.im),
        }
    }
}

impl<T> From<Point<T>> for Complex<T> {
    /// Converts a point to a complex number by directly
    /// mapping the point's x-value to the real component
    /// and the point's y-value to the imaginary component.
    #[inline]
    fn from(value: Point<T>) -> Self {
        Self {
            re: value.x,
            im: value.y,
        }
    }
}

impl<T: Default> Default for Complex<T> {
    fn default() -> Self {
        Self {
            re: T::default(),
            im: T::default(),
        }
    }
}

impl<T: Add<T, Output = T>> Add for Complex<T> {
    type Output = Self;

    /// Adds two complex numbers together.
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T: Sub<T, Output = T>> Sub for Complex<T> {
    type Output = Self;

    /// Subtracts one complex number from another.
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<T: Clone + Copy + Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T>> Mul for Complex<T> {
    type Output = Self;

    /// Computes the product of two complex numbers.
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl<T: Clone + Copy + Div<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T>> Div for Complex<T> {
    type Output = Self;

    /// Performs complex division on two complex numbers.
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / (rhs.re * rhs.re + rhs.im * rhs.im),
            im: (self.im * rhs.re - self.re * rhs.im) / (rhs.re * rhs.re + rhs.im * rhs.im),
        }
    }
}

impl<T> From<(T, T)> for Complex<T> {
    /// Converts a tuple into a complex number.
    fn from(value: (T, T)) -> Self {
        Complex::new(value.0, value.1)
    }
}

impl<T> From<Complex<T>> for (T, T) {
    /// Converts a complex number into a tuple.
    fn from(value: Complex<T>) -> Self {
        (value.re, value.im)
    }
}

#[allow(private_bounds)]
impl<T: Hypot> Complex<T> {
    /// Computes the absolute value (magnitude) of a complex number.
    #[inline]
    pub fn abs(self) -> T {
        self.re.hypotenuse(self.im)
    }
}


trait Hypot {
    fn hypotenuse(self, rhs: Self) -> Self;
}

impl Hypot for f32 {
    fn hypotenuse(self, rhs: Self) -> Self {
        f32::hypot(self, rhs)
    }
}

impl Hypot for f64 {
    fn hypotenuse(self, rhs: Self) -> Self {
        f64::hypot(self, rhs)
    }
}


macro_rules! impl_op_real {
    ($($op:tt, $fn:ident, $trait:ident, $doc:tt);*) => {
        $(
            impl<T: Clone + Copy + $trait<T, Output = T>> $trait<T> for Complex<T> {
                type Output = Self;

                #[inline]
                #[doc = $doc]
                fn $fn(self, rhs: T) -> Self::Output {
                    Self {
                        re: self.re $op rhs,
                        im: self.im $op rhs,
                    }
                }
            }
        )*
    }
}

impl_op_real!(
    +, add, Add, "Adds a real number to both the real and imaginary components of this complex number."; 
    -, sub, Sub, "Subtracts a real number from both the real and imaginary components of this complex number."; 
    *, mul, Mul, "Multiplies both the real and imaginary components of this complex number by a real number, effectively scaling it."; 
    /, div, Div, "Divides both the real and imaginary components of this complex number by a real number, effectively scaling it."
);
