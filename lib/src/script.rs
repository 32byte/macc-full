/*
use bitcoin_hashes::hex::FromHex;

// use crate::ecdsa;

fn op_eq(stack: &mut Vec<String>) -> bool {
    let val1 = match stack.pop() {
        Some(x) => x,
        None => return false,
    };

    let val2 = match stack.pop() {
        Some(x) => x,
        None => return false,
    };

    val1 == val2
}

fn to_addr(stack: &mut Vec<String>) -> bool {
    let pub_key = match stack.pop() {
        Some(x) => x,
        None => return false,
    };

    let pk_bytes = match Vec::from_hex(&pub_key) {
        Ok(p) => p,
        Err(_) => return false,
    };

    let addr = ecdsa::pk_to_address(&pk_bytes);

    stack.push(addr);
    true
}


fn verify_signature(stack: &mut Vec<String>) -> bool {
    // pop data from stack
    let msg = match stack.pop() {
        Some(x) => x,
        None => return false,
    };
    let sig = match stack.pop() {
        Some(x) => x,
        None => return false,
    };
    let pub_key = match stack.pop() {
        Some(x) => x,
        None => return false,
    };

    // parse data
    let msg = match ecdsa::msg_from_hex(&msg) {
        Some(m) => m,
        None => return false,
    };
    let sig = match ecdsa::sig_from_hex(&sig) {
        Some(s) => s,
        None => return false,
    };
    let pk = match ecdsa::pk_from_hex(&pub_key) {
        Some(p) => p,
        None => return false,
    };

    Secp256k1::new().verify(&msg, &sig, &pk).is_ok()
}

pub fn eval(script: String) -> Option<Vec<String>> {
    let mut stack: Vec<String> = Vec::new();

    for val in script.split(" ") {
        if val == "" {
            continue;
        }
        if !match val.to_lowercase().as_str() {
            "op_eq" => op_eq(&mut stack),
            "to_addr" => to_addr(&mut stack),
            "verify_sign" => verify_signature(&mut stack),
            _ => {
                stack.push(val.to_string());
                true
            }
        } {
            log::error!("{}: {}", val, script);
            return None;
        }
    }
    Some(stack)
}
*/
pub fn eval(_script: String) -> Option<Vec<String>> {
    //TODO: implement this
    Some(Vec::new())
}