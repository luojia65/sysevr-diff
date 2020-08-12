use crate::format::*;

pub struct Cv<'a> {
    pub name: String,
    pub input: &'a str,
    pub idx: usize
}

// 左边的变量就可以了
// 两种情况：FUNC(var)，是var
// a || b，继续寻找a或者b
// a op b，是a
// (a)，继续寻找a
pub fn gen_add_if_return<'a>(a: &'a AddIfReturn) -> Vec<Cv<'a>> {
    let mut ret = Vec::new();
    for sym in get_syms(&a.cond) {
        dbg!(&sym);
    }
    
    ret
}

// 输出："s", "->", "start_addr", "<", "0"等等
#[derive(Debug)]
struct GetSyms<'a> {
    cur: &'a str
}
fn get_syms(input: &str) -> GetSyms {
    GetSyms{ cur: input }
}
impl<'a> Iterator for GetSyms<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.cur = self.cur.trim();
        if self.cur == "" {
            return None
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
        Some(ans)
    }
}
