import { setCookies, getCookie } from 'cookies-next';
import { randomBytes } from 'crypto';
import { useState, useEffect } from 'react';
import { get_client, my_utxos, send, tx_hash } from 'macc-bindings';
import Settings from '../components/settings';

function Loading() {
    return (
        <>
            <p>Loading, please wait</p>
        </>
    )
}

function Error() {
    return (
        <>
            <p>Something went wrong.. please check your input!</p>
        </>
    )
}

function RenderWallet( { client, store } ) {
    let [sk, pb, addr] = JSON.parse(client);
    let owned = my_utxos(store, addr);
    let [bal, _] = JSON.parse(owned);

    let [showSk, setShowSk] = useState(false);

    const send_ui = () => {
        let addr = document.getElementById('address').value;
        let amount = parseInt((parseFloat(document.getElementById('amount').value) * 1000)).toString();

        let tx = send(owned, sk, addr, amount);
        if (!tx) {
            alert('Please check your input!')
            return;
        }

        fetch(`${Settings.apiUrl}/transaction`, {
            method: 'POST',
            headers: {},
            body: tx
        })
        .then((res) => res.text())
        .then((_) => {
            alert(`Transaction sent successfully! Hash: ${tx_hash(tx)}`)
        })
    }

    const receive = () => {
        fetch(`/api/faucet?addr=${addr}`)
        .then((res) => res.json())
        .then((r) => {
            alert(`Transaction sent successfully! Hash: ${r.hash}`)
        })
    }

    return (
        <>
        <p>Address: {addr}</p>
        <p onClick={() =>  {setShowSk(!showSk)}}>Secret Key: {showSk ? sk : (<i>click to show</i>)}</p>
        <p>Balance: {bal / 1000}cc</p>
        <br />

        <div>
            <p>Receive 1 cc from faucet:</p>
            <button onClick={receive}>Receive!</button>
        </div>

        <p>Send crypto:</p>
        <div className='flex'>
            <input id='address' placeholder="Address:"/>
            <input id='amount' placeholder="Amount:"/>
            <button onClick={send_ui}>Send</button>
        </div>
        </>
    )
}

export default function Wallet() {
    const [store, setStore] = useState(null);
    const [isLoading, setLoading] = useState(false);
    const [sk, setSk] = useState(getCookie('sk'));
    const [client, setClient] = useState(null);
    
    useEffect(() => {
        if (store) return;
        
        setLoading(true);
        fetch(`${Settings.apiUrl}/txstore`)
        .then((res) => res.text())
        .then((b) => {
            setStore(b);
            setLoading(false);
        })
        .catch((_) => setLoading(false));
    }, [store])
    
    if (isLoading) return <Loading />
    if (!store) return <Error />

    const loadWallet = () => {
        let sk = document.getElementById('seed-input').value;

        setCookies('sk', sk);
        setSk(sk);
    }

    const createWallet = () => {
        let sk = randomBytes(32).toString('hex');
        setCookies('sk', sk);
        setSk(sk);
    }

    if (sk && !client) setClient(get_client(sk));

    return (
        <div className="content">
            <div>
                <div className="wallet">
                    <div className='flex'>
                        <button onClick={createWallet}>New Wallet</button>
                        <h1>or</h1>
                        <input id='seed-input' placeholder="Input Secret Key:"/>
                        <button onClick={loadWallet}>Create Wallet</button>
                    </div>

                    <br />
                    
                    {client && <RenderWallet client={client} store={store}/>}
                </div>
            </div>
        </div>
    )
}