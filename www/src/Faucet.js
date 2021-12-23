import React, { useState } from 'react';

import api from './API';

function Receive(props) {
  // load macc-lib
  let [macc, setMACC] = useState(null);
  if (macc === null) {
    import("../build/macc_wasm").then(m => {
      setMACC(m);
    })
    return <p>Updating blockchain..</p>;
  }

  // send callback
  const send = () => {
    const amount = 1000; // 1.000 adjusted for precision
    const address = document.getElementById("receiver-addr").value;

    // send new-tx request
    let req = api.send(
      macc,                                                               // lib-reference
      JSON.stringify(props.mine),                                         // utxo's owned by the faucet account
      props.balance,                                                      // balance of the faucet account
      1000,                                                               // amount to send (1.000cc)
      address,                                                            // whom to send
      '217ddd77db89aa2787ab7f67df9ad885bf2e4925e3530e580fcc305308b2d54d', // private key of faucet
      '1C38wZFmk6XphdF4kg9dtpAxLjzroC1a75',                               // faucet address
      0                                                                   // fees
    );

    // user feedback for the request
    req.then(res => {
      if (res.status === 200) alert((amount/1000) + "cc sent!");
      else alert("Something went wrong!");
    });
  };

  return (
    <div>
      <input type="text" id="receiver-addr" placeholder="Your address:" />
      <button onClick={send}>Receive!</button>
    </div>
  )
}

function Faucet() {
  // load info about the faucet
  let [info, setInfo] = useState(null);
  if (info === null) {
    import("../build/macc_wasm").then(macc => {
      api.getTxStore().then(res => res.json()).then(txs => {
        setInfo(api.getMyUTXOs(macc, JSON.stringify(txs), "1C38wZFmk6XphdF4kg9dtpAxLjzroC1a75"));
      });
    })
    return <p className="info">Updating blockchain..</p>;
  }

  return (
    <div className="content">
      <h1>Faucet has currently {info.bal/1000}cc!</h1>
      <Receive balance={info.bal} mine={info.mine}/>
    </div>
  )
}

export default Faucet;