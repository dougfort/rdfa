pub trait State {
    fn curr(&self) -> u8;
    fn next(&self, input: u8) -> Self;
}


#[cfg(test)]
mod tests {
    use super::*;

    struct UsizeState {
        curr: u8,        
    }

    impl UsizeState {
        fn new(curr: u8) -> Self {
            UsizeState { curr }
        }
    }

    impl State for UsizeState {
        fn next(&self, input: u8) -> Self {
            UsizeState::new(self.curr + input)
        }
        fn curr (&self) -> u8 {
            self.curr            
        }
    }  

    #[test]
    fn usize_next() {
        let u1 = UsizeState::new(5);
        let result = u1.next(1);
        assert_eq!(result.curr(), 6);
    }
}
