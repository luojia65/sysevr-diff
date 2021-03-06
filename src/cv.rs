use crate::format::*;

pub struct Cv<'a> {
    pub name: String,
    pub input: &'a str,
    pub idx: usize
}

use core::fmt;
impl fmt::Debug for Cv<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cv")
         .field("name", &self.name)
         .field("idx", &self.idx)
         .field("input(len)", &self.input.len())
         .finish()
    }
}

// 左边的变量就可以了
// 两种情况：FUNC(var)，是var
// a || b，继续寻找a或者b
// a op b，是a
// (a)，继续寻找a

// 应该做成表达式树，只要找到<、>、<=、>=、==这些符号，
// 这个结点的左儿子如果是字母串，它就是关键变量
// 如果找关键变量找到一个“->”，假如它的左边和右边都是字母串，
// 应该把左右和箭头拼接起来，看作一个关键变量

// 其实从左往右扫就可以了
pub fn gen_add_if_return<'a>(a: &'a AddIfReturn) -> Vec<Cv<'a>> {
    let mut ret = Vec::new();
    let syms = get_syms(&a.cond).collect::<Vec<_>>();
    // dbg!(&syms);
    for i in 0..syms.len() {
        if syms.len() - i >= 2 {
            let (cur_idx, cur_str) = syms[i].clone();
            let (_nxt_idx, nxt_str) = syms[i + 1].clone();
            if is_ident(&cur_str) && is_cv_sym(&nxt_str) {
                // dbg!(&cur_str, &nxt_str);
                let cv = Cv {
                    name: cur_str,
                    input: a.input,
                    idx: cur_idx
                };
                ret.push(cv);
            }
        } else if syms.len() - i >= 4 {
            let (par_idx, par_str) = syms[i].clone();
            let (_, arrow_str) = syms[i + 1].clone();
            let (_, child_str) = syms[i + 2].clone();
            let (_, sym_str) = syms[i + 3].clone();
            if is_ident(&par_str) && arrow_str == "->" && is_ident(&child_str) && is_cv_sym(&sym_str) {
                let cv = Cv { 
                    name: par_str + "->" + &child_str,
                    input: a.input,
                    idx: par_idx
                };
                ret.push(cv);
            }
        }
    }
    ret
}

// fn is_digit(input: &str) -> bool {
//     input.chars().all(|e| e.is_digit(10))
// }

// 这个规则是自己规定的
fn is_cv_sym(input: &str) -> bool {
    match input {
        "+" | "-" | "*" | "/" | "%" | "<" | ">" | "<=" | ">=" | ")" | "," => true,
        _ => false,
    }
}

fn is_ident(input: &str) -> bool {
    input.chars().all(|e| e.is_alphabetic() || e == '_')
}

// 输出："s", "->", "start_addr", "<", "0"等等
#[derive(Debug)]
struct GetSyms<'a> {
    cur: &'a str,
    idx: usize,
}
fn get_syms(input: &str) -> GetSyms {
    GetSyms{ cur: input, idx: 0 }
}
impl<'a> Iterator for GetSyms<'a> {
    type Item = (usize, String);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.cur == "" {
                return None
            }
            let (head, rem) = self.cur.split_at(1);
            if head.trim() == "" {
                // dbg!(&head, &rem);
                self.cur = rem;
                self.idx += 1;
            } else {
                break
            }
        }
        let mut end = 0;
        let first_ch = &self.cur[0..=0];
        #[derive(Copy, Clone, Eq, PartialEq)]
        enum Ty {
            Number,
            Ident,
            Symbol,
        }
        let first_ty = match first_ch {
            "+" | "-" | "*" | "/" | "%" | ">" | "<" | "=" | "(" | ")" | "[" | "]" | "," | "&" => Ty::Symbol,
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => Ty::Number,
            " " | "\r" | "\n" => unreachable!(),
            _ => Ty::Ident,
        };
        loop {
            if end >= self.cur.len() {
                break
            }
            let cur_ch = &self.cur[end..=end];
            match cur_ch {
                "+" | "-" | "*" | "/" | "%" | ">" | "<" | "=" | "(" | ")" | "[" | "]" | "," | "&" => {
                    if let Ty::Symbol = first_ty {
                        end += 1;
                        continue
                    } else {
                        break
                    }
                }
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                    if let Ty::Number = first_ty {
                        end += 1;
                        continue
                    } else {
                        break
                    }
                }
                " " | "\r" | "\n" => break,
                _ => {
                    if let Ty::Ident = first_ty {
                        end += 1;
                        continue
                    } else {
                        break
                    }
                }
            }
        }
        let ans = String::from(&self.cur[..end]);
        self.cur = &self.cur[end..];
        self.idx += end;
        Some((self.idx, ans))
    }
}

// 循环条件的变量
// while(len > 0) => while(len > = && --maxloop > 0) ==> maxloop
// 循环条件内部修改了赋值的变量
// cmd_start =... => cmd_start
pub fn gen_while_for<'a>(ctx: &'a ModifyWhileFor) -> Vec<Cv<'a>> {
    let mut ret = Vec::new();
    let a = get_syms(ctx.cond_sub).collect::<Vec<_>>();
    let b = get_syms(ctx.cond_add).collect::<Vec<_>>();

    // LCS算法第一步，得到子序列索引数组
    let (la, lb) = (a.len(), b.len());
    let mut dp = vec![0; a.len() * b.len()];
    for (i, ca) in a.iter().enumerate() {
        for (j, cb) in b.iter().enumerate() {
            if i > 0 && j > 0 && ca == cb {
                dp[i*lb + j] = dp[(i - 1)*lb + j - 1] + 1
            } else if i > 0 && j > 0 {
                dp[i*lb + j] = usize::max(dp[(i - 1)*lb + j], dp[i*lb + j - 1]);
            }
        }
    }

    // for i in 0..la {
    //     println!("{:?}", &dp[i*lb..(i+1)*lb]);
    // }
    // LCS第二步，得到有差别的元素。注意的是这个算法是从后往前倒着输出的
    let mut sa = a.iter().rev().peekable();
    let mut sb = b.iter().rev().peekable();
    let mut i = la - 1;
    let mut j = lb - 1;
    while let (Some(ca), Some(cb)) = (sa.peek(), sb.peek()) {
        if i == 0 && j == 0 {
            break
        }
        if ca == cb {
            sa.next();
            sb.next();
            i -= 1;
            j -= 1;
        } else {
            if dp[i*lb + j - 1] > dp[(i-1)*lb + j] {
            println!("{:?}", cb);
                sb.next();
                j -= 1;
            } else {
            println!("  {:?}", ca);
                sa.next();
                i -= 1;
            }
        }
    }

    // print的是不同的部分
    // 不同的部分里包含的任何变量，都是关键变量
    // 下面还需要从While语句块的内容来生成
    // todo

    // 考虑一个情况：
    /*
    cond_sub: "p != &src[size-4]",
    cond_add: "p != &src[size]",
    */
    // 这时候关键变量应该是p，因为p是受变化影响的。
    // 我觉得这种情况往下，就是往左找到第一个比较运算符，然后再往左找第一个变量
    // 未解决问题，后面再考虑

    ret
}
// 也是运算符左侧的变量
//
pub fn gen_modify_if<'a>(ctx: &'a ModifyIf) -> Vec<Cv<'a>> {
    let mut ret = Vec::new();

    ret
}

/* 行内diff
fn main() {
    let a = "abcdefg";
    let b = "nbncueig";
    let la = a.chars().count();
    let lb = b.chars().count();
    let mut dp = vec![0; la * lb];
    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            if i > 0 && j > 0 && ca == cb {
                dp[i*lb + j] = dp[(i - 1)*lb + j - 1] + 1
            } else if i > 0 && j > 0 {
                dp[i*lb + j] = usize::max(dp[(i - 1)*lb + j], dp[i*lb + j - 1]);
            }
        }
    }
    for i in 0..la {
        println!("{:?}", &dp[i*lb..(i+1)*lb]);
    }
    let mut sa = a.chars().rev().peekable();
    let mut sb = b.chars().rev().peekable();
    let mut i = la - 1;
    let mut j = lb - 1;
    while let (Some(ca), Some(cb)) = (sa.peek(), sb.peek()) {
        if i == 0 && j == 0 {
            break
        }
        if ca == cb {
            sa.next();
            sb.next();
            i -= 1;
            j -= 1;
        } else {
            if dp[i*lb + j - 1] > dp[(i-1)*lb + j] {
            println!("{}", cb);
                sb.next();
                j -= 1;
            } else {
            println!("  {}", ca);
                sa.next();
                i -= 1;
            }
        }
    }
}

*/
