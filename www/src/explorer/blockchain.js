import React, { useState } from 'react';
import { Link } from "react-router-dom";

import api from "../API";

function DisplayBlock(props) {
  let path = '/explorer/' + props.block['height'];
  return (
    <div>
      <Link to={path}>Block Nr: {props.block['height']} Transactions: {props.block['transactions'].length}</Link>
    </div>
  )
}

function BlockchainDisplay() {
  let [blockchain, setBlockchain] = useState(null);
  
  if (blockchain == null) {
    api.getBlockchain().then(res => res.json()).then(bc => setBlockchain(bc));

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

export default BlockchainDisplay;