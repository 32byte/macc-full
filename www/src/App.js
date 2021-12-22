import {
  BrowserRouter as Router,
  Switch,
  Route
} from "react-router-dom";

import Explorer from './Explorer';
import Faucet from './Faucet';
import Wallet from './Wallet';
import React from "react";

function App() {
  return (
    <div>
      <Router>
        <Switch>
          <Route path="/explorer">
            <Explorer />
          </Route>
          <Route path="/faucet">
            <Faucet />
          </Route>
          <Route path="/wallet">
            <Wallet />
          </Route>
          <Route path="/">
            <Explorer />
          </Route>
        </Switch>
      </Router>
    </div>
  );
}

export default App;