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

pub struct WhileOrFor<'a> {
    pub condition: &'a str,
    pub block: &'a str,
}

pub fn while_or_for(input: &str) -> Vec<WhileOrFor> {
    todo!()
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
