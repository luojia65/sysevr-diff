// 把“@@ -312,18 +312,42 @@ static int ssd0323_load(QEMUFile *f, void *opaque, int version_id)”
// 包含的一段内容识别出来

pub fn code_slices(input: &str) -> impl Iterator<Item = &str> {
    input.split("\n@@")
        .skip(1)
        .map(|segment| { 
            let idx = segment.find("\n").unwrap(); // todo: unwrap
            let (_, ans) = segment.split_at(idx + 1); // skip \n
            ans
        })
}

