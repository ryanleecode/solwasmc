print("// GENERATED: DO NOT EDIT")
print("""use crate::atom::Atom;

use nom::{{
    IResult,
    bytes::{{
        complete::{{tag}}
    }},
    combinator::{{map}}
}};
""")
with open('keywords.txt') as f:
    lines = [line for line in f]
    for line in lines:
        print(f"const {line.rstrip().upper()}: &str = \"{line.rstrip()}\";")
    print("")
    pass
    for line in lines:
        print(
            f"pub fn parse_{line.rstrip()}(i: &[u8]) -> IResult<&[u8], Atom> {{")
        print(
            f"    map(tag({line.rstrip().upper()}), |_| Atom::Keyword({line.rstrip().upper()}.to_string()))(i)")
        print("}")
        print("")
    print("#[cfg(test)]")
    print(
        "mod tests {\n\tuse super::*;\n\tuse std::str::from_utf8;\n")
    for line in lines:
        print("\t#[test]")
        print(f"\tfn parses_{line.rstrip()}() {{")
        print(f"\t\tlet input = \"{line.rstrip()}\\n\";")
        print(
            f"\t\tlet (remaining, keyword) = parse_{line.rstrip()}(input.as_bytes()).ok().unwrap();")
        print("\t\tassert_eq!(")
        print("\t\t\t(from_utf8(remaining).unwrap(), keyword),")
        print(
            f"\t\t\t(\"\\n\", Atom::Keyword({line.rstrip().upper()}.to_string())))")
        print("\t}")
        print("")
    print("}")
