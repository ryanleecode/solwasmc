def snake_to_upper_camel(s):
    words = s.split("_")
    return "".join([word.lower().capitalize() for word in words])


print("// GENERATED: DO NOT EDIT")
print("""use nom::{{
named,
tag,
IResult,
combinator::{{map}}
}};
""")
with open('storage_locations.txt') as f:
    lines = [line.rstrip() for line in f]
    for line in lines:
        print(f"const {line.upper()}: &str = r#\"{line}\"#;")
    print("")
    print("""#[derive(Debug, PartialEq, Clone)]
pub enum StorageLocation {""")
    for line in lines:
        print(f"\t{snake_to_upper_camel(line)},")
    print("}")
    print("")
    for line in lines:
        print(f"""pub fn parse_{line.lower()}(i: &[u8]) -> IResult<&[u8], StorageLocation> {{
named!(semi, tag!(r#"{line}"#));
map(semi, |_| StorageLocation::{snake_to_upper_camel(line)})(i)
}}""")
        print("")
    print("#[cfg(test)]")
    print(
        "mod tests {\n\tuse super::*;\n\tuse std::str::from_utf8;\n")
    for line in lines:
        print(f"""\t#[test]
\tfn parses_{line.lower()}() {{
\t    let input = r#"{line}a"#;
\t    let (remaining, name) = parse_{line.lower()}(input.as_bytes()).ok().unwrap();
\t    assert_eq!(
\t        (from_utf8(remaining).unwrap(), name),
\t        ("a", StorageLocation::{snake_to_upper_camel(line)}))
\t}}""")
print("}")
