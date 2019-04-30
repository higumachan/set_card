#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 3, 5);
    }
    #[test]
    #[should_panic]
    fn it_not_works() {
        assert_eq!(2 + 3, 6);
    }
}
