const NODE_URL = 'http://localhost:1111/';

function getBlockchain(start, stop) {
  let url = NODE_URL + 'blockchain?from=' + start + '&until=' + stop;

  console.log(url);

  return fetch(url, {
    method: 'GET',
    mode: 'cors'
  });
}

export default getBlockchain;