pub static SOME_NUMBER: Option<i32> = Some(5);
pub static SOME_CHAR: Option<char> = Some('e');
pub static ABSENT_NUMBER: Option<i32> = None;

pub fn sum_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x + y),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}
