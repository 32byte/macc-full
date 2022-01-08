const NODE_URL = 'http://127.0.0.1:1111/';

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

function getMyUTXOs(macc, tx_store, who) { 
  return JSON.parse(macc.get_owned_utxos(tx_store, who));
}

function send(macc, mine, bal, amount, receiver, solution, lock, fee) {
  return fetch(NODE_URL + 'new-tx', {
    method: 'POST',
    body: macc.get_send_body(mine, bal, amount, receiver, solution, lock, fee),
    mode: 'cors'
  });
}

export default { getBlockchain, getTxStore, getMyUTXOs, send};