import Settings from "../../components/settings";
import { tx_hash, my_utxos, send } from 'macc-bindings';

export default async function handler(req, res) {
    const { addr } = req.query;

    if (!addr) {
        res.status(400).json({'error': 'pleace specify addr'});
        return;
    };

    let fres = await fetch(`${Settings.apiUrl}/txstore`);
    let store = await fres.text();

    let owned = my_utxos(store, Settings.faucetAddr);
    
    let tx = send(owned, Settings.faucetSk, addr, '1000');

    await fetch(`${Settings.apiUrl}/transaction`, {
        method: 'POST',
        headers: {},
        body: tx
    });

    res.status(200).json({'hash': tx_hash(tx)})
}