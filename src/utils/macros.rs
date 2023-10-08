//! Macros for near-identical implementations in Vec2D and Vec3D

/// Ignore all parameters after the first and just return the first
macro_rules! ignore_rest {
    ($type:tt, $( $dummy:tt ),*) => {
        $type
    };
}

macro_rules! impl_vec_core {
    ($struct:ty, $field_type:ty, ($( $field:ident ),+)) => {
        #[doc = concat!("Create a `", stringify!($struct), "` from the provided `", stringify!($field_type), "` values")]
        pub const fn new($( $field: $field_type, )*) -> Self {
            Self { $( $field, )* }
        }

        #[doc = concat!("Return the `", stringify!($struct), "` as a tuple")]
        pub fn as_tuple(&self) -> (
            $( ignore_rest!($field_type, $field), )*
        ) {
            (
                $( self.$field, )*
            )
        }
    };
}

macro_rules! impl_vec_single_value_const {
    ($struct:ty, $name:ident, $value:expr, ($( $field:ident ),+)) => {
        #[doc = concat!("The `", stringify!($struct), "`'s ", stringify!($name), " value")]
        pub const $name: Self = Self {
            $(
                $field: $value,
            )*
        };
    };
}

macro_rules! impl_vec_add {
    ($struct:ty, ($( $field:ident ),+)) => {
        use std::ops::{Add, AddAssign};

        impl Add<Self> for $struct {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    $(
                        $field: self.$field + rhs.$field,
                    )*
                }
            }
        }

        impl AddAssign<Self> for $struct {
            fn add_assign(&mut self, rhs: Self) {
                $(
                    self.$field += rhs.$field;
                )*
            }
        }
    };
}

macro_rules! impl_vec_sub {
    ($struct:ty, ($( $field:ident ),+)) => {
        use std::ops::{Sub, SubAssign};

        impl Sub<Self> for $struct {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    $(
                        $field: self.$field - rhs.$field,
                    )*
                }
            }
        }

        impl SubAssign<Self> for $struct {
            fn sub_assign(&mut self, rhs: Self) {
                $(
                    self.$field -= rhs.$field;
                )*
            }
        }
    };
}

macro_rules! impl_vec_neg {
    ($struct:ty, $zero:expr, ($( $field:ident ),+)) => {
        use std::ops::{Neg};

        impl Neg for $struct {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self {
                    $(
                        $field: $zero,
                    )*
                }
            }
        }
    };
}

macro_rules! impl_vec_mul {
    ($struct:ty, ($( $field:ident ),+)) => {
        use std::ops::{Mul, MulAssign};

        impl Mul<Self> for $struct {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self {
                    $(
                        $field: self.$field * rhs.$field,
                    )*
                }
            }
        }

        impl MulAssign<Self> for $struct {
            fn mul_assign(&mut self, rhs: Self) {
                $(
                    self.$field *= rhs.$field;
                )*
            }
        }
    };
}

macro_rules! impl_vec_mul_single {
    ($struct:ty, $multiplier:ty, ($( $field:ident ),+)) => {
        impl Mul<$multiplier> for $struct {
            type Output = Self;

            fn mul(self, rhs: $multiplier) -> Self::Output {
                Self {
                    $(
                        $field: self.$field * rhs,
                    )*
                }
            }
        }

        impl MulAssign<$multiplier> for $struct {
            fn mul_assign(&mut self, rhs: $multiplier) {
                $(
                    self.$field *= rhs;
                )*
            }
        }
    };
}

macro_rules! impl_vec_div {
    ($struct:ty, ($( $field:ident ),+)) => {
        use std::ops::{Div, DivAssign};

        impl Div<Self> for $struct {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self {
                    $(
                        $field: self.$field / rhs.$field,
                    )*
                }
            }
        }

        impl DivAssign<Self> for $struct {
            fn div_assign(&mut self, rhs: Self) {
                $(
                    self.$field /= rhs.$field;
                )*
            }
        }
    };
}

macro_rules! impl_vec_div_single {
    ($struct:ty, $multiplier:ty, ($( $field:ident ),+)) => {
        impl Div<$multiplier> for $struct {
            type Output = Self;

            fn div(self, rhs: $multiplier) -> Self::Output {
                Self {
                    $(
                        $field: self.$field / rhs,
                    )*
                }
            }
        }

        impl DivAssign<$multiplier> for $struct {
            fn div_assign(&mut self, rhs: $multiplier) {
                $(
                    self.$field /= rhs;
                )*
            }
        }
    };
}

macro_rules! impl_vec_rem {
    ($struct:ty, ($( $field:ident ),+)) => {
        use std::ops::{Rem, RemAssign};

        impl Rem<Self> for $struct {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self::Output {
                Self {
                    $(
                        $field: self.$field.rem_euclid(rhs.$field),
                    )*
                }
            }
        }

        impl RemAssign<Self> for $struct {
            fn rem_assign(&mut self, rhs: Self) {
                $(
                    self.$field = self.$field.rem_euclid(rhs.$field);
                )*
            }
        }
    };
}
