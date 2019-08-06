#[derive(Debug)]
pub struct PragmaDirective {
    pub name: str,
    pub value: str,
    pub range: [u128; 2]
}