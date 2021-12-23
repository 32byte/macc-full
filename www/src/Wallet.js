import React, { useState } from 'react';
import { useCookies } from 'react-cookie';

import randomBytes from './helper';
import api from './API';

function WalletData(props) {
  let [stuff, setStuff] = useState(null);
  
  if (stuff === null) {
    import("../build/macc_wasm").then(macc => {
      api.getTxStore().then(res => res.json()).then(txs => {
        setStuff({data: api.getMyUTXOs(macc, JSON.stringify(txs), props.address), m: macc });
      });
    })
    return <p className="info">Updating blockchain..</p>;
  }

  const send = () => {
    let amount = parseInt(parseFloat(document.getElementById("amount").value) * 1000);
    let receiver = document.getElementById("receiver").value;

    let req = api.send(stuff.m, JSON.stringify(stuff.data.mine), stuff.data.bal, amount, receiver, props.wallet, props.address, 0);

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

function WalletWrapper(props) {
  let [cookies, setCookie, removeCookie] = useCookies(['wallet', 'address']);

  // handle setting cookies
  const setWallet = () => {
    let wallet = document.getElementById("wallet-name").value;

    if (wallet === "") {
      alert("Generated new wallet!");
      wallet = randomBytes(32).toString('hex');
    }

    let pub_key = props.macc.get_public_key(wallet);
    if (pub_key === undefined) { alert('Something went wrong!'); return; }
    let address = props.macc.get_address(pub_key);
    if (address === undefined) { alert('Something went wrong!'); return; }

    setCookie('wallet', wallet, { path: '/' });
    setCookie('address', address, { path: '/' });
  }

  if (cookies.wallet === undefined) {
    return (<div>
      <input type="text" id="wallet-name" placeholder="Wallet name:"/>
      <button onClick={setWallet}>Create new wallet</button>
    </div>)
  }

  // handle deleteCookies
  const deleteWallet = () => {
    removeCookie("wallet");
    removeCookie("address");
  }

  return (
    <div className="content">
      <h1>Wallet</h1>
      <p id="address">Your address: {cookies.address}</p>
      <WalletData wallet={cookies.wallet} address={cookies.address}/>
      <button className="fancyBtn" onClick={deleteWallet}>Switch account</button>
    </div>
  )
}

function Wallet() {
  // load macc-lib
  let [macc, setMACC] = useState(null);
  if (macc === null) {
    import("../build/macc_wasm").then(m => {
      setMACC(m);
    })
    return <p>Updating blockchain..</p>;
  }

  return (
    <WalletWrapper macc={macc}/>
  )
}

export default Wallet;