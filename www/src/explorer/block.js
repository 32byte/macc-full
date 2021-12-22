import React, { useState } from 'react';
import { useParams } from "react-router-dom";

import getBlockchain from "../API";

function DisplayVIN(props) {
  return (
    <div>
      <p>Index: {props.vin['index']}</p>
      <p>Solution: {props.vin['solution']}</p>
    </div>
  )
}

function DisplayVOUT(props) {
  return (
    <div>
      <p>Value: {props.vout['value']/1000}</p>
      <p>Lock: {props.vout['lock']}</p>
    </div>
  )
}

function Transaction(props) {
  let vouts = props.tx['vout'].map((vout, index) => 
    <DisplayVOUT vout={vout} key={index}/>
  );

  if (props.tx['vin'].length > 0) {
    let vins = props.tx['vin'].map((vin, index) => 
      <DisplayVIN vin={vin} key={index}/>
    );

    return (
      <div>
        <p>Normal Transaction: </p>
        <p>Vin: {props.tx['vin'].length}</p>
        {vins}
        <p>Vout: {props.tx['vout'].length}</p>
        {vouts}
      </div>
    )
  }

  return (
    <div>
      <p>Coinbase transaction:</p>
      <p>Vout: {props.tx['vout'].length}</p>
      {vouts}
    </div>
  )
}

function BlockDisplay() {
  let { blockHeight } = useParams();

  let [blockchain, setBlockchain] = useState(null);
  
  if (blockchain == null) {
    getBlockchain(blockHeight, parseInt(blockHeight) + 1).then(res => res.json()).then(bc => setBlockchain(bc));

    return <h1>Please wait for the blockchain to load..</h1>
  }

  let block = blockchain[0];

  
  let transactions = block['transactions'].map((tx, index) =>
    <Transaction tx={tx} key={index}/>
  );


  return (
    <div>
      <h3>Block: {block['height']} Nonce: {block['nonce']}</h3>
      {transactions}
    </div>
  );
}

export default BlockDisplay;