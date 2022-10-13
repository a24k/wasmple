#[no_mangle]
pub extern "C" fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[no_mangle]
pub extern "C" fn sub(left: i32, right: i32) -> i32 {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_add() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn it_works_sub() {
        let result = sub(2, 3);
        assert_eq!(result, -1);
    }
}
