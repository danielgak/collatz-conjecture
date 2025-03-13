#[cfg(test)]
mod tests {
    use crate::{version_0, version_1, version_2};
    use num_bigint::BigUint;

    #[test]
    fn one_step_version_0() {
        const INTEGER: u64 = 12327829543u64;

        let r = version_0::run_steps_until_reaching_base(INTEGER);

        assert_eq!(r, 11701806950);
    }

    #[test]
    fn one_step_version_1() {
        let big_uint = BigUint::from(12327829543u64);

        let r = version_1::run_steps_until_reaching_base(&big_uint);

        assert_eq!(r, BigUint::from(11701806950u64));
    }

    #[test]
    fn one_step_version_2() {
        let big_uint = BigUint::from(12327829543u64);

        let r = version_2::run_steps_until_reaching_base(&big_uint);

        assert_eq!(r, BigUint::from(11701806950u64));
    }

    #[test]
    fn last_digit_check_extraction_for_odd_big_u_int() {
        let big_uint = BigUint::from(12327829543u64);

        let r = version_2::isEven(&big_uint);

        assert_eq!(r, false);
    }

    #[test]
    fn last_digit_check_extraction_for_even_big_u_int() {
        let big_uint = BigUint::from(12327829542u64);

        let r = version_2::isEven(&big_uint);

        assert_eq!(r, true);
    }
}
