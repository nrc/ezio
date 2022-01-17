use rand::{random, thread_rng, Rng};

macro_rules! rand_fns {
    ($name: ident, $max_name: ident) => {
        /// Generate a random integer from a uniform distribution.
        ///
        /// Implemented using the [rand](https://docs.rs/rand) crate, see their
        /// docs for more information.
        pub fn $name() -> $name {
            random()
        }

        /// Generate a random floating point number between 0 and `upper_bound`
        /// from a uniform distribution.
        ///
        /// `upper_bound` is exclusive so will never be returned.
        ///
        /// Implemented using the [rand](https://docs.rs/rand) crate, see their
        /// docs for more information.
        pub fn $max_name(upper_bound: $name) -> $name {
            thread_rng().gen_range(0..upper_bound)
        }
    };
}

macro_rules! frand_fns {
    ($name: ident, $max_name: ident) => {
        /// Generate a random integer from a uniform distribution.
        ///
        /// Implemented using the [rand](https://docs.rs/rand) crate, see their
        /// docs for more information.
        pub fn $name() -> $name {
            random()
        }

        /// Generate a random floating point number between 0 and `upper_bound`
        /// from a uniform distribution.
        ///
        /// `upper_bound` is exclusive so will never be returned.
        ///
        /// Implemented using the [rand](https://docs.rs/rand) crate, see their
        /// docs for more information.
        pub fn $max_name(upper_bound: $name) -> $name {
            thread_rng().gen_range(0.0..upper_bound)
        }
    };
}

rand_fns!(u8, u8_bound);
rand_fns!(u16, u16_bound);
rand_fns!(u32, u32_bound);
rand_fns!(u64, u64_bound);
rand_fns!(u128, u128_bound);
rand_fns!(usize, usize_bound);

rand_fns!(i8, i8_bound);
rand_fns!(i16, i16_bound);
rand_fns!(i32, i32_bound);
rand_fns!(i64, i64_bound);
rand_fns!(i128, i128_bound);
rand_fns!(isize, isize_bound);

frand_fns!(f32, f32_bound);
frand_fns!(f64, f64_bound);

/// Generate a random `bool using a uniform distribution.
pub fn bool() -> bool {
    random()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        let _foo: u8 = u8();
        let bounded: u8 = u8_bound(42);
        assert!(bounded < 42);

        let _foo: u16 = u16();
        let bounded: u16 = u16_bound(42);
        assert!(bounded < 42);

        let _foo: u32 = u32();
        let bounded: u32 = u32_bound(42);
        assert!(bounded < 42);

        let _foo: u64 = u64();
        let bounded: u64 = u64_bound(42);
        assert!(bounded < 42);

        let _foo: u128 = u128();
        let bounded: u128 = u128_bound(42);
        assert!(bounded < 42);

        let _foo: usize = usize();
        let bounded: usize = usize_bound(42);
        assert!(bounded < 42);

        let _foo: i8 = i8();
        let bounded: i8 = i8_bound(42);
        assert!(bounded < 42);

        let _foo: i16 = i16();
        let bounded: i16 = i16_bound(42);
        assert!(bounded < 42);

        let _foo: i32 = i32();
        let bounded: i32 = i32_bound(42);
        assert!(bounded < 42);

        let _foo: i64 = i64();
        let bounded: i64 = i64_bound(42);
        assert!(bounded < 42);

        let _foo: i128 = i128();
        let bounded: i128 = i128_bound(42);
        assert!(bounded < 42);

        let _foo: isize = isize();
        let bounded: isize = isize_bound(42);
        assert!(bounded < 42);

        let _foo: f32 = f32();
        let bounded: f32 = f32_bound(42.0);
        assert!(bounded < 42.0);

        let _foo: f64 = f64();
        let bounded: f64 = f64_bound(42.0);
        assert!(bounded < 42.0);

        let _foo: bool = bool();
    }
}
