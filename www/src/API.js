const NODE_URL = 'http://localhost:1111/';

function getBlockchain(start, stop) {
  let url = NODE_URL + 'blockchain?from=' + start + '&until=' + stop;

  return fetch(url, {
    method: 'GET',
    mode: 'cors'
  });
}

export default getBlockchain;