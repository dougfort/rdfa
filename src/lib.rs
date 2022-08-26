pub trait State<T> {
    fn curr(&self) -> T;
    fn next(&self, input: T) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    const FUNCS: [fn(u8) -> u8; 3] = [
        |x| match x {
            0 => 0,
            _ => 1,
        },
        |x| match x {
            0 => 2,
            _ => 0,
        },
        |x| match x {
            0 => 1,
            _ => 2,
        },
    ];

    struct UsizeState {
        curr: u8,
    }

    impl UsizeState {
        fn new(curr: u8) -> Self {
            UsizeState { curr }
        }
    }

    impl State<u8> for UsizeState {
        fn curr(&self) -> u8 {
            self.curr
        }
        fn next(&self, input: u8) -> Self {
            UsizeState::new((FUNCS[self.curr as usize])(input))
        }
    }

    #[test]
    fn wikipedia_example() {
        let result = [1, 0, 0, 1]
            .iter()
            .fold(UsizeState::new(0), |s, x| s.next(*x));
        assert_eq!(result.curr(), 0);
    }
}
