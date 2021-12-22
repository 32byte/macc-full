import React, { useState } from 'react';
import {
  Switch,
  Route,
  Link,
  useRouteMatch,
  useParams
} from "react-router-dom";

import Header from './Header';
import getBlockchain from "./API";

function DisplayBlock(props) {
  let path = '/explorer/' + props.block['height'];
  return (
    <div>
      <Link to={path}>Block Nr: {props.block['height']} Transactions: {props.block['transactions'].length}</Link>
    </div>
  )
}

function BlockchainExplorer() {
  let [blockchain, setBlockchain] = useState(null);
  
  if (blockchain == null) {
    getBlockchain().then(res => res.json()).then(bc => setBlockchain(bc));

    return <h1>Please wait for the blockchain to load..</h1>
  }
  
  let blocks = blockchain.reverse().map((block, index) =>
    <DisplayBlock block={block} key={index}/>
  );

  return (
    <div>
      <h1>Blockchain explorer</h1>
      {blocks}
    </div>
  );
}

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

function DisplayTransaction(props) {
  let vins = props.tx['vin'].map((vin, index) => 
    <DisplayVIN vin={vin} key={index}/>
  );

  let vouts = props.tx['vout'].map((vout, index) => 
    <DisplayVOUT vout={vout} key={index}/>
  );

  return (
    <div>
      <p>Vin: {props.tx['vin'].length}</p>
      {vins}
      <p>Vout: {props.tx['vout'].length}</p>
      {vouts}
    </div>
  )
}

function Block() {
  let { blockHeight } = useParams();

  let [blockchain, setBlockchain] = useState(null);
  
  if (blockchain == null) {
    getBlockchain(blockHeight, parseInt(blockHeight) + 1).then(res => res.json()).then(bc => setBlockchain(bc));

    return <h1>Please wait for the blockchain to load..</h1>
  }

  let block = blockchain[0];

  
  let transactions = block['transactions'].map((tx, index) =>
    <DisplayTransaction tx={tx} key={index}/>
  );


  return (
    <div>
      <h3>Block: {block['height']} Nonce: {block['nonce']}</h3>
      {transactions}
    </div>
  );
}

function Explorer() {
  let match = useRouteMatch();

  return (
    <div>
      <Header/>
      <Switch>
        <Route path={`${match.path}/:blockHeight`}>
          <Block />
        </Route>
        <Route path={match.path}>
          <BlockchainExplorer />
        </Route>
      </Switch>
    </div>
  );
}

export default Explorer;
