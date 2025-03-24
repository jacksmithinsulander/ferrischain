use crate::controllers::wallet::Wallet;
use k256::schnorr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet() {
        let seed = String::from("Heyy");
        let wallet = match Wallet::new(seed) {
            Ok(w) => w,
            Err(_) => panic!()
        };

        let my_message = String::from("wazzup");

        let signature = wallet.clone().sign_message(my_message.clone()).unwrap();

        println!("Signature: {:?}", signature);

        let res = wallet.verify_message(my_message, signature);

        println!("Result: {:?}", res)
    }
}