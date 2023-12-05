mod day;
pub mod template;
pub use day::*;

pub mod parsers {
    use nom::{character::complete::digit1, combinator::map_res};
    use nom::{
        character::complete::{space0, space1},
        multi::separated_list0,
        sequence::preceded,
        IResult,
    };
    use std::str::FromStr;

    pub trait IsNumber: FromStr {}
    impl IsNumber for i8 {}
    impl IsNumber for i16 {}
    impl IsNumber for i32 {}
    impl IsNumber for i64 {}
    impl IsNumber for u8 {}
    impl IsNumber for u16 {}
    impl IsNumber for u32 {}
    impl IsNumber for u64 {}

    pub fn number_parser<NumberType: IsNumber>(input: &str) -> IResult<&str, NumberType> {
        map_res(digit1, str::parse::<NumberType>)(input)
    }

    pub fn space_separated_numbers_parser<NumberType: IsNumber>(
        input: &str,
    ) -> IResult<&str, Vec<NumberType>> {
        separated_list0(space1, preceded(space0, number_parser))(input)
    }
}
