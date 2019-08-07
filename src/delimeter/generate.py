print("// GENERATED: DO NOT EDIT")
print("""use nom::{{
    named,
    tag,
    IResult,
    combinator::{{map}}
}};
""")
with open('delimeters.txt') as f:
    lines = [line for line in f]
    for line in lines:
        name, token = line.rstrip().split(" ")
        print(f"const {name.upper()}: &str = r#\"{token}\"#;")
    print("")
    print("""#[derive(Debug, PartialEq, Clone)]
pub enum Delimeter {""")
    for line in lines:
        name, _ = line.rstrip().split(" ")
        print(f"\t{name.upper()},")
    print("}")
    print("")
    for line in lines:
        name, token = line.rstrip().split(" ")
        print(f"""pub fn parse_{name}(i: &[u8]) -> IResult<&[u8], Delimeter> {{
    named!(semi, tag!(r#"{token}"#));
    map(semi, |_| Delimeter::{name.upper()})(i)
}}""")
        print("")
    print("#[cfg(test)]")
    print(
        "mod tests {\n\tuse super::*;\n\tuse std::str::from_utf8;\n")
    for line in lines:
        name, token = line.rstrip().split(" ")
        print(f"""\t#[test]
\tfn parses_{name}() {{
\t    let input = r#"{token}a"#;
\t    let (remaining, delim) = parse_{name}(input.as_bytes()).ok().unwrap();
\t    assert_eq!(
\t        (from_utf8(remaining).unwrap(), delim),
\t        ("a", Delimeter::{name.upper()}))
\t}}""")
print("}")
