solc-assem-wasm

# Features

- [X] `PragmaDirective = 'pragma' Identifier ([^;]+) ';'`
- [ ] `ImportDirective = 'import' StringLiteral ('as' Identifier)? ';'
               | 'import' ('*' | Identifier) ('as' Identifier)? 'from' StringLiteral ';'
               | 'import' '{' Identifier ('as' Identifier)? ( ',' Identifier ('as' Identifier)? )* '}' 'from' StringLiteral ';'`
- [ ] `ContractDefinition = ( 'contract' | 'library' | 'interface' ) Identifier
                            ( 'is' InheritanceSpecifier (',' InheritanceSpecifier )* )?
                            '{' ContractPart* '}'`