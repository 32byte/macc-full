import React from 'react';
import {
  Switch,
  Route,
  useRouteMatch,
} from "react-router-dom";

import BlockchainDisplay from './explorer/blockchain';
import BlockDisplay from './explorer/block';
import Header from './Header';

function Explorer() {
  let match = useRouteMatch();

  return (
    <div>
      <Header/>
      <Switch>
        <Route path={`${match.path}/:blockHeight`}>
          <BlockDisplay />
        </Route>
        <Route path={match.path}>
          <BlockchainDisplay />
        </Route>
      </Switch>
    </div>
  );
}

export default Explorer;
