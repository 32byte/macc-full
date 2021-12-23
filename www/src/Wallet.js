import React, { useState } from 'react';
import { useCookies } from 'react-cookie';

import api from "./API";

function randomBytes(length) {
  var result           = '';
  var characters       = '0123456789abcdef';
  var charactersLength = characters.length;
  for ( var i = 0; i < length; i++ ) {
      result += characters.charAt(Math.floor(Math.random() * charactersLength));
  }
  return result;
}

function random_secret_key() {
  return randomBytes(64).toString('hex');
}

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

function WalletWrap(props) {
  let [cookies, setCookie, removeCookie] = useCookies(['wallet', 'address']);

  const setWallet = () => {
    let wallet = document.getElementById("wallet-name").value;

    if (wallet === "") {
      alert("Generated new wallet!");
      wallet = random_secret_key();
    }

    let pub_key = props.macc.get_public_key(wallet);
    let address = props.macc.get_address(pub_key);

    setCookie('wallet', wallet, { path: '/' });
    setCookie('address', address, { path: '/' });
  }

  const deleteWallet = () => {
    removeCookie("wallet");
    removeCookie("address");
  }

  if (cookies.wallet === undefined) {
    return (<div>
      <input type="text" id="wallet-name" placeholder="Wallet name:"/>
      <button onClick={setWallet}>Create new wallet</button>
    </div>)
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
  let [macc, setMACC] = useState(null);
  
  if (macc === null) {
    import("../build/macc_wasm").then(m => {
      setMACC(m);
    })
    return <p>Updating blockchain..</p>;
  }

  return (
    <WalletWrap macc={macc}/>
  )
}

export default Wallet;