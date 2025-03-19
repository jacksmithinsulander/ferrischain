use crate::controllers::transaction::Transaction;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction() {
        let val = 5555;
        let t = Transaction::new(
            String::from("098413"),
            String::from("429387"),
            val,
            0,
            30,
            String::from("902384"),
            String::from("")
        );

        assert_eq!(t.value, val)
    }
}