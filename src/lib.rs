
fn calc(input: &str) -> i32 {

    0
}
#[derive(Debug, Clone, PartialEq)]
enum Op {
    Num(i32),
    Add,
    Minus,
    Mul,
    PL,
    PR,
}

pub use Op::*;
fn cacl1(ops: &Vec<Op>) -> i32 {
    let pars:Vec<Op> = ops.clone().into_iter().filter(|x| x == &PL || x == &PR).collect();
    // dbg!(&pars);
    // check pars
    if pars.len() % 2 != 0 {
        panic!("bad pars");
    }
    let pl_idx = ops.clone().into_iter().position(|x| x == PL);
    if pl_idx.is_some() {
        let pr_idx = ops.clone()[pl_idx.unwrap()..].into_iter().position(|x| x == &PR);
        dbg!(pl_idx, pr_idx);
        let exp = &ops.clone()[pl_idx.unwrap()+1..pr_idx.unwrap()];
        let mut new_args = vec![Num(cacl1(&Vec::from(exp)))];
        new_args.extend_from_slice(&ops.clone()[pr_idx.unwrap()+1..]);
        return cacl1(&new_args);
    }
    // dbg!(&ops, &ops.len());
    if ops.len() == 3 {
        if let Num(left) = ops[0] {
            if let Num(right) = ops[2] {
                return match ops[1] {
                    Add => left + right,
                    Mul => left * right,
                    _ => unimplemented!(),
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
        let mut v:Vec<Op> = vec![];
        if part.ends_with(&[Add]) {
            v.extend_from_slice(&part[..part.len()-1]);
        } else if part.ends_with(&[Minus]) {
            v.extend_from_slice(&part[..part.len()-1]);
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
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        // assert_eq!(calc("1+1"), 2);
        // Num(1) Add Num(1)
        assert_eq!(cacl1(&vec![Num(1), Add, Num(1)]), 2);

        // assert_eq!(calc("1+2*3"), 7);
        // Num(1) Add Num(2) Mul Num(3)
        assert_eq!(cacl1(&vec![Num(1), Add, Num(2), Mul, Num(3)]), 7);

        
        // assert_eq!(calc("(1+2)*3"), 9);
        // PL Num(1) Add Num(2) PR Mul Num(3)
        assert_eq!(cacl1(&vec![ PL ,Num(1), Add, Num(2), PR, Mul, Num(3)]), 9);


        // assert_eq!(calc("1+2*3-4*2-3"), -4);
        // Num(1) Add Num(2) Mul Num(3) Minus Num(4) Mul Num(2) Minus Num(3)
        assert_eq!(cacl1(&vec![ Num(1), Add, Num(2), Mul, Num(3), Minus, Num(4), Mul, Num(2), Minus, Num(3)]), -4);

    }
}
