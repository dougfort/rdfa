pub trait State<T> {
    fn curr(&self) -> T;
    fn next(&self, input: T) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct UsizeState {
        curr: u8,
        funcs: [fn(u8) -> u8; 3],
    }

    impl UsizeState {
        fn new(curr: u8, funcs: [fn(u8) -> u8; 3]) -> Self {
            UsizeState { curr, funcs }
        }
    }

    impl State<u8> for UsizeState {
        fn next(&self, input: u8) -> Self {
            UsizeState::new((self.funcs[self.curr as usize])(input), self.funcs)
        }
        fn curr(&self) -> u8 {
            self.curr
        }
    }

    #[test]
    fn wikipedia_example() {
        // https://en.wikipedia.org/wiki/Deterministic_finite_automaton
        let funcs: [fn(u8) -> u8; 3] = [
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

        let result = [1u8, 0u8, 0u8, 1u8]
            .iter()
            .fold(UsizeState::new(0, funcs), |s, x| s.next(*x));
        assert_eq!(result.curr(), 0);
    }
}
