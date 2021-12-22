import React, { useState } from 'react';
import { Link } from "react-router-dom";

import api from "../API";

function DisplayBlock(props) {
  let path = '/' + props.block['height'];
  return (
    <div>
      <Link className="block" to={path}>Block Nr: {props.block['height']} Transactions: {props.block['transactions'].length}</Link>
    </div>
  )
}

function BlockchainDisplay() {
  let [blockchain, setBlockchain] = useState(null);
  
  if (blockchain == null) {
    api.getBlockchain().then(res => res.json()).then(bc => setBlockchain(bc));

    return <p className="info">Updating blockchain..</p>
  }
  
  let blocks = blockchain.reverse().map((block, index) =>
    <DisplayBlock block={block} key={index}/>
  );

  return (
    <div className="content">
      <h1>Blockchain explorer</h1>
      {blocks}
    </div>
  );
}

export default BlockchainDisplay;