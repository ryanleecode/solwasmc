extern crate pest;

#[derive(Parser)]
#[grammar = "sol.pest"]
pub struct SolParser;

use pest::error::Error;
use pest::Parser;
use pest::{consumes_to, parses_to};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn string_literal_with_all_escape_types() {
    parses_to! {
        parser: SolParser,
        input: r#""a\nb\x0Fc\u{a}d\u{AbAbAb}e""#,
        rule: Rule::string_literal,
        tokens: [
            string_literal(0, 28, [
                raw_string(1, 2),
                escape(2, 4, [
                    predefined(3, 4)
                ]),
                raw_string(4, 5),
                escape(5, 9, [
                    byte(6, 9)
                ]),
                raw_string(9, 10),
                escape(10, 15, [
                    unicode(11, 15, [
                        unicode_hex(13, 14)
                    ])
                ]),
                raw_string(15, 16),
                escape(16, 26, [
                    unicode(17, 26, [
                        unicode_hex(19, 25)
                    ])
                ]),
                raw_string(26, 27)
            ])
        ]
    };
  }
}
