use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
  #[end]
  End,

  #[error]
  Error,

  /** Delimiters **/
  #[token = "."]
  Period,
  #[token = ";"]
  Semicolon,
  #[token = ","]
  Comma,
  #[regex = "\""]
  Quotation,

  /** Function Visibility Specifiers **/
  #[token = "public"]
  Public,
  #[token = "private"]
  Private,
  #[token = "external"]
  External,
  #[token = "internal"]
  Internal,

  /** Brackets/Braces/Parentheses **/
  #[token = "("]
  LeftParentheses,
  #[token = ")"]
  RightParentheses,
  #[token = "{"]
  LeftBrace,
  #[token = "}"]
  RightBrace,
  #[token = "["]
  LeftBracket,
  #[token = "]"]
  RightBracket,

  /** Modifiers **/
  #[token = "pure"]
  Pure,
  #[token = "view"]
  View,
  #[token = "payable"]
  Payable,
  #[token = "constant"]
  Constant,
  #[token = "Anonymous"]
  Anonymous,
  #[token = "indexed"]
  Indexed,

  /** Reserved **/
  #[token = "abstract"]
  Abstract,
  #[token = "after"]
  After,
  #[token = "alias"]
  Alias,
  #[token = "apply"]
  Apply,
  #[token = "auto"]
  Auto,
  #[token = "case"]
  Case,
  #[token = "catch"]
  Catch,
  #[token = "copyof"]
  Copyof,
  #[token = "default"]
  Default,
  #[token = "define"]
  Define,
  #[token = "final"]
  Final,
  #[token = "immutable"]
  Immutable,
  #[token = "implements"]
  Implements,
  #[token = "in"]
  In,
  #[token = "inline"]
  Inline,
  #[token = "let"]
  Let,
  #[token = "macro"]
  Macro,
  #[token = "match"]
  Match,
  #[token = "mutable"]
  Mutable,
  #[token = "null"]
  Null,
  #[token = "of"]
  Of,
  #[token = "override"]
  Override,
  #[token = "partial"]
  Partial,
  #[token = "promise"]
  Promise,
  #[token = "reference"]
  Reference,
  #[token = "relocatable"]
  Relocatable,
  #[token = "sealed"]
  Sealed,
  #[token = "sizeof"]
  Sizeof,
  #[token = "static"]
  Static,
  #[token = "supports"]
  Supports,
  #[token = "switch"]
  Switch,
  #[token = "try"]
  Try,
  #[token = "typedef"]
  Typedef,
  #[token = "typeof"]
  Typeof,
  #[token = "unchecked"]
  Unchecked,

  /** Operators **/
  #[token = "++"]
  Increment,
  #[token = "--"]
  Decrement,
  #[token = "new"]
  New,
  #[token = "&&"]
  And,
  #[token = "||"]
  Or,
  #[token = "-"]
  Minus,
  #[token = "+"]
  Plus,
  #[token = "delete"]
  Delete,
  #[token = "!"]
  NotLogical,
  #[token = "~"]
  NotBitwise,
  #[token = "**"]
  Exponentiation,
  #[token = "*"]
  Times,
  #[token = "/"]
  Divide,
  #[token = "%"]
  Mod,
  #[token = "<<"]
  LeftShift,
  #[token = ">>"]
  RightShift,
  #[token = "&"]
  AndBitwise,
  #[token = "|"]
  OrBitwise,
  #[token = "<"]
  LT,
  #[token = ">"]
  GT,
  #[token = "<="]
  LTE,
  #[token = ">="]
  GTE,
  #[token = "=="]
  Equals,
  #[token = "!="]
  NotEquals,
  #[token = "|="]
  AssignBitwiseOr,
  #[token = "&="]
  AssignBitwiseAnd,
  #[token = "^="]
  AssignBitwiseXOR,
  #[token = "<<="]
  AssignBitwiseLeftShift,
  #[token = ">>="]
  AssignBitwiseRightShift,
  #[token = "+="]
  AssignAdd,
  #[token = "-="]
  AssignMinus,
  #[token = "*="]
  AssignMultiply,
  #[token = "/="]
  AssignDivide,
  #[token = "%="]
  AssignMod,
  #[token = "="]
  Assign,

  /** Keywords **/
  #[token = "pragma"]
  Pragma,
  #[token = "interface"]
  Interface,
  #[token = "constructor"]
  Constructor,
  #[token = "assembly"]
  Assembly,
  #[token = "function"]
  Function,

  /** Solidity Assembly **/
  // Operators
  #[token = ":="]
  AssignAssembly,
  #[token = "let"]
  DeclarationAssembly,

  // Op Codes
  #[token = "stop"]
  OpStop,
  #[token = "add"]
  OpAdd,
  #[token = "sub"]
  OpSub,
  #[token = "mul"]
  OpMul,
  #[token = "div"]
  OpDiv,
  #[token = "sdiv"]
  OpSdiv,
  #[token = "mod"]
  OpMod,
  #[token = "smod"]
  OpSmod,
  #[token = "exp"]
  OpExp,
  #[token = "not"]
  OpNot,
  #[token = "lt"]
  OpLt,
  #[token = "gt"]
  OpGt,
  #[token = "slt"]
  OpSlt,
  #[token = "sgt"]
  OpSgt,
  #[token = "eq"]
  OpEq,
  #[token = "iszero"]
  OpIszero,
  #[token = "and"]
  OpAnd,
  #[token = "or"]
  OpOr,
  #[token = "xor"]
  OpXor,
  #[token = "byte"]
  OpByte,
  #[token = "shl"]
  OpShl,
  #[token = "shr"]
  OpShr,
  #[token = "sar"]
  OpSar,
  #[token = "addmod"]
  OpAddmod,
  #[token = "mulmod"]
  OpMulmod,
  #[token = "signextend"]
  OpSignextend,
  #[token = "keccak256"]
  OpKeccak256,
  #[token = "jump"]
  OpJump,
  #[token = "jumpi"]
  OpJumpi,
  #[token = "pc"]
  OpPc,
  #[token = "pop"]
  OpPop,
  #[token = "dup1"]
  OpDup1,
  #[token = "dup2"]
  OpDup2,
  #[token = "dup3"]
  OpDup3,
  #[token = "dup4"]
  OpDup4,
  #[token = "dup5"]
  OpDup5,
  #[token = "dup6"]
  OpDup6,
  #[token = "dup7"]
  OpDup7,
  #[token = "dup8"]
  OpDup8,
  #[token = "dup9"]
  OpDup9,
  #[token = "dup10"]
  OpDup10,
  #[token = "dup11"]
  OpDup11,
  #[token = "dup12"]
  OpDup12,
  #[token = "dup13"]
  OpDup13,
  #[token = "dup14"]
  OpDup14,
  #[token = "dup15"]
  OpDup15,
  #[token = "dup16"]
  OpDup16,
  #[token = "swap1"]
  OpSwap1,
  #[token = "swap2"]
  OpSwap2,
  #[token = "swap3"]
  OpSwap3,
  #[token = "swap4"]
  OpSwap4,
  #[token = "swap5"]
  OpSwap5,
  #[token = "swap6"]
  OpSwap6,
  #[token = "swap7"]
  OpSwap7,
  #[token = "swap8"]
  OpSwap8,
  #[token = "swap9"]
  OpSwap9,
  #[token = "swap10"]
  OpSwap10,
  #[token = "swap11"]
  OpSwap11,
  #[token = "swap12"]
  OpSwap12,
  #[token = "swap13"]
  OpSwap13,
  #[token = "swap14"]
  OpSwap14,
  #[token = "swap15"]
  OpSwap15,
  #[token = "swap16"]
  OpSwap16,
  #[token = "mload"]
  OpMload,
  #[token = "mstore"]
  OpMstore,
  #[token = "mstore8"]
  OpMstore8,
  #[token = "sload"]
  OpSload,
  #[token = "sstore"]
  OpSstore,
  #[token = "msize"]
  OpMsize,
  #[token = "gas"]
  OpGas,
  #[token = "address"]
  OpAddress,
  #[token = "balance"]
  OpBalance,
  #[token = "caller"]
  OpCaller,
  #[token = "callvalue"]
  OpCallvalue,
  #[token = "calldataload"]
  OpCalldataload,
  #[token = "calldatasize"]
  OpCalldatasize,
  #[token = "calldatacopy"]
  OpCalldatacopy,
  #[token = "codesize"]
  OpCodesize,
  #[token = "codecopy"]
  OpCodecopy,
  #[token = "extcodesize"]
  OpExtcodesize,
  #[token = "extcodecop"]
  OpExtcodecop,
  #[token = "returndatasize"]
  OpReturndatasize,
  #[token = "returndatacopy"]
  OpReturndatacopy,
  #[token = "extcodehash"]
  OpExtcodehash,
  #[token = "create"]
  OpCreate,
  #[token = "create2"]
  OpCreate2,
  #[token = "call"]
  OpCall,
  #[token = "callcode"]
  OpCallcode,
  #[token = "delegatecall"]
  OpDelegatecall,
  #[token = "staticcall"]
  OpStaticcall,
  #[token = "return"]
  OpReturn,
  #[token = "revert"]
  OpRevert,
  #[token = "selfdestruct"]
  OpSelfdestruct,

  /** Other **/
  #[regex = "0x[0-9a-zA-Z]+"]
  HexNumber,

  #[regex = "[a-zA-Z0-9_]+"]
  Identifier,

  #[regex = r"[\^a-zA-Z0-9_]+"]
  Anything,
}
