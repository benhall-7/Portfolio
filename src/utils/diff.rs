use diff::{Diff, VecDiff};

fn reduce_string(s: String) -> Vec<char> {
    // cut out windows style newlines
    s.replace("\r\n", "\n").chars().collect()
}

pub fn get_diff(a: String, b: String) -> VecDiff<char> {
    let a = reduce_string(a);
    let b = reduce_string(b);
    a.diff(&b)
}
