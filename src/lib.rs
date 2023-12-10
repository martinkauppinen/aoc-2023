mod day;
pub mod template;
pub use day::*;

pub mod parsers {
    use nom::bytes::complete::tag;
    use nom::combinator::recognize;
    use nom::{character::complete::digit1, combinator::map_res};
    use nom::{
        character::complete::{space0, space1},
        multi::separated_list0,
        sequence::preceded,
        IResult,
    };
    use nom_supreme::ParserExt;
    use std::str::FromStr;

    pub trait IsNumber: FromStr {}
    impl IsNumber for i8 {}
    impl IsNumber for i16 {}
    impl IsNumber for i32 {}
    impl IsNumber for i64 {}
    impl IsNumber for i128 {}
    impl IsNumber for u8 {}
    impl IsNumber for u16 {}
    impl IsNumber for u32 {}
    impl IsNumber for u64 {}
    impl IsNumber for u128 {}

    pub fn number_parser<NumberType: IsNumber>(input: &str) -> IResult<&str, NumberType> {
        map_res(
            recognize(digit1.opt_preceded_by(tag("-"))),
            str::parse::<NumberType>,
        )(input)
    }

    pub fn space_separated_numbers_parser<NumberType: IsNumber>(
        input: &str,
    ) -> IResult<&str, Vec<NumberType>> {
        separated_list0(space1, preceded(space0, number_parser))(input)
    }
}
