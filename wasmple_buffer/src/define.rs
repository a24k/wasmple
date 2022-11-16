#[derive(Debug, PartialEq, Eq)]
pub(super) enum T {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
}

impl From<u8> for T {
    fn from(t: u8) -> Self {
        match t {
            0 => T::I8,
            1 => T::U8,
            2 => T::I16,
            3 => T::U16,
            4 => T::I32,
            5 => T::U32,
            6 => T::I64,
            7 => T::U64,
            8 => T::F32,
            9 => T::F64,
            _ => T::U8,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::T;

    #[rstest]
    #[case(0, T::I8)]
    #[case(1, T::U8)]
    #[case(2, T::I16)]
    #[case(3, T::U16)]
    #[case(4, T::I32)]
    #[case(5, T::U32)]
    #[case(6, T::I64)]
    #[case(7, T::U64)]
    #[case(8, T::F32)]
    #[case(9, T::F64)]
    fn t_as_u8(#[case] expected: u8, #[case] input: T) {
        assert_eq!(expected, input as u8);
    }

    #[rstest]
    #[case(T::I8,  0)]
    #[case(T::U8,  1)]
    #[case(T::I16, 2)]
    #[case(T::U16, 3)]
    #[case(T::I32, 4)]
    #[case(T::U32, 5)]
    #[case(T::I64, 6)]
    #[case(T::U64, 7)]
    #[case(T::F32, 8)]
    #[case(T::F64, 9)]
    #[case(T::U8, 10)]
    #[case(T::U8,255)]
    fn u8_as_t(#[case] expected: T, #[case] input: u8) {
        assert_eq!(expected, T::from(input));
    }
}
