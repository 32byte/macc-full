import React from 'react';
import {
  Switch,
  Route,
  useRouteMatch,
} from "react-router-dom";

import BlockchainDisplay from './explorer/blockchain';
import BlockDisplay from './explorer/block';

function Explorer() {
  let match = useRouteMatch();

  return (
    <Switch>
      <Route path={`${match.path}/:blockHeight`}>
        <BlockDisplay />
      </Route>
      <Route path={match.path}>
        <BlockchainDisplay />
      </Route>
    </Switch>
  );
}

export default Explorer;
