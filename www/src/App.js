import {
  BrowserRouter as Router,
  Switch,
  Route
} from "react-router-dom";

import Explorer from './Explorer';
import Faucet from './Faucet';
import Wallet from './Wallet';
import React from "react";
import Header from "./Header";
import BlockchainDisplay from './explorer/blockchain';
import BlockDisplay from './explorer/block';


function App() {
  return (
    <div>
      <Router>
        <Header />
        <Switch>
          <Route path="/explorer">
            <BlockchainDisplay />
          </Route>
          <Route path="/faucet">
            <Faucet />
          </Route>
          <Route path="/wallet">
            <Wallet />
          </Route>
          <Route path="/:blockHeight">
            <BlockDisplay />
          </Route>
          <Route path="/">
            <BlockchainDisplay />
          </Route>
        </Switch>
      </Router>
    </div>
  );
}

export default App;