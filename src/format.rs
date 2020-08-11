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

#[derive(Clone, Debug)]
pub struct AddIfReturn<'a> {
    pub cond: String,
    pub input: &'a str,
    pub idx: usize,
}

pub fn add_if_return<'a>(input: &'a str) -> Vec<AddIfReturn<'a>> {
    let mut ret = Vec::new();
    let mut cur = 0;
    loop { 
        cur = next_line(input, cur);
        if cur >= input.len() {
            break
        }
        let nxt = next_line(input, cur);
        if nxt >= input.len() {
            break
        }
        let cur_str = &input[cur..nxt]; 
        if !cur_str.starts_with('+') {
            continue
        }
        let (_add_sym, add) = cur_str.split_at(1);
        if !add.trim().starts_with("if") {
            continue
        }
        let idx_start_add = add.find("(");
        let idx_end_add = add.rfind(")");
        let cond = if let (Some(idx_start_add), Some(idx_end_add)) = (idx_start_add, idx_end_add) {
            &add[idx_start_add + 1 ..= idx_end_add - 1]
        } else {
            continue;
        }.to_string();
        if idx_start_add == None {
            continue;
        }
        let idx_start_add = idx_start_add.unwrap() + cur + 2;
        let mut line_start = idx_start_add;
        let mut line_end = line_start;
        let mut cond_add = String::new();
        let nxt_line = &input[line_start..next_line(input, line_start)];
        loop {
            line_end = next_line(input, line_start);
            let nxt_line = &input[line_start..line_end];
            if nxt_line.starts_with('+') { 
                cond_add += &nxt_line[1..];
                line_start = line_end;
            } else {
                break
            }
        }
        // dbg!(&cond_add);
        
        let ans = AddIfReturn { cond, input, idx: cur };
        ret.push(ans);
    }
    ret
} 

/*

-    while (len > 0) {
+    while (len > 0 && --maxloop > 0) {
        ...
     }

*/

// 下一行的开头位置
// 因为Rust标准库里的切行函数没有这一块，只能自己写
fn next_line(input: &str, cur_idx: usize) -> usize {
    assert!(cur_idx < input.len());
    for i in cur_idx..input.len() {
        let cur_ch = &input[i..i+1];
        if cur_ch == "\r" {
            continue
        } // 其实可以不写，\r不算
        if cur_ch == "\n" {
            return i + 1 // 当前的是\n，下一个才是新行
        }
    }
    input.len()
}

#[derive(Clone, Debug)]
pub struct ModifyWhileFor<'a> {
    pub mark: &'a str, // "for" or "while"
    pub cond_sub: &'a str,
    pub cond_add: &'a str,
    pub block: &'a str,
}

pub fn modify_while_for(input: &str) -> Vec<ModifyWhileFor> {
    let mut ret = Vec::new();
    let mut cur = 0;
    loop { 
        cur = next_line(input, cur);
        if cur >= input.len() {
            break
        }
        let nxt = next_line(input, cur);
        if nxt >= input.len() {
            break
        }
        let cur_str = &input[cur..nxt]; // "-    while (len > 0) {\r\n"
        let nxtnxt = next_line(input, nxt);
        let nxt_str = &input[nxt..nxtnxt]; // "+    while (len > 0 && --maxloop > 0) {\r\n"
        if !(cur_str.starts_with('-') && nxt_str.starts_with('+')) {
            continue
        }
        let (_sub_sym, sub) = cur_str.split_at(1);
        let (_add_sym, add) = nxt_str.split_at(1);
        if !sub.trim().starts_with("while") && !sub.trim().starts_with("for") {
            continue
        }
        let idx_start_sub = sub.find("(");
        let idx_end_sub = sub.rfind(")");
        let idx_start_add = add.find("(");
        let idx_end_add = add.rfind(")");

        if let (Some(idx_start_sub), Some(idx_end_sub), Some(idx_start_add), Some(idx_end_add)) = 
            (idx_start_sub, idx_end_sub, idx_start_add, idx_end_add) 
        {
            let (mark, _rem) = sub.split_at(idx_start_sub);
            let mark = mark.trim();
            let cond_sub = &sub[(idx_start_sub + 1)..idx_end_sub];
            let cond_add = &add[(idx_start_add + 1)..idx_end_add];
            // dbg!(cond_sub, cond_add); // cond_sub = "len > 0" cond_add = "len > 0 && --maxloop > 0"
            // dbg!(&add[idx_end_add+1..idx_end_add+2]); // +0..=+1 = ")"
            let mut begin = nxt + idx_end_add + 2; // +1 = ")"
            let mut end = begin;
            let mut depth = 0;
            let mut has_region_mark = true; // 有没有大括号
            loop {
                if end >= input.len() {
                    break
                }
                let cur_ch = &input[end..end+1];
                // dbg!(&input[begin..end]);
                if cur_ch.trim() == "" {
                    end += 1;
                    continue // 空字符，可以是空格或者换行，跳过
                }
                // dbg!(cur_ch, begin, end);
                if !has_region_mark && cur_ch == ";" {
                    end += 1; // 包含这个分号
                    break
                }
                if depth == 0 && cur_ch != "{" {
                    has_region_mark = false; // 没有最大的大括号的特殊情况
                    // dbg!(has_region_mark); // false
                }
                if cur_ch == "{" {
                    depth += 1;
                }
                if cur_ch == "}" {
                    depth -= 1;
                }
                if depth == 0 && has_region_mark {
                    break
                }
                end += 1;
            }
            let block = &input[begin..end].trim();
            let ans = ModifyWhileFor {
                mark,
                cond_sub,
                cond_add,
                block
            };
            ret.push(ans);
        }
    }
    ret
}

#[derive(Clone, Debug)]
pub struct ModifyIf {
    pub cond_sub: String,
    pub cond_add: String,
    pub block: String,
}

pub fn modify_if(input: &str) -> Vec<ModifyIf> {
    let mut ret = Vec::new();
    
    let mut cur = 0;
    loop { 
        cur = next_line(input, cur);
        if cur >= input.len() {
            break
        }
        let nxt = next_line(input, cur);
        if nxt >= input.len() {
            break
        }
        let cur_str = &input[cur..nxt]; 
        if !cur_str.starts_with('-') {
            continue
        }
        let (_sub_sym, sub) = cur_str.split_at(1);
        if !sub.trim().starts_with("if") {
            continue
        }
        let idx_start_sub = sub.find("(");
        if idx_start_sub == None {
            continue;
        }
        let idx_start_sub = idx_start_sub.unwrap() + cur + 2;
        let mut cond_sub = String::new();
        cond_sub += &input[idx_start_sub..nxt];
        let mut line_start = nxt;
        let mut line_end = nxt; // not necessary
        let mut depth = 0; // 括号的深度
        loop {
            line_end = next_line(input, line_start);
            let nxt_line = &input[line_start..line_end];
            if nxt_line.starts_with('-') {
                cond_sub += &nxt_line[1..];
                line_start = line_end;   
            } else {
                break
            }
        }
        // dbg!(cond_sub);
        let mut cond_add = String::new();
        let nxt_line = &input[line_start..next_line(input, line_start)];
        let mut skip = 0;
        if nxt_line.starts_with('+') {
            if !&nxt_line[1..].trim().starts_with("if") {
                continue
            }
            // dbg!(nxt_line);
            let idx_start_add = nxt_line.find("(");
            if idx_start_add == None {
                continue;
            }
            skip = idx_start_add.unwrap();
        } else {
            continue
        }
        loop {
            line_end = next_line(input, line_start);
            let nxt_line = &input[line_start..line_end];
            if nxt_line.starts_with('+') { 
                cond_add += &nxt_line[1..][skip..];
                line_start = line_end;
            } else {
                break
            }
            if skip != 0 {
                skip = 0;
            }
        }
        // dbg!(cond_add);
        let mut begin = line_start; // +1 = ")"
        let mut end = begin;
        let mut depth = 0;
        let mut has_region_mark = true; // 有没有大括号
        loop {
            if end >= input.len() {
                break
            }
            let cur_ch = &input[end..end+1];
            // dbg!(&input[begin..end]);
            if cur_ch.trim() == "" {
                end += 1;
                continue // 空字符，可以是空格或者换行，跳过
            }
            // dbg!(cur_ch, begin, end);
            if !has_region_mark && cur_ch == ";" {
                end += 1; // 包含这个分号
                break
            }
            if depth == 0 && cur_ch != "{" {
                has_region_mark = false; // 没有最大的大括号的特殊情况
                // dbg!(has_region_mark); // false
            }
            if cur_ch == "{" {
                depth += 1;
            }
            if cur_ch == "}" {
                depth -= 1;
            }
            if depth == 0 && has_region_mark {
                break
            }
            end += 1;
        }
        let block = (&input[begin..end].trim()).to_string();
        
        let ans = ModifyIf { cond_sub, cond_add, block };
        ret.push(ans);
    }
    ret
}

#[derive(Clone, Debug)]
pub struct ModifyValueAssign<'a> {
    pub sub_left: &'a str,
    pub sub_right: &'a str,
    pub add_left: &'a str,
    pub add_right: &'a str,
}

pub fn modify_value_assign(input: &str) -> Vec<ModifyValueAssign> {
    let mut ret = Vec::new();
    let mut iter = input.lines().peekable();
    while let Some(cur) = iter.next() {
        let nxt = iter.peek();
        if let Some(nxt) = nxt {
            if !(cur.ends_with(';') && nxt.ends_with(';')) {
                continue
            }
            let cur_1 = &cur[1..cur.len() - 1];
            let nxt_1 = &nxt[1..nxt.len() - 1];
            let idx1 = cur_1.find("=");
            let idx2 = nxt_1.find("=");

            if let (Some(idx1), Some(idx2)) = (idx1, idx2) {
                let (al, ar) = cur_1.split_at(idx1);
                let (_eq, ar) = ar.split_at(1);
                let (sl, sr) = nxt_1.split_at(idx2);
                let (_eq, sr) = sr.split_at(1);
                // dbg!(mark);
                // dbg!(cond_sub);
                // dbg!(cond_add);
                let ans = ModifyValueAssign {
                    sub_left: sl.trim(),
                    sub_right: sr.trim(),
                    add_left: al.trim(),
                    add_right: ar.trim(),
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
