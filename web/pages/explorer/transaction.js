import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import Settings from '../../components/settings';
import { get_tx, tx_hash, to_hex } from 'macc-bindings';
import Link from "next/link";

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

export default function TransactionExplorer() {
    const router = useRouter();
    const { hash } = router.query;

    const [blockchain, setBlockchain] = useState(null);
    const [isLoading, setLoading] = useState(false);
    
    useEffect(() => {
        if (blockchain) return;
        
        setLoading(true);
        fetch(`${Settings.apiUrl}/blockchain`)
        .then((res) => res.text())
        .then((b) => {
            setBlockchain(b);
            setLoading(false);
        })
        .catch((_) => setLoading(false));
    }, [blockchain])
    
    if (isLoading) return <Loading />
    if (!blockchain || !hash) return <Error />

    let transaction = get_tx(blockchain, hash);
    if (!transaction) return <Error />
    
    let data_json = JSON.parse(transaction);
    
    let tx_json = data_json[0];
    let transaction_hash = tx_hash(JSON.stringify(tx_json));

    return (
        <div className='content'>
            <div>
                <div className='data'>
                    <p> In Block: <Link href={`/explorer/block?height=${data_json[1]}`} passHref>
                        <a className='link'>#{data_json[1]}</a>
                    </Link></p>
                    <p>Hash: {transaction_hash}</p>
                    { (tx_json.vin.length === 0) ? 
                        (<p>Type: Coinbase transaction</p>) : (<>
                            <p>Type: Normal Transaction</p>
                            {tx_json.vin.map((vi, i) => (
                                <div key={i}>
                                    <br />
                                    <h1>Input {i}:</h1>
                                    <p>Transaction hash: {to_hex(vi[0])}</p>
                                    <p>Index: {vi[1]}</p>
                                    <p>Solution: {vi[2]}</p>
                                </div>
                            ))}
                        </>)
                    }
                    <br />
                    {tx_json.vout.map((vo, i) => (
                        <div key={i}>
                            <br />
                            <h1>Output {i}:</h1>
                            <br />
                            <p>Amount: {vo[0] / 1000}</p>
                            <p>Lock: {vo[1]}</p>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    )
}