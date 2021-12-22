use secp256k1::bitcoin_hashes::hex::{FromHex, ToHex};

use crate::hashes;

fn sha256(stack: &mut Vec<String>) -> bool {
    let to_hash = match stack.pop() {
        Some(x) => x,
        None => return false,
    };

    let to_hash_bytes = match Vec::from_hex(&to_hash) {
        Ok(x) => x,
        Err(_) => return false,
    };

    let hashed = hashes::sha256(&to_hash_bytes);

    stack.push(hashed.to_hex());
    true
}

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

/* Note: And operator probably not needed
fn op_and(stack: &mut Vec<String>) -> bool {
    let val1 = match stack.pop() {
        Some(x) => match x.parse::<bool>() {
            Ok(x) => x,
            Err(_) => return false,
        },
        None => return false,
    };

    let val2 = match stack.pop() {
        Some(x) => match x.parse::<bool>() {
            Ok(x) => x,
            Err(_) => return false,
        },
        None => return false,
    };

    val1 && val2
}
*/

pub fn eval(script: String) -> Option<Vec<String>> {
    let mut stack: Vec<String> = Vec::new();

    for val in script.split(" ") {
        if val == "" {
            continue;
        }
        if !match val.to_lowercase().as_str() {
            "sha256" => sha256(&mut stack),
            "op_eq" => op_eq(&mut stack),
            // "op_and" => op_and(&mut stack),
            _ => {
                stack.push(val.to_string());
                true
            }
        } {
            return None;
        }
    }
    Some(stack)
}
