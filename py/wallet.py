import macc_lib
import requests
import json

# SERVER = 'http://164.68.103.253:8033'
SERVER = 'http://127.0.0.1:1111'
KEY = input('input your user: ')

def get_blockchain():
    return requests.get(SERVER + '/blockchain').json()

def hash_tx(tx, h):
    return macc_lib.tx_hash(tx, h)
    # return requests.post(SERVER + '/calc-tx-hash?height={}'.format(h), data=tx).text

def send(mine, bal, amount, recipient, fee):
    def str_to_list(s):
        return [int(x) for x in s.replace(',', '').replace('[', '').replace(']', '').split(' ')]

    if bal < amount:
        print('You don\'t have enough balance!')
        return

    # TODO: figure out a more efficient way of doing that
    sending = 0
    req = {
        'vin': [],
        'vout': [],
    }

    # create vin
    while sending < amount:
        txid = list(mine.keys())[0]
        utxos = mine[txid]
        index = list(utxos.keys())[0]
        utxo = utxos[index]

        sending += int(utxo['value'])
        bal -= int(utxo['value'])

        # add to request
        req['vin'].append({
            'tx_hash': str_to_list(txid),
            'index': int(index),
            'solution': KEY
        })

        # clean up
        del utxos[list(utxos.keys())[0]]
        if len(utxos.keys()) == 0:
            del mine[list(mine.keys())[0]]
    
    # create vout
    req['vout'].append({
        'value': amount,
        'lock': recipient
    })
    if amount + fee < sending:
        req['vout'].append({
            'value': sending - amount - fee,
            'lock': KEY
        })

    requests.post(SERVER + '/new-tx', data=json.dumps(req))

    print(f'You sent {amount/1000} to {recipient}! Please wait for network confirmation before doing any other transactions!')

def parse_bc(bc):
    txs = {}
    for h, block in enumerate(bc):
        for tx in block['transactions']:
            txid = hash_tx(json.dumps(tx), h)

            # remove vin
            for utxou in tx['vin']:
                del txs[str(utxou['tx_hash'])][str(utxou['index'])]

            # add vout
            utxos = {}
            for i, utxo in enumerate(tx['vout']):
                utxos[str(i)] = utxo
            txs[str(txid)] = utxos

    return txs

def get_my_utxos(txs):
    mine = {}
    balance = 0

    for k, v in txs.items():
        for i, utxo in v.items():
            if utxo['lock'] == KEY:
                balance += int(utxo['value'])
                
                if k not in mine:
                    mine[k] = {}
                mine[k][i] = utxo
                

    return balance, mine


if __name__ == '__main__':
    bc = get_blockchain()
    txs = parse_bc(bc)

    bal, mine = get_my_utxos(txs)
    print(mine)
    print(f'You currently have {bal/1000}cc in your wallet!')

    try:
        send(mine, bal, int(float(input('How much would you like to send? ')) * 1000), 'byte', 0)
    except KeyboardInterrupt:
        print()
    print('Bye!')
