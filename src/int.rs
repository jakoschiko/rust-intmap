use std::fmt::Debug;
use std::num::{
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroU8, NonZeroUsize,
};

/// An integer that can be used as underlying key for [`IntMap`].
///
/// Note that this is a sealed trait that cannot be implemented externally.
///
/// [`IntMap`]: crate::IntMap
pub trait Int: SealedInt {}

impl Int for u8 {}
impl Int for u16 {}
impl Int for u32 {}
impl Int for u64 {}
impl Int for usize {}
impl Int for i8 {}
impl Int for i16 {}
impl Int for i32 {}
impl Int for i64 {}
impl Int for isize {}

/// A type that can be used as key for [`IntMap`].
///
/// You can implement this for types that wrap primitive integers.
///
/// [`IntMap`]: crate::IntMap
pub trait IntKey: Copy {
    /// The underlying integer that is used as actual key.
    type Int: Int;
    /// Converts the key into the underlying integer.
    fn to_int(self) -> Self::Int;
    /// Converts the integer into a key.
    ///
    /// You can assume that this function will only be called with integers returned by
    /// [`IntKey::to_int`].
    fn from_int(int: Self::Int) -> Self;
}

macro_rules! impl_int_key_with_self {
    ($self:ident) => {
        impl IntKey for $self {
            type Int = $self;

            fn to_int(self) -> Self::Int {
                self
            }

            fn from_int(int: Self::Int) -> Self {
                int
            }
        }
    };
}

impl_int_key_with_self!(u8);
impl_int_key_with_self!(u16);
impl_int_key_with_self!(u32);
impl_int_key_with_self!(u64);
impl_int_key_with_self!(i8);
impl_int_key_with_self!(i16);
impl_int_key_with_self!(i32);
impl_int_key_with_self!(i64);

macro_rules! impl_int_key_for_non_zero_int {
    ($non_zero_int:ident as $int:ident) => {
        impl IntKey for $non_zero_int {
            type Int = $int;

            fn to_int(self) -> Self::Int {
                self.get()
            }

            fn from_int(int: Self::Int) -> Self {
                Self::new(int).unwrap()
            }
        }
    };
}

impl_int_key_for_non_zero_int!(NonZeroU8 as u8);
impl_int_key_for_non_zero_int!(NonZeroU16 as u16);
impl_int_key_for_non_zero_int!(NonZeroU32 as u32);
impl_int_key_for_non_zero_int!(NonZeroU64 as u64);
impl_int_key_for_non_zero_int!(NonZeroUsize as usize);
impl_int_key_for_non_zero_int!(NonZeroI8 as i8);
impl_int_key_for_non_zero_int!(NonZeroI16 as i16);
impl_int_key_for_non_zero_int!(NonZeroI32 as i32);
impl_int_key_for_non_zero_int!(NonZeroI64 as i64);
impl_int_key_for_non_zero_int!(NonZeroIsize as isize);

pub trait SealedInt: Copy + PartialEq + Debug + SerdeInt {
    fn calc_index(self, mod_mask: usize) -> usize;
}

#[cfg(not(feature = "serde"))]
pub trait SerdeInt {}

#[cfg(feature = "serde")]
pub trait SerdeInt: serde::Serialize + for<'de> serde::Deserialize<'de> {}

macro_rules! impl_sealed_int_with_highest_prime {
    ($uint:ident, $prime:expr) => {
        impl SealedInt for $uint {
            #[inline(always)]
            fn calc_index(self, mod_mask: usize) -> usize {
                let hash = $prime.wrapping_mul(self);
                // Faster modulus
                (hash as usize) & mod_mask
            }
        }

        impl SerdeInt for $uint {}
    };
}

macro_rules! impl_sealed_int_with_cast {
    ($int:ident as $uint:ident) => {
        impl SealedInt for $int {
            #[inline(always)]
            fn calc_index(self, mod_mask: usize) -> usize {
                (self as $uint).calc_index(mod_mask)
            }
        }

        impl SerdeInt for $int {}
    };
}

impl_sealed_int_with_highest_prime!(u8, 251_u8);
impl_sealed_int_with_highest_prime!(u16, 65_521_u16);
impl_sealed_int_with_highest_prime!(u32, 4_294_967_291_u32);
impl_sealed_int_with_highest_prime!(u64, 11_400_714_819_323_198_549_u64);

#[cfg(target_pointer_width = "16")]
impl_sealed_int_with_cast!(usize as u16);
#[cfg(target_pointer_width = "32")]
impl_sealed_int_with_cast!(usize as u32);
#[cfg(target_pointer_width = "64")]
impl_sealed_int_with_cast!(usize as u64);

impl_sealed_int_with_cast!(i8 as u8);
impl_sealed_int_with_cast!(i16 as u16);
impl_sealed_int_with_cast!(i32 as u32);
impl_sealed_int_with_cast!(i64 as u64);
impl_sealed_int_with_cast!(isize as usize);
