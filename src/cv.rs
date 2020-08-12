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
pub fn gen_add_if_return(a: AddIfReturn) -> Vec<Cv> {
    let mut ret = Vec::new();
    
    
    ret
}

// 输出："s", "->", "start_addr", "<", "0"等等
struct GetSyms<'a> {
    cur: &'a str
}
fn get_syms(input: &str) -> GetSyms {
    GetSyms{ cur: input }
}
impl<'a> Iterator for GetSyms<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        self.cur = self.cur.trim();
        if self.cur == "" {
            return None
        }
        let mut ans_idx = 0;
        let ch_cur = &self.cur[ans_idx..ans_idx+1];
        if ch_cur == "-" {
            if ans_idx+1 < self.cur.len() {

            } else {
                
            }
        }
        todo!("a tokenizer")
    }
}
