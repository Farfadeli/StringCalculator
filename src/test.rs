#[cfg(test)]
mod test{
    use crate::add;

    #[test]
    fn working(){
        assert_eq!(0, add("".to_string()))
    }
}