
pub mod event_builder;

pub mod other {
    pub fn add(x:i64, y:i64) -> i64 {
        return x+y;
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(crate::other::add(2,2), 4);
    }
}



