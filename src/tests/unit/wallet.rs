use crate::controllers::wallet::Wallet;
use k256::schnorr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet() {
        let seed = String::from("Hey");
        let wallet = match Wallet::new(seed) {
            Ok(w) => w,
            Err(_) => panic!()
        };

        let my_message = String::from("wazzup");

        let signature = wallet.sign_message(my_message).unwrap();

        println!("{:?}", signature);
    }
}