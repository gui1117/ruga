// use nom::{IResult,digit};

// use std::collections::HashMap;
// use std::str;
// use std::str::FromStr;

// #[derive(Debug,PartialEq)]
// pub enum SignalNetwork {
//     Constant(bool),
//     Variable(usize),
//     Up((Box<SignalNetwork>,bool)),
//     Not(Box<SignalNetwork>),
//     And(Vec<Box<SignalNetwork>>),
//     Or(Vec<Box<SignalNetwork>>),
// }

// impl SignalNetwork {
//     pub fn compute_state(&mut self, entry: &Vec<bool>) -> bool {
//         match *self {
//             SignalNetwork::Constant(s) => s,
//             SignalNetwork::Variable(s) => entry[s],
//             SignalNetwork::Not(ref mut s) => !s.compute_state(entry),
//             SignalNetwork::Up((ref mut s, ref mut old_state)) => {
//                 if *old_state {
//                     *old_state = s.compute_state(entry);
//                     false
//                 } else {
//                     if s.compute_state(entry) {
//                         *old_state = true;
//                         true
//                     } else {
//                         false
//                     }
//                 }
//             },
//             SignalNetwork::And(ref mut s) => s.iter_mut().fold(false, |acc, s| acc && s.compute_state(entry)),
//             SignalNetwork::Or(ref mut s) => s.iter_mut().fold(false, |acc, s| acc || s.compute_state(entry)),
//         }
//     }
// }

// named!(signal<SignalNetwork>,
//     chain!(
//         first: signal_term~
//         mut vec: many0!(preceded!(char!('+'),signal_term)),
//         || {
//             if vec.len() == 0 {
//                 first
//             } else {
//                 let mut vec = vec.drain(..).map(|s| Box::new(s)).collect::<Vec<Box<SignalNetwork>>>();
//                 vec.push(Box::new(first));
//                 SignalNetwork::Or(vec)
//             }
//         }
//     )
// );

// named!(signal_term<SignalNetwork>,
//     chain!(
//         first: signal_factor~
//         mut vec: many0!(preceded!(char!('*'),signal_factor)),
//         || {
//             if vec.len() == 0 {
//                 first
//             } else {
//                 let mut vec = vec.drain(..).map(|s| Box::new(s)).collect::<Vec<Box<SignalNetwork>>>();
//                 vec.push(Box::new(first));
//                 SignalNetwork::And(vec)
//             }
//         }
//     )
// );

// named!(signal_factor<SignalNetwork>,
//     alt!(
//         signal_unary
//         | signal_variable
//         | signal_constant
//         | delimited!(
//             char!('('),
//             signal,
//             char!(')')
//         )
//     )
// );

// named!(signal_constant<SignalNetwork>,
//     alt!(
//         map!(tag!("true"),|_| SignalNetwork::Constant(true))
//         | map!(tag!("false"),|_| SignalNetwork::Constant(false))
//     )
// );

// named!(signal_variable<SignalNetwork>,
//     chain!(
//          char!('s')~
//          n: map_res!(
//              map_res!(
//                  digit,
//                  str::from_utf8
//              ),
//              FromStr::from_str
//          ),
//          || {SignalNetwork::Variable(n)}
//     )
// );

// named!(signal_unary<SignalNetwork>,
//     chain!(
//         up: alt!(
//             map!(tag!("up"), |_| 0)
//             | map!(tag!("down"), |_| 1)
//             | map!(tag!("not"), |_| 2)
//         )~
//         s: delimited!(
//             char!('('),
//             signal,
//             char!(')')
//         ),
//         || {
//             if up == 0 {
//                 SignalNetwork::Up((Box::new(s),false))
//             } else if up == 1 {
//                 SignalNetwork::Up((Box::new(SignalNetwork::Not(Box::new(s))),true))
//             } else if up == 2 {
//                 SignalNetwork::Not(Box::new(s))
//             } else {
//                 panic!();
//             }
//         }
//     )
// );

// pub fn parse_signal(mut s: String) -> Result<SignalNetwork,String> {
//     let s = s.drain(..)
//         .filter(|c| !c.is_whitespace())
//         .collect::<String>();
//     match signal(s.as_ref()) {
//         IResult::Done(_,o) => Ok(o),
//         IResult::Error(e) => Err(format!("{:?}",e)),
//         IResult::Incomplete(e) => Err(format!("{:?}",e)),
//     }
// }

