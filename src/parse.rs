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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn code_slices_test() {
        let input = "commit aaaaaaaa666666666
Author:    Person
AuthorDate: Date
Commit:     Person
CommitDate: Date

    comment

    (cherry picked from commit 8888888877777777)
    Signed-off-by: xxx <xxx@xxx.com>

diff --git a/file.c b/file.c
index aaaaaaaaaa..bbbbbbbbbb 666666
--- a/file.c
+++ b/file.c
@@ -1512,7 +1512,7 @@ static uint32_t data1
 a
     b;
     c;
-    d;
+    e;
 
     f
@@ -1521,8 +1521,8 @@ static uint32_t fn1
         return 0;
     }
     a;
@@ -1867,10 +1867,13 @@ static void fn2
-    sub1
+    add1
         /* comment2 */
@@ -1970,7 +1973,7 @@ static uint8_t data2
 static void fn4
 a
     b;
-    c;
+    d;
@@ -2019,7 +2022,9 @@ static void fn3
     a
     b
     c
        ";
        let mut iter = code_slices(input);
        assert_eq!(iter.next(), Some(" a\n     b;\n     c;\n-    d;\n+    e;\n \n     f"));
        assert_eq!(iter.next(), Some("         return 0;\n     }\n     a;"));
        assert_eq!(iter.next(), Some("-    sub1\n+    add1\n         /* comment2 */"));
        assert_eq!(iter.next(), Some(" static void fn4\n a\n     b;\n-    c;\n+    d;"));
        assert_eq!(iter.next(), Some("     a\n     b\n     c\n        "));
        assert_eq!(iter.next(), None);
    }
}
