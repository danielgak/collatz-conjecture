#[cfg(test)]
mod tests {
    use crate::{version_0, version_1};
    use num_bigint::BigUint;

    #[test]
    fn one_step_version_0() {
        const INTEGER: u64 = 12327829543u64;

        let r = version_0::run_steps_until_reaching_base(INTEGER);

        assert_eq!(r, 11701806950)
    }

    #[test]
    fn one_step_version_1() {
        let big_uint = BigUint::from(12327829543u64);

        let r = version_1::run_steps_until_reaching_base(&big_uint);

        assert_eq!(r, BigUint::from(11701806950u64))
    }
}
