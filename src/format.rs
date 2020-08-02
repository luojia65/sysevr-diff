/*

+    if(....)
+        return ....;
或者
+    if(....) {
+        return ....;
+    }

*/

fn add_lines(input: &str) -> impl Iterator<Item = &str> {
    input.lines()
        .filter(|x| x.starts_with("+"))
        .map(|x| {
            let (_add, ans) = x.split_at(1);
            ans
        })
}

#[derive(Debug)]
pub struct AddIfReturn<'a> {
    pub condition: &'a str
}

pub fn add_if_return(input: &str) -> Vec<AddIfReturn> {
    let mut last_line = "";
    let mut ans = Vec::new();
    for line in add_lines(input) {
        if last_line.trim().starts_with("if") && line.trim().starts_with("return") {
            let idx_start = last_line.find("(");
            let idx_end = last_line.rfind(")");
            if let (Some(idx_start), Some(idx_end)) = (idx_start, idx_end) {
                let mat = AddIfReturn {
                    condition: &last_line[(idx_start + 1)..idx_end]
                };
                ans.push(mat)
            }
        }
        last_line = line;
    }
    ans
} 

/*

-    while (len > 0) {
+    while (len > 0 && --maxloop > 0) {

*/

#[derive(Debug)]
pub struct WhileOrFor<'a> {
    pub mark: &'a str, // "for" or "while"
    pub cond_sub: &'a str,
    pub cond_add: &'a str,
    pub block: String,
}

pub fn while_or_for(input: &str) -> Vec<WhileOrFor> {
    let mut ret = Vec::new();
    let mut iter = input.lines().peekable();
    while let Some(cur) = iter.next() {
        let nxt = iter.peek();
        if let Some(nxt) = nxt {
            if !(cur.starts_with('-') && nxt.starts_with('+')) {
                continue
            }
            // dbg!(cur);
            // dbg!(nxt);
            let (_sub, cur_1) = cur.split_at(1);
            let (_add, nxt_1) = nxt.split_at(1);
            if !cur_1.trim().starts_with("while") && !cur_1.trim().starts_with("for") {
                continue
            }
            // dbg!(cur_1);
            let idx_start = cur_1.find("(");
            let idx_end = cur_1.rfind(")");
            let idx_start_2 = nxt_1.find("(");
            let idx_end_2 = nxt_1.rfind(")");
            if let (Some(idx_start), Some(idx_end), Some(idx_start_2), Some(idx_end_2)) = 
                (idx_start, idx_end, idx_start_2, idx_end_2) 
            {
                let (mark, _rem) = cur_1.split_at(idx_start);
                let mark = mark.trim();
                let cond_sub = &cur_1[(idx_start + 1)..idx_end];
                let cond_add = &nxt_1[(idx_start_2 + 1)..idx_end_2];
                // dbg!(mark);
                // dbg!(cond_sub);
                // dbg!(cond_add);
                let ans = WhileOrFor {
                    mark,
                    cond_sub,
                    cond_add,
                    block: String::new(),
                };
                ret.push(ans);
            }
        } else {
            continue
        }
    }
    ret
}

/*
    if(val op val)，而且左边是变量，左边的val是关键变量
    如果左边是 val + const，val是关键变量
    if(func(val))，（还没有明确）val是关键变量
    if((val1 op val) op (val2 op val))，（复杂，不考虑）val1、val2
*/

// pub fn add_if_return_vals(if_input: &str) -> Vec<&str> {
//     let mut ans = Vec::new();
    
//     ans
// }
