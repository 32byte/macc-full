import React, { useState } from 'react';

import Header from "./Header";

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

    let req = api.send(macc, JSON.stringify(props.mine), props.balance, 1000, address, 'miner1', 'miner1', 0);

    req.then(res => console.log(res));
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
      // api.getBlockchain().then(res => res.json()).then(blockchain => {
      //   let txs = api.parseBlockchain(blockchain, macc.hash_tx);
      //   setTXS(txs);
      // });
      api.getTxStore().then(res => res.json()).then(txs => {
        setStuff(api.getMyUTXOs(macc, JSON.stringify(txs), "miner1"));
      });
    })
    return <p>Updating blockchain..</p>;
  }

  return (
    <div>
      <Header />
      <h1>Faucet has currently {stuff.bal/1000}cc!</h1>
      <Receive balance={stuff.bal} mine={stuff.mine}/>
    </div>
  )
}

export default Faucet;