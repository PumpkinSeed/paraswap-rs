extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod paraswap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
