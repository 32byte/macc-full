use secp256k1::{All, Secp256k1};

use crate::ecdsa;
use crate::hex::FromHex;

fn op_eq(stack: &mut Vec<String>) -> Option<bool> {
    let val1 = stack.pop()?;
    let val2 = stack.pop()?;

    Some(val1 == val2)
}

fn to_addr(stack: &mut Vec<String>) -> Option<()> {
    // get the public key in hex format
    let pb_key = stack.pop()?;
    // convert to array of bytes
    let pk_bytes = Vec::from_hex(&pb_key).ok()?;
    // convert to address
    let addr = ecdsa::pb_key_to_addr(&pk_bytes);
    // push onto the stack
    stack.push(addr);

    Some(())
}

fn verify_signature(stack: &mut Vec<String>, secp: &Secp256k1<All>) -> Option<bool> {
    // pop data from stack
    let msg_str = stack.pop()?;
    let sig_str = stack.pop()?;
    let pb_key_str = stack.pop()?;

    // parse data
    let msg_bytes = Vec::from_hex(&msg_str).ok()?;
    let msg = ecdsa::msg_from_bytes(&msg_bytes).ok()?;

    let sig_bytes = Vec::from_hex(&sig_str).ok()?;
    let sig = ecdsa::sig_from_bytes(&sig_bytes).ok()?;

    let pk_bytes = Vec::from_hex(&pb_key_str).ok()?;
    let pb_key = ecdsa::pb_key_from_bytes(&pk_bytes).ok()?;

    // push the public key back onto the stack
    stack.push(pb_key_str);

    Some(ecdsa::valid_signature(secp, &msg, &sig, &pb_key))
}

pub fn eval(script: String) -> Option<Vec<String>> {
    let mut stack: Vec<String> = Vec::new();
    let secp = ecdsa::create_secp();

    for val in script.split(' ') {
        if val.is_empty() {
            continue;
        }
        if !match val.to_lowercase().as_str() {
            "eq" => op_eq(&mut stack).unwrap_or(false),
            "to_addr" => to_addr(&mut stack).is_some(),
            "verify_sig" => verify_signature(&mut stack, &secp).unwrap_or(false),
            _ => {
                stack.push(val.to_string());
                true
            }
        } {
            log::debug!("Ivalid Script  at {}: {}", val, script);
            return None;
        }
    }
    Some(stack)
}
