#![allow(unused)]

use std::ops::{Add, Div, Mul, Sub};

use image::{ImageBuffer, Pixel, PixelWithColorType, Rgb};

pub type MandelbrotImage = ImageBuffer<Rgb<u8>, Vec<u8>>;

/// A nice abstraction to handle instantiating various colors.
#[derive(Debug, Clone, Copy)]
pub struct Color;

impl Color {
    /// Create a new grayscale color from a single luminance value.
    pub fn grayscale(l: f32) -> Rgb<u8> {
        assert!((0.0..=1.0).contains(&l));
        let v = (l * 255.0) as u8;

        Rgb([v, v, v])
    }

    /// Create a color from the given `r`, `g`, and `b` values.
    pub fn rgb(r: f32, g: f32, b: f32) -> Rgb<u8> {
        assert!((0.0..=1.0).contains(&r));
        assert!((0.0..=1.0).contains(&g));
        assert!((0.0..=1.0).contains(&b));

        Rgb([
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
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
