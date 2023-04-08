#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::day10::RelevantCycle;

    #[test]
    fn test_relevant_cycle() {
        assert_eq!((20 as usize).is_relevant(), true);
        assert_eq!((40 as usize).is_relevant(), false);
        assert_eq!((60 as usize).is_relevant(), true);
        assert_eq!((100 as usize).is_relevant(), true);
        assert_eq!((0 as usize).is_relevant(), false);
        assert_eq!((19 as usize).is_relevant(), false);
    }

}