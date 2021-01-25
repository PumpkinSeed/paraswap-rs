pub trait Paraswap {
    fn list_tokens();
    fn get_rate();
    fn swap();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
