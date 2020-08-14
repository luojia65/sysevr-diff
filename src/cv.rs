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
            "+" | "-" | "*" | "/" | "%" | ">" | "<" | "=" | "(" | ")" | "," => Ty::Symbol,
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
                "+" | "-" | "*" | "/" | "%" | ">" | "<" | "=" | "(" | ")" | "," => {
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

//
//
pub fn gen_modify_if<'a>(a: &ModifyIf) -> Vec<Cv<'a>> {
    let mut ret = Vec::new();

    ret
}
