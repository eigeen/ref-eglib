// use std::collections::HashMap;

// use mlua::prelude::*;

// #[derive(Debug, thiserror::Error)]
// pub enum FormatError {
//     #[error("Unexpected token `{0}` at :{1}")]
//     UnexpectedToken(char, u32),
//     /// Expected token `{0}`, found `{1}` at :{2}
//     #[error("Expected token `{0}`, found `{1}` at :{2}")]
//     UnexpectedTokenEx(&'static str, char, u32),
//     #[error("Nested braces are not allowed at :{0}")]
//     NestedBraces(u32),
//     #[error("Unmatched braces at :{0}")]
//     UnmatchedBraces(u32),

//     #[error(
//         "Unexpected named argument `{0}`. Mixed named and positional arguments are not allowed."
//     )]
//     UnexpectedNamedArg(String),
//     #[error(
//         "Unexpected positional argument `{0}`. Mixed named and positional arguments are not allowed."
//     )]
//     UnexpectedPositionalArg(u32),

//     #[error("Lua error: {0}")]
//     Lua(#[from] mlua::Error),
// }

// type Result<T> = std::result::Result<T, FormatError>;

// /// A cached format string instance.
// pub struct LuaFormat {
//     fmtstr: String,
//     fmt_parts: Vec<FormatPart>,
// }

// impl PartialEq for LuaFormat {
//     fn eq(&self, other: &Self) -> bool {
//         self.fmtstr == other.fmtstr
//     }
// }
// impl Eq for LuaFormat {}

// impl LuaFormat {
//     /// Create a new format string instance and parse it.
//     pub fn new(fmtstr: &str) -> Result<Self> {
//         let tokens = parse_format_tokens(&fmtstr)?;
//         let parts = tokens_into_parts(tokens)?;

//         Ok(Self {
//             fmtstr: fmtstr.to_string(),
//             fmt_parts: parts,
//         })
//     }

//     pub fn format_args_multi(&self, args: &LuaMultiValue) -> Result<String> {
//         let mut result_parts = vec![];

//         for part in self.fmt_parts.iter() {
//             match part {
//                 FormatPart::LitString(s) => result_parts.push(s.to_string()),
//                 FormatPart::Arg(fmtarg) => {
//                     let pos = fmtarg.pos.as_ref().unwrap();
//                     match pos {
//                         ArgPos::Named(name) => {
//                             return Err(FormatError::UnexpectedNamedArg(name.to_string()))
//                         }
//                         ArgPos::Positional(index) => {
//                             if let Some(val) = args.get(*index as usize) {
//                                 let result = Self::format_arg(fmtarg, val)?;
//                                 result_parts.push(result);
//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         Ok(result_parts.join(""))
//     }

//     pub fn format_args_table(&self, args_table: &LuaTable) -> Result<String> {
//         let mut result_parts = vec![];

//         for part in self.fmt_parts.iter() {
//             match part {
//                 FormatPart::LitString(s) => result_parts.push(s.to_string()),
//                 FormatPart::Arg(fmtarg) => {
//                     let pos = fmtarg.pos.as_ref().unwrap();
//                     match pos {
//                         ArgPos::Named(name) => {
//                             let val = args_table.get::<LuaValue>(name.as_str())?;
//                             let result = Self::format_arg(fmtarg, &val)?;
//                             result_parts.push(result);
//                         }
//                         ArgPos::Positional(index) => {
//                             return Err(FormatError::UnexpectedPositionalArg(*index));
//                         }
//                     }
//                 }
//             }
//         }

//         Ok(result_parts.join(""))
//     }

//     fn format_arg(fmtarg: &FormatArg, value: &LuaValue) -> Result<String> {
//         if let Some(precision) = fmtarg.precision {
//             // 是数字类型
//             match value {
//                 LuaNil => return Ok(value.fmt_to_string()),
//                 LuaValue::Integer(_) => todo!(),
//                 LuaValue::Number(_) => todo!(),
//                 LuaValue::String(_) => todo!(),
//                 LuaValue::Table(table) => todo!(),
//                 LuaValue::Function(function) => todo!(),
//                 LuaValue::Thread(thread) => todo!(),
//                 LuaValue::UserData(any_user_data) => todo!(),
//                 LuaValue::Error(error) => todo!(),
//                 LuaValue::Other(value_ref) => todo!(),
//             }
//             let LuaValue::Number(num) = value else {
//                 return Err(FormatError::Bad);
//             };
//             todo!()
//         }

//         if fmtarg.pretty {
//             todo!()
//         }
//         todo!()
//     }
// }

// pub fn runtime_formatter(fmtstr: &str, args: &LuaMultiValue) -> Result<String> {
//     // parse format string
//     let fmt_tokens = parse_format_tokens(fmtstr)?;

//     // 从fmt_tokens解析格式化参数
//     let mut arg_defs = HashMap::new();
//     for token in fmt_tokens.iter() {
//         match token {
//             FormatToken::LitString(litstr) => {}
//             FormatToken::Placeholder(phs) => {
//                 for ph in phs.iter() {
//                     match ph {
//                         FormatParam::ArgName(arg_name) => {
//                             arg_defs.insert(arg_name.to_string(), token.clone());
//                             break;
//                         }
//                         FormatParam::ArgIndex(arg_index) => {
//                             arg_defs.insert(arg_index.to_string(), token.clone());
//                             break;
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//         }
//     }

//     // 格式化参数

//     todo!()
// }

// pub trait LuaFormattable {
//     fn fmt_to_string(&self) -> String;
//     fn fmt_to_string_pretty(&self) -> String;
//     fn fmt_to_string_debug(&self) -> String;
// }

// impl LuaFormattable for LuaValue {
//     fn fmt_to_string(&self) -> String {
//         match self {
//             LuaNil => "nil".to_string(),
//             LuaValue::Boolean(v) => v.to_string(),
//             LuaValue::LightUserData(ud) => format!("<{} {:p}>", self.type_name(), ud),
//             LuaValue::Integer(v) => v.to_string(),
//             LuaValue::Number(v) => v.to_string(),
//             LuaValue::String(v) => v.display().to_string(),
//             LuaValue::Table(v) => format!("<{} {:p}>", self.type_name(), v),
//             LuaValue::Function(v) => format!("<{} {:p}>", self.type_name(), v),
//             LuaValue::Thread(v) => format!("<{} {:p}>", self.type_name(), v),
//             LuaValue::UserData(v) => format!("<{} {:p}>", self.type_name(), v),
//             LuaValue::Error(v) => v.to_string(),
//             _ => "<Other>".to_string(),
//         }
//     }

//     fn fmt_to_string_pretty(&self) -> String {
//         todo!()
//     }

//     fn fmt_to_string_debug(&self) -> String {
//         todo!()
//     }
// }

// fn tokens_into_parts(tokens: Vec<FormatToken>) -> Result<Vec<FormatPart>> {
//     let mut parts: Vec<FormatPart> = tokens
//         .into_iter()
//         .map(|token| token.try_into())
//         .collect::<Result<Vec<_>>>()?;

//     // 计算未提供的位置参数
//     let mut current_pos = 0;
//     for part in parts.iter_mut() {
//         if let FormatPart::Arg(fmtarg) = part {
//             if fmtarg.pos.is_none() {
//                 fmtarg.pos = Some(ArgPos::Positional(current_pos));
//                 current_pos += 1;
//             }
//         }
//     }

//     Ok(parts)
// }

// fn parse_format_tokens(fmtstr: &str) -> Result<Vec<FormatToken>> {
//     let mut tokens: Vec<FormatToken> = Vec::new();
//     let mut chars = fmtstr.chars().enumerate().peekable();
//     let mut arg_state = FormatArgState::None;

//     let mut current_literal = String::new();
//     let mut current_arg_name = String::new();
//     let mut is_index_arg = false;
//     let mut current_width_chars = String::new();
//     let mut current_params: Vec<FormatParam> = Vec::new();

//     while let Some((idx, c)) = chars.next() {
//         // 一些统一的处理，关键字过滤
//         if arg_state.is_inside() {
//             // 不允许嵌套
//             if c == '{' {
//                 return Err(FormatError::NestedBraces(idx as u32));
//             }
//             if arg_state > FormatArgState::ArgNameOrIndex
//                 && arg_state < FormatArgState::Suffix
//                 && arg_state != FormatArgState::Precision
//             {
//                 // 不允许终止
//                 if c == '}' {
//                     return Err(FormatError::UnexpectedToken(c, idx as u32));
//                 }
//             }
//             if arg_state >= FormatArgState::AfterColon && c == ':' {
//                 return Err(FormatError::UnexpectedToken(c, idx as u32));
//             }
//         }

//         match arg_state {
//             FormatArgState::None => {
//                 match c {
//                     '{' => {
//                         if let Some((_, next)) = chars.peek() {
//                             if *next == '{' {
//                                 // 转义
//                                 current_literal.push(c);
//                                 chars.next();
//                                 continue;
//                             }
//                         }
//                         if !current_literal.is_empty() {
//                             let lit_str = std::mem::take(&mut current_literal);
//                             tokens.push(FormatToken::LitString(lit_str));
//                         }
//                         arg_state = FormatArgState::ArgNameOrIndex;
//                     }
//                     _ => current_literal.push(c),
//                 }
//             }
//             FormatArgState::ArgNameOrIndex => match c {
//                 '}' => {
//                     // 保存当前的arg索引
//                     if is_index_arg {
//                         let Ok(index) = current_arg_name.parse::<u32>() else {
//                             return Err(FormatError::UnexpectedTokenEx(
//                                 "integer index",
//                                 c,
//                                 idx as u32,
//                             ));
//                         };
//                         current_params.push(FormatParam::ArgIndex(index));
//                     } else {
//                         let arg_name = std::mem::take(&mut current_arg_name);
//                         current_params.push(FormatParam::ArgName(arg_name));
//                     };
//                     // 结束
//                     tokens.push(FormatToken::Placeholder(std::mem::take(
//                         &mut current_params,
//                     )));
//                     arg_state = FormatArgState::None;
//                 }
//                 ':' => {
//                     // 保存当前的arg索引
//                     if is_index_arg {
//                         let Ok(index) = current_arg_name.parse::<u32>() else {
//                             return Err(FormatError::UnexpectedTokenEx(
//                                 "integer index",
//                                 c,
//                                 idx as u32,
//                             ));
//                         };
//                         current_params.push(FormatParam::ArgIndex(index));
//                     } else {
//                         let arg_name = std::mem::take(&mut current_arg_name);
//                         current_params.push(FormatParam::ArgName(arg_name));
//                     };
//                     // 进入格式化参数
//                     arg_state = FormatArgState::AfterColon;
//                 }
//                 c if c.is_ascii_digit() => {
//                     // 如果第一个字符是数字，则认为是数字索引
//                     if current_arg_name.is_empty() {
//                         is_index_arg = true;
//                         current_arg_name.push(c);
//                     } else {
//                         current_arg_name.push(c);
//                     }
//                 }
//                 _ => {
//                     if is_index_arg {
//                         // 索引混用
//                         return Err(FormatError::UnexpectedTokenEx(
//                             "index argument cannot be mixed with named argument",
//                             c,
//                             idx as u32,
//                         ));
//                     }
//                     current_arg_name.push(c);
//                 }
//             },
//             FormatArgState::AfterColon => {
//                 match c {
//                     '.' => {
//                         // 进入精度匹配模式
//                         arg_state = FormatArgState::Precision;
//                     }
//                     '}' => {
//                         // 结束
//                         tokens.push(FormatToken::Placeholder(std::mem::take(
//                             &mut current_params,
//                         )));
//                         arg_state = FormatArgState::None;
//                     }
//                     _ => {
//                         // 进入padding width
//                         current_params.push(FormatParam::PaddingChar(c));
//                         arg_state = FormatArgState::Width;
//                     }
//                 }
//             }
//             FormatArgState::Width => {
//                 // padding宽度
//                 match c {
//                     '.' => {
//                         let Ok(width) = current_width_chars.parse::<u32>() else {
//                             return Err(FormatError::UnexpectedTokenEx(
//                                 "width integer",
//                                 c,
//                                 idx as u32,
//                             ));
//                         };
//                         current_width_chars.clear();
//                         current_params.push(FormatParam::Width(width));
//                         // 进入精度匹配模式
//                         arg_state = FormatArgState::Precision;
//                     }
//                     '}' => {
//                         let Ok(width) = current_width_chars.parse::<u32>() else {
//                             return Err(FormatError::UnexpectedTokenEx(
//                                 "width integer",
//                                 c,
//                                 idx as u32,
//                             ));
//                         };
//                         current_width_chars.clear();
//                         current_params.push(FormatParam::Width(width));
//                         // 结束
//                         tokens.push(FormatToken::Placeholder(std::mem::take(
//                             &mut current_params,
//                         )));
//                         arg_state = FormatArgState::None;
//                     }
//                     c if c.is_ascii_digit() => {
//                         current_width_chars.push(c);
//                     }
//                     _ => {
//                         return Err(FormatError::UnexpectedTokenEx(
//                             "width integer char, `.` or `}`",
//                             c,
//                             idx as u32,
//                         ))
//                     }
//                 }
//             }
//             FormatArgState::Precision => {
//                 match c {
//                     'f' => {
//                         // 结束精度
//                         let Ok(precision) = current_width_chars.parse::<u32>() else {
//                             return Err(FormatError::UnexpectedTokenEx(
//                                 "precision integer",
//                                 c,
//                                 idx as u32,
//                             ));
//                         };
//                         current_width_chars.clear();
//                         current_params.push(FormatParam::Precision(precision));
//                         arg_state = FormatArgState::Suffix;
//                     }
//                     c if c.is_ascii_digit() => {
//                         current_width_chars.push(c);
//                     }
//                     _ => {
//                         return Err(FormatError::UnexpectedTokenEx(
//                             "precision integer char, `f` or digit",
//                             c,
//                             idx as u32,
//                         ))
//                     }
//                 }
//             }
//             FormatArgState::Suffix => {
//                 match c {
//                     '}' => {
//                         // 结束
//                         tokens.push(FormatToken::Placeholder(std::mem::take(
//                             &mut current_params,
//                         )));
//                         arg_state = FormatArgState::None;
//                     }
//                     '#' => {
//                         // 标记为pretty格式
//                         current_params.push(FormatParam::Pretty);
//                         arg_state = FormatArgState::Suffix;
//                     }
//                     '?' => {
//                         // 标记为debug格式
//                         current_params.push(FormatParam::Debug);
//                         arg_state = FormatArgState::Suffix;
//                     }
//                     _ => {
//                         return Err(FormatError::UnexpectedTokenEx(
//                             "suffix (`}`, `#` or `?`)",
//                             c,
//                             idx as u32,
//                         ))
//                     }
//                 }
//             }
//         }
//     }
//     // EOF
//     if arg_state != FormatArgState::None {
//         // 未闭合
//         return Err(FormatError::UnmatchedBraces(fmtstr.len() as u32));
//     }
//     if !current_literal.is_empty() {
//         let lit_str = std::mem::take(&mut current_literal);
//         tokens.push(FormatToken::LitString(lit_str));
//     }
//     // 检查
//     for token in &tokens {
//         let mut has_pretty = false;
//         let mut has_debug = false;

//         if let FormatToken::Placeholder(params) = token {
//             for param in params {
//                 // 后缀是否有重复
//                 if let FormatParam::Pretty = param {
//                     if has_pretty {
//                         return Err(FormatError::UnexpectedTokenEx(
//                             "duplicate pretty format",
//                             '}',
//                             fmtstr.len() as u32,
//                         ));
//                     }
//                     has_pretty = true;
//                 }
//                 if let FormatParam::Debug = param {
//                     if has_debug {
//                         return Err(FormatError::UnexpectedTokenEx(
//                             "duplicate debug format",
//                             '}',
//                             fmtstr.len() as u32,
//                         ));
//                     }
//                     has_debug = true;
//                 }
//             }
//         }
//     }

//     Ok(tokens)
// }

// /// Format argument state.
// ///
// /// {[<ArgName>]:[<PaddingChar:1><Width>][.<Precision>f][<Pretty>][<Debug>]}
// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// enum FormatArgState {
//     None,
//     ArgNameOrIndex,
//     // `:` has been seen
//     AfterColon,
//     Width,
//     // `.` has been seen
//     Precision,
//     /// `#` or `?`
//     Suffix,
// }

// impl FormatArgState {
//     fn is_inside(&self) -> bool {
//         self != &FormatArgState::None
//     }
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// enum FormatToken {
//     /// A literal string in the format string.
//     LitString(String),
//     /// `{}` argument placeholder.
//     Placeholder(Vec<FormatParam>),
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// enum FormatParam {
//     /// Named argument.
//     ArgName(String),
//     /// Positional argument.
//     ArgIndex(u32),
//     PaddingChar(char),
//     /// `:<width>` field width
//     Width(u32),
//     /// `:.<precision>f` field precision
//     Precision(u32),
//     /// `:#` pretty format
//     Pretty,
//     /// `:?` debug format
//     Debug,
// }

// pub enum FormatPart {
//     LitString(String),
//     Arg(FormatArg),
// }

// impl TryFrom<FormatToken> for FormatPart {
//     type Error = FormatError;

//     fn try_from(value: FormatToken) -> std::result::Result<Self, Self::Error> {
//         match value {
//             FormatToken::LitString(s) => Ok(FormatPart::LitString(s)),
//             FormatToken::Placeholder(params) => {
//                 let mut arg = FormatArg::default();

//                 for param in params {
//                     match param {
//                         FormatParam::ArgName(name) => {
//                             arg.pos = Some(ArgPos::Named(name));
//                         }
//                         FormatParam::ArgIndex(index) => {
//                             arg.pos = Some(ArgPos::Positional(index));
//                         }
//                         FormatParam::PaddingChar(padding_char) => {
//                             if let Some(padding) = &mut arg.padding {
//                                 padding.padding_char = padding_char;
//                             } else {
//                                 arg.padding = Some(ArgPadding {
//                                     padding_char,
//                                     width: 0,
//                                 });
//                             }
//                         }
//                         FormatParam::Width(width) => {
//                             if let Some(padding) = &mut arg.padding {
//                                 padding.width = width;
//                             } else {
//                                 arg.padding = Some(ArgPadding {
//                                     padding_char: '0',
//                                     width,
//                                 });
//                             }
//                         }
//                         FormatParam::Precision(precision) => {
//                             arg.precision = Some(precision);
//                         }
//                         FormatParam::Pretty => {
//                             arg.pretty = true;
//                         }
//                         FormatParam::Debug => {
//                             arg.debug = true;
//                         }
//                     }
//                 }

//                 Ok(FormatPart::Arg(arg))
//             }
//         }
//     }
// }

// /// A format argument.
// #[derive(Debug, Clone, Default)]
// struct FormatArg {
//     pos: Option<ArgPos>,
//     padding: Option<ArgPadding>,
//     precision: Option<u32>,
//     pretty: bool,
//     debug: bool,
// }

// #[derive(Debug, Clone)]
// enum ArgPos {
//     Named(String),
//     Positional(u32),
// }

// #[derive(Debug, Clone)]
// struct ArgPadding {
//     padding_char: char,
//     width: u32,
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_format_string() {
//         // let input = "Hello, {:>10.2f}";
//         // let input = "{:>10.2f}Hello";
//         // let input = "{0}";
//         // let input = "{name}";
//         // let input = "{1name}";
//         // let input = "{name1}";
//         // let input = "{name:>10.2f}";
//         let input = "{name:10.2f";
//         eprintln!("input: \"{}\"", input);
//         let result = parse_format_tokens(input).unwrap();
//         eprintln!("output: {:?}", result);
//     }
// }
