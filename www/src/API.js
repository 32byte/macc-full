const NODE_URL = 'http://localhost:1111/';

function getBlockchain(start, stop) {
  let url = NODE_URL + 'blockchain?from=' + start + '&until=' + stop;

  return fetch(url, {
    method: 'GET',
    mode: 'cors'
  });
}

function getTxStore() {
  let url = NODE_URL + 'tx-store';

  return fetch(url, {
    method: 'GET',
    mode: 'cors'
  });
}

// TODO(IDEA): use tx_store endpoint
function parseBlockchain(blockchain, hasher) {
  let txs = new Map();

  blockchain.map((block, idx) => {
    block['transactions'].forEach(tx => {
      let txhash = hasher(JSON.stringify(tx), idx);

      // remove vin
      tx['vin'].forEach(utxou => {
        if (txs.get(utxou['tx_hash']) !== undefined) {
          txs.get(utxou['tx_hash']).delete(String(utxou['index']));
        }
      });

      // add vout
      let utxos = new Map();
      tx['vout'].forEach((utxo, i) => {
        utxos.set(String(i), utxo);
      });
      txs.set(txhash, utxos);
    });
  });

  return txs;
}

function getMyUTXOs(macc, tx_store, who) { 
  return JSON.parse(macc.get_mine(tx_store, who));
}

/*
function getMyUTXOs(tx_store, who) {
  let mine = new Map();
  let balance = 0;

  tx_store.forEach((utxos, tx_hash, _) => {
    utxos.forEach((utxo, index, _) => {
      if (utxo['lock'] === who) {
        balance += parseInt(utxo['value']);

        if (mine.get(tx_hash) === undefined) {
          mine.set(tx_hash, new Map());
        }
        mine.get(tx_hash).set(index, utxo);
      }
    });
  });

  return [mine, balance];
}
*/

function send(macc, mine, bal, amount, receiver, solution, lock, fee) {
  return fetch(NODE_URL + 'new-tx', {
    method: 'POST',
    body: macc.get_send_body(mine, bal, amount, receiver, solution, lock, fee),
    mode: 'cors'
  });
}

/*
function send(mine, bal, amount, receiver, solution, lock, fee) {
  if (bal < amount) return "You don't have enough balance!";

  let sending = 0;
  let body = {
    vin: [],
    vout: []
  };

  // create vin
  while (sending < amount) {
    let [tx_hash, utxos] = mine.entries().next().value;
    let [index, utxo] = utxos.entries().next().value;

    sending += parseInt(utxo['value']);
    bal -= parseInt(utxo['value']);

    body.vin.push({
      'tx_hash': JSON.parse(tx_hash),
      'index': parseInt(index),
      'solution': solution
    });

    console.log(mine.get(tx_hash))

    mine.get(tx_hash).delete(index)
    if (mine.get(tx_hash).size === 0) {
      mine.delete(tx_hash);
    }
  }

  // create vout
  body.vout.push({
    'value': amount,
    'lock': receiver
  });

  // create change utxo
  if (amount + fee < sending) {
    body.vout.push({
      'value': sending - amount - fee,
      'lock': lock
    });
  }

  return fetch(NODE_URL + 'new-tx', {
    method: 'POST',
    body: JSON.stringify(body),
    mode: 'cors'
  });
}
*/

export default { getBlockchain, getTxStore, parseBlockchain, getMyUTXOs, send};