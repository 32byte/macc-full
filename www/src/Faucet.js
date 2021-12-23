import React, { useState } from 'react';

import api from './API';

function Receive(props) {
  let [macc, setMACC] = useState(null);
  
  if (macc === null) {
    import("../build/macc_wasm").then(m => {
      setMACC(m);
    })
    return <p>Updating blockchain..</p>;
  }

  let send = () => {
    let address = document.getElementById("receiver-addr").value;

    let req = api.send(macc, JSON.stringify(props.mine), props.balance, 1000, address, '217ddd77db89aa2787ab7f67df9ad885bf2e4925e3530e580fcc305308b2d54d', '1C38wZFmk6XphdF4kg9dtpAxLjzroC1a75', 0);

    req.then(_ => alert("1cc sent!"));
  };

  return (
    <div>
      <input type="text" id="receiver-addr" placeholder="Your address:" />
      <button onClick={send}>Receive!</button>
    </div>
  )
}

function Faucet() {
  let [stuff, setStuff] = useState(null);
  
  if (stuff === null) {
    import("../build/macc_wasm").then(macc => {
      api.getTxStore().then(res => res.json()).then(txs => {
        setStuff(api.getMyUTXOs(macc, JSON.stringify(txs), "1C38wZFmk6XphdF4kg9dtpAxLjzroC1a75"));
      });
    })
    return <p className="info">Updating blockchain..</p>;
  }

  return (
    <div className="content">
      <h1>Faucet has currently {stuff.bal/1000}cc!</h1>
      <Receive balance={stuff.bal} mine={stuff.mine}/>
    </div>
  )
}

export default Faucet;