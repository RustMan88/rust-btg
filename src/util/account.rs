use crate::util::key::{PublicKey,PrivateKey};
use crate::network::Network;
use crate::address::address::Address;
use secp256k1::Secp256k1;
//use rand::{thread_rng,Rng};
use secp256k1::rand::OsRng;

#[derive(Debug)]
pub struct Account {
    pub public_key: PublicKey,
    pub private_key: PrivateKey,
    pub address: String,
}


impl Account {

    pub fn generate_p2pkh_accounts(network:Network, num :u32) -> Vec<Account> {

        let s = Secp256k1::new();
        let mut acts = vec![];

        for i in 0..num {
            let p =s.generate_keypair(&mut OsRng::new().unwrap());

            let prv_key = PrivateKey{
                compressed:false,
                network,
                key:p.0,
            };

            let pub_key = PublicKey{
                compressed:false,
                key:p.1,
            };

            let act = Account{
                private_key:prv_key,
                public_key:pub_key,
                address:Address::p2pkh(&pub_key,network).to_string(),
            };

           // println!("priv:{}, public:{},address:{}",act.private_key,act.public_key,act.address);

            acts.push(act);
        }

        return acts
    }
}




#[cfg(test)]
mod test {
    use crate::network::Network;
    use super::Account;
    use std::time;

    #[test]
    fn batch_generate_keys() {
        let start = time::SystemTime::now();//获取开始时间
        let a = Account::generate_p2pkh_accounts(Network::Mainnet,10000);
        let end = time::SystemTime::now();//获取结束时间
        println!("time spend: {:?}", end.duration_since(start).unwrap());
    }
}