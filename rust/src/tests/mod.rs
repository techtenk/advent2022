#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_relevant_cycle() {
        assert(20.is_relevant(), true);
    }

}