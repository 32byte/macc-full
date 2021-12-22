// module exports
pub mod blockchain;
pub mod ecdsa;
pub mod hashes;
pub mod script;
pub mod settings;

// helper functions

// python lib
/*
use blockchain::Transaction;
use pyo3::prelude::*;
use serde_json;

#[pyfunction]
fn tx_hash(txj: String, h: usize) -> PyResult<[u8; 32]> {
    let tx: Transaction = serde_json::from_str(&txj).unwrap();
    Ok(tx.hash(h))
}

#[pymodule]
fn macc_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tx_hash, m)?)?;

    Ok(())
}
 */
