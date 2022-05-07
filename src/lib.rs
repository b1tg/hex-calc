use nom::branch::alt;
use nom::character::complete::none_of;
use nom::character::complete::{alpha1, digit1, hex_digit1};
use nom::character::is_digit;
use nom::combinator::{map, opt, recognize};
use nom::multi::{many0, many1, many_m_n};
// use nom::Err::Error;
use nom::bytes::complete::take_while1;
use nom::character::is_alphabetic;
use nom::combinator::eof;
use nom::error::Error;
use nom::sequence::{preceded, terminated};
use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};
use nom::{error::ErrorKind, Err, Needed};
use nom::{InputIter, Parser};

fn read_return(input: &str) -> IResult<&str, ()> {
    let rn = alt((tag::<_, &str, _>("\r\n"), tag::<_, &str, _>("\n")));
    // map(tag::<_, &str, _>("\n"), |x: &str| Token::Newline)(input)
    map(rn, |_: &str| ())(input)
}

#[test]
fn test_read_num() {
    assert_eq!(read_num("0x10+123"), Ok(("+123", Op::Num(16))));
    assert_eq!(read_num("123 -111"), Ok((" -111", Op::Num(123))));
    assert_eq!(read_num("").is_err(), true);
    // unsupport negative number
    assert_eq!(read_num("-123 -111").is_err(), true);

    assert_eq!(read_symbol("-123"), Ok(("123", Op::Minus)));
    assert_eq!(read_symbol("+123"), Ok(("123", Op::Add)));
    assert_eq!(read_symbol("* 123"), Ok((" 123", Op::Mul)));
    assert_eq!(read_symbol("(1+2)"), Ok(("1+2)", Op::PL)));
    assert_eq!(read_symbol(")+1"), Ok(("+1", Op::PR)));
    // unsupport divid
    assert_eq!(read_symbol("/ 123").is_err(), true);
    assert_eq!(read_symbol("").is_err(), true);

    dbg!(parse_expr(" (1+2)*3"));
}
#[test]
fn test_hex_to_i32() {
    assert_eq!(hex_to_i32("0x11"), Some(17));
    assert_eq!(hex_to_i32("11"), Some(17));
}

fn hex_to_i32(input: &str) -> Option<i32> {
    if !input.starts_with("0x") && !input.starts_with("0X") {
        return i32::from_str_radix(&input[..], 16).ok();
    }
    i32::from_str_radix(&input[2..], 16).ok()
}

fn read_num(input: &str) -> IResult<&str, Op> {
    map_res::<&str, _, _, _, (), _, _>(
        alt::<&str, _, _, _>((
            map_res(preceded(tag("0x"), hex_digit1), |x: &str| {
                i32::from_str_radix(&x, 16)
            }),
            map_res(terminated(hex_digit1, tag("h")), |x: &str| {
                i32::from_str_radix(&x, 16)
            }),
            // TODO: limit 0 and 1
            map_res(terminated(digit1, tag("b")), |x: &str| {
                i32::from_str_radix(&x, 2)
            }),
            map_res(digit1, |x: &str| x.parse::<i32>()),
        )),
        |x: i32| Ok(Op::Num(x)),
    )(input)
}
fn read_symbol(input: &str) -> IResult<&str, Op> {
    alt((
        // sign
        map_res::<&str, _, _, _, (), _, _>(tag("+"), |_: &str| Ok(Op::Add)),
        map_res::<&str, _, _, _, (), _, _>(tag("-"), |_: &str| Ok(Op::Minus)),
        map_res::<&str, _, _, _, (), _, _>(tag("*"), |_: &str| Ok(Op::Mul)),
        // para
        map_res::<&str, _, _, _, (), _, _>(tag("("), |_: &str| Ok(Op::PL)),
        map_res::<&str, _, _, _, (), _, _>(tag(")"), |_: &str| Ok(Op::PR)),
        map_res::<&str, _, _, _, (), _, _>(tag(" "), |_: &str| Ok(Op::Ignore)),
        map_res::<&str, _, _, _, (), _, _>(tag("\n"), |_: &str| Ok(Op::Ignore)),
        map_res::<&str, _, _, _, (), _, _>(tag("\r"), |_: &str| Ok(Op::Ignore)),
        map_res::<&str, _, _, _, (), _, _>(tag("\t"), |_: &str| Ok(Op::Ignore)),
    ))(input)
}

fn read_expr(input: &str) -> IResult<&str, Vec<Op>> {
    many0(alt((read_symbol, read_num)))(&input)
}

fn parse_expr(input: &str) -> Vec<Op> {
    read_expr(input)
        .unwrap()
        .1
        .into_iter()
        .filter(|x| x != &Op::Ignore)
        .collect::<Vec<Op>>()
}

fn calc(input: &str) -> i32 {
    let ops = parse_expr(input);
    cacl1(&ops)
}
#[derive(Debug, Clone, PartialEq)]
enum Op {
    Num(i32),
    Add,
    Minus,
    Mul,
    PL,
    PR,
    Ignore,
}

pub use Op::*;

fn cacl1(ops: &Vec<Op>) -> i32 {
    let pars: Vec<Op> = ops
        .clone()
        .into_iter()
        .filter(|x| x == &PL || x == &PR)
        .collect();
    if pars.len() % 2 != 0 {
        panic!("bad pars");
    }
    let mut pl_idx = None;
    let mut pr_idx = None;
    for (i, op) in ops.clone().iter().enumerate() {
        if op == &PL {
            pl_idx = Some(i);
        }
        if op == &PR {
            if pl_idx.is_none() {
                panic!("error pr");
            }
            pr_idx = Some(i);
            let exp = &ops.clone()[pl_idx.unwrap() + 1..pr_idx.unwrap()];
            let mut new_ops = ops.clone();
            new_ops.splice(
                pl_idx.unwrap()..=pr_idx.unwrap(),
                vec![Num(cacl1(&Vec::from(exp)))],
            );
            return cacl1(&new_ops);
        }
    }
    // dbg!(&ops, &ops.len());
    if ops.len() == 3 {
        if let Num(left) = ops[0] {
            if let Num(right) = ops[2] {
                return match ops[1] {
                    Add => left + right,
                    Minus => left - right,
                    Mul => left * right,
                    _ => unimplemented!("unknow ops: {:?}", &ops),
                };
            }
        }
    } else if ops.len() == 1 {
        if let Some(&Num(v)) = ops.get(0) {
            return v;
        } else {
            panic!("wtf");
        }
    }
    let add_parts: Vec<&[Op]> = ops.split_inclusive(|x| x == &Add || x == &Minus).collect();
    // dbg!(&add_parts);
    let mut sum = 0;
    let mut op = Add;
    for part in add_parts {
        let mut v: Vec<Op> = vec![];
        if part.ends_with(&[Add]) {
            v.extend_from_slice(&part[..part.len() - 1]);
        } else if part.ends_with(&[Minus]) {
            v.extend_from_slice(&part[..part.len() - 1]);
        } else {
            v.extend_from_slice(&part[..part.len()]);
        }
        let t = cacl1(&v);
        if op == Add {
            sum += t;
        } else if op == Minus {
            sum -= t;
        }
        if part.ends_with(&[Add]) {
            op = Add;
        } else if part.ends_with(&[Minus]) {
            op = Minus;
        } else {
            //ignore
        }
    }
    return sum;

    // return 1000;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_hex() {
        assert_eq!(calc("0x10 *0x2"), 32);
    }
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        assert_eq!(calc("1+1"), 2);
        // Num(1) Add Num(1)
        assert_eq!(cacl1(&vec![Num(1), Add, Num(1)]), 2);

        assert_eq!(calc("1+2*3"), 7);
        // Num(1) Add Num(2) Mul Num(3)
        assert_eq!(cacl1(&vec![Num(1), Add, Num(2), Mul, Num(3)]), 7);

        assert_eq!(calc("(1+2)*3"), 9);
        // PL Num(1) Add Num(2) PR Mul Num(3)
        assert_eq!(cacl1(&vec![PL, Num(1), Add, Num(2), PR, Mul, Num(3)]), 9);

        assert_eq!(calc("1+2*3-4*2-3"), -4);
        // Num(1) Add Num(2) Mul Num(3) Minus Num(4) Mul Num(2) Minus Num(3)
        assert_eq!(
            cacl1(&vec![
                Num(1),
                Add,
                Num(2),
                Mul,
                Num(3),
                Minus,
                Num(4),
                Mul,
                Num(2),
                Minus,
                Num(3)
            ]),
            -4
        );

        assert_eq!(
            parse_expr("( 0x12- 0x10) *4"),
            vec![PL, Num(18,), Minus, Num(16,), PR, Mul, Num(4,),]
        );
        assert_eq!(calc("( 0x12- 0x10) *4"), 8);
        assert_eq!(calc("1 + ( 0x12- 0x10) *4"), 9);

        assert_eq!(calc("0xa0+1"), 0xa1);

        assert_eq!(calc("80h+1"), 0x81);
        assert_eq!(calc("80+1"), 81);

        assert_eq!(calc("1001b"), 9);
    }
}
