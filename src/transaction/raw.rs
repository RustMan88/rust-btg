use crate::messages::{Tx, TxIn,TxOut,OutPoint};
use crate::script::{Script};
use crate::address::addr_decode;
use crate::network::Network;
use crate::transaction::generate_signature2;
use crate::transaction::p2pkh::{self,create_pk_script, create_sig_script};
use crate::transaction::sighash::{sighash, SigHashCache, SIGHASH_FORKID, SIGHASH_ALL,SIGHASH_NONE};
use crate::util::{hash160, Hash256,Amount,key};
use crate::{TxInputReq,TxOutputReq};
use crate::Error;

// Use real values here
//let mut tx = Tx {
//inputs: vec![TxIn {
//..Default::default()
//}],
//..Default::default()
//};
//let private_key = [1; 32];
//let public_key = [1; 33];
//
//let pk_script = create_pk_script(&hash160(&public_key));
//let mut cache = SigHashCache::new();
//let sighash_type = SIGHASH_NONE | SIGHASH_FORKID;
//let sighash = sighash(&tx, 0, &pk_script.0, Amount(0), sighash_type, &mut cache).unwrap();
//let signature = generate_signature(&private_key, &sighash, sighash_type).unwrap();
//tx.inputs[0].sig_script = create_sig_script(&signature, &public_key);

fn get_outpoint(input : &TxInputReq) -> Result<OutPoint,Error>{
    Ok(OutPoint{
        hash: Hash256::decode(&input.txid).map_err(|_|Error::TxidParseError)?,
        index: input.index
    })
}

pub fn create_sign_rawtx(vins:&Vec<TxInputReq>,vouts:&Vec<TxOutputReq>,accounts:&Vec<key::Account>)-> Result<String, Error>{
    //检查入数量是否大于等于出数量
    let total_out = vouts.iter().fold(0, |acc, output| acc + output.value);
    let total_in = vins.iter().fold(0, |acc, input| acc + input.credit);

    if total_in < total_out {
        return Err(Error::NotEnoughAmount);
    }

    let mut tx = Tx {
        version: 2,
        inputs: vec![],
        outputs: vec![],
        lock_time: 0,
    };

    for i in 0..vins.len(){
        let vin = &vins[i];
        let op = get_outpoint(vin)?;
        let input = TxIn{
            prev_output: op,
            sig_script:      Script(vec![]),
            sequence:        0xFFFFFFFF,
        };
        tx.inputs.push(input);
    }

    for j in 0..vouts.len(){
        let vout = &vouts[j];
        let (addr_hash,addr_type) = addr_decode(&vout.address, Network::Mainnet).map_err(|_|Error::AddressParseError)?;

        let output = TxOut{
            amount: Amount(vout.value as i64),
            pk_script: p2pkh::create_pk_script(&addr_hash),
        };
        tx.outputs.push(output);
    }

    //签名
    for i in 0..vins.len(){
        let vin = &vins[i];
        let act = &accounts[i];

        let (from_addr_hash,addr_type) = addr_decode(&vin.address, Network::Mainnet).map_err(|_|Error::AddressParseError)?;
        let pk_script = create_pk_script(&from_addr_hash);
        let mut cache = SigHashCache::new();
        let sighash_type = SIGHASH_ALL| SIGHASH_FORKID;
        let sighash = sighash(&tx, i, &pk_script.0, Amount(vin.credit as i64), sighash_type, &mut cache).unwrap();

        //签名
        let signature = generate_signature2(&act.private_key.key, &sighash, sighash_type).unwrap();

        tx.inputs[i].sig_script = create_sig_script(&signature, &act.public_key.to_bytes());
    }

    Ok(tx.serialize_hex())
}
