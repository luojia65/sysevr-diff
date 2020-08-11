use crate::format::*;

pub struct Cv<'a> {
    pub src: &'a str,
    pub var_name: &'a str,
}

// 左边的变量就可以了
pub fn gen_add_if_return<'a>(a: AddIfReturn, src: &'a str) -> Vec<Cv<'a>> {
    let mut ret = Vec::new();


    ret
}
