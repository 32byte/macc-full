import React, { useState } from 'react';
import { useCookies } from 'react-cookie';

import Header from "./Header";
import api from "./API";

function WalletData(props) {
  let [stuff, setStuff] = useState(null);
  
  if (stuff === null) {
    import("../build/macc_wasm").then(macc => {
      api.getTxStore().then(res => res.json()).then(txs => {
        setStuff({data: api.getMyUTXOs(macc, JSON.stringify(txs), props.wallet), m: macc });
      });
    })
    return <p>Updating blockchain..</p>;
  }

  const send = () => {
    let amount = parseInt(parseFloat(document.getElementById("amount").value) * 1000);
    let receiver = document.getElementById("receiver").value;

    let req = api.send(stuff.m, JSON.stringify(stuff.data.mine), stuff.data.bal, amount, receiver, "solution...", props.wallet, 0);

    if (typeof(req) === typeof("")) {
      alert(req);
      return;
    }

    req.then(_ => alert("Sent " + amount + " to " + receiver + "!"));
  }

  return (
    <div>
      <p>You currently have {stuff.data.bal/1000}cc!</p>
      <div>
        <input type="number" id="amount" placeholder="Amount:"/>
        <input type="text" id="receiver" placeholder="Receiver:"/>
        <button onClick={send}>Send!</button>
      </div>
    </div>
  )
}

function WalletBody() {
  let [cookies, setCookie, removeCookie] = useCookies(['wallet']);

  const setWallet = () => {
    let wallet = document.getElementById("wallet-name").value;

    setCookie('wallet', wallet, { path: '/' });
  }

  const deleteWallet = () => {
    removeCookie("wallet");
  }

  if (cookies.wallet === undefined) {
    return (<div>
      <input type="text" id="wallet-name" placeholder="Wallet name:"/>
      <button onClick={setWallet}>Create new wallet</button>
    </div>)
  }

  return (
    <div>
      <h1>Wallet here {cookies.wallet}!</h1>
      <WalletData wallet={cookies.wallet}/>
      <button onClick={deleteWallet}>Log out</button>
    </div>
  )
}

function Wallet() {
  return (
    <div>
      <Header/>
      <WalletBody />
    </div>
  )
}

export default Wallet;