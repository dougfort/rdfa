pub trait State<T> {
    fn curr(&self) -> T;
    fn next(&self, input: T) -> Option<Self> where Self: std::marker::Sized;
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

    impl State<u8> for UsizeState {
        fn next(&self, input: u8) -> Option<Self> {
            Some(UsizeState::new(self.curr + input))
        }
        fn curr (&self) -> u8 {
            self.curr            
        }
    }  

    #[test]
    fn usize_next() {
        let u1 = UsizeState::new(5);
        let result = u1.next(1);
        assert!(result.is_some());
        if let Some(state) = result {
            assert_eq!(state.curr(), 6);
        }
    }
}
