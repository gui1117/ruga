use nom;

use nom::{IResult,digit};

use std::collections::HashMap;
use std::str;
use std::str::FromStr;

#[derive(Debug,PartialEq)]
pub enum Signal {
    Constant(bool),
    Variable(u8),
    Up((Box<Signal>,bool)),
    Not(Box<Signal>),
    And(Vec<Box<Signal>>),
    Or(Vec<Box<Signal>>),
}

impl Signal {
    fn compute_state(&mut self, entry: &HashMap<u8,bool>) -> bool {
        match *self {
            Signal::Constant(s) => s,
            Signal::Variable(s) => entry[&s],
            Signal::Not(ref mut s) => !s.compute_state(entry),
            Signal::Up((ref mut s, ref mut old_state)) => {
                if *old_state {
                    *old_state = s.compute_state(entry);
                    false
                } else {
                    if s.compute_state(entry) {
                        *old_state = true;
                        true
                    } else {
                        false
                    }
                }
            },
            Signal::And(ref mut s) => s.iter_mut().fold(false, |acc, s| acc && s.compute_state(entry)),
            Signal::Or(ref mut s) => s.iter_mut().fold(false, |acc, s| acc || s.compute_state(entry)),
        }
    }
}

named!(signal<Signal>,
    chain!(
        first: signal_term~
        mut vec: many0!(preceded!(char!('+'),signal_term)),
        || {
            if vec.len() == 0 {
                first
            } else {
                let mut vec = vec.drain(..).map(|s| Box::new(s)).collect::<Vec<Box<Signal>>>();
                vec.push(Box::new(first));
                Signal::Or(vec)
            }
        }
    )
);

named!(signal_term<Signal>,
    chain!(
        first: signal_factor~
        mut vec: many0!(preceded!(char!('*'),signal_factor)),
        || {
            if vec.len() == 0 {
                first
            } else {
                let mut vec = vec.drain(..).map(|s| Box::new(s)).collect::<Vec<Box<Signal>>>();
                vec.push(Box::new(first));
                Signal::And(vec)
            }
        }
    )
);

named!(signal_factor<Signal>,
    alt!(
        signal_unary
        | signal_variable
        | signal_constant
        | delimited!(
            char!('('),
            signal,
            char!(')')
        )
    )
);

named!(signal_constant<Signal>,
    alt!(
        map!(tag!("true"),|_| Signal::Constant(true))
        | map!(tag!("false"),|_| Signal::Constant(false))
    )
);

named!(signal_variable<Signal>,
    chain!(
         char!('s')~
         n: map_res!(
             map_res!(
                 digit,
                 str::from_utf8
             ),
             FromStr::from_str
         ),
         || {Signal::Variable(n)}
    )
);

named!(signal_unary<Signal>,
    chain!(
        up: alt!(
            map!(tag!("up"), |_| 0)
            | map!(tag!("down"), |_| 1)
            | map!(tag!("not"), |_| 2)
        )~
        s: delimited!(
            char!('('),
            signal,
            char!(')')
        ),
        || {
            if up == 0 {
                Signal::Up((Box::new(s),false))
            } else if up == 1 {
                Signal::Up((Box::new(Signal::Not(Box::new(s))),true))
            } else if up == 2 {
                Signal::Not(Box::new(s))
            } else {
                panic!();
            }
        }
    )
);

fn parse_signal(mut s: String) -> Result<Signal,String> {
    let s = s.drain(..)
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    match signal(s.as_ref()) {
        IResult::Done(_,o) => Ok(o),
        IResult::Error(e) => Err(format!("{:?}",e)),
        IResult::Incomplete(e) => Err(format!("{:?}",e)),
    }
}

