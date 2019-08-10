use crate::atom::parse_identifier;
use crate::expression::{parse_expression, Expression};
use nom::{
    bytes::complete::tag, character::complete::multispace0, sequence::preceded,
    sequence::separated_pair, IResult,
};

pub type NameValue = (String, Expression);

fn parse_name_value_list(i: &[u8]) -> IResult<&[u8], NameValue> {
    separated_pair(
        parse_identifier,
        preceded(multispace0, tag(":")),
        preceded(multispace0, parse_expression),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elementary_type_name::ElementaryTypeName;
    use crate::expression::primary_expr::PrimaryExpression;
    use pretty_assertions::assert_eq;
    use std::str::from_utf8;

    #[test]
    fn parses_name_value() {
        let input = b"a     : bool\n";
        let result = parse_name_value_list(input);
        if result.is_err() {
            result.expect("should parse name value");
        } else {
            let (remaining, b) = result.ok().unwrap();
            assert_eq!(
                (from_utf8(remaining).unwrap(), b),
                (
                    "\n",
                    (
                        "a".to_string(),
                        Expression::PrimaryExpression(
                            PrimaryExpression::ElementaryTypeNameExpression(
                                ElementaryTypeName::Bool
                            )
                        )
                    )
                )
            )
        }
    }
}
