import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import Settings from '../../components/settings';
import * as wasm from 'macc-bindings';
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

function DisplayTransaction({ tx, index }) {
    let hash = wasm.tx_hash(JSON.stringify(tx));

    return (
        <p>[{index}] Hash: { ' ' }
            <Link href={`/explorer/transaction?hash=${hash}`} passHref>
                <a>{ hash }</a>
            </Link>
        </p>
    )
}

function BlockDisplay({ block, height }) {
    let transactions = block.transactions.map((tx, index) => <DisplayTransaction tx={tx} index={index} key={index}/> )

    return (
        <div className='content'>
            <div>
                <div className='data'>
                    <h1>Block Explorer for Block #{height}</h1>
                    <p>Height: {height}</p>
                    <p>Timestamp: {block.timestamp}</p>
                    <p>Block reward: {wasm.calculate_mining_reward(height)}</p>
                    <p>Transactions:</p>
                    <div>
                        {transactions}
                    </div>
                </div>
            </div>
        </div>
    )
}

export default function BlockExplorer() {
    const router = useRouter();
    const { height } = router.query;

    const [block, setBlock] = useState(null);
    const [isLoading, setLoading] = useState(false);
    
    useEffect(() => {
        const h = parseInt(height, 10);
        if (!h) return;
        if (block) return;
        
        setLoading(true);
        fetch(`${Settings.apiUrl}/blockchain?start=${h}&stop=${h+1}`)
        .then((res) => res.json())
        .then((b) => {
            setBlock(b);
            setLoading(false);
        })
        .catch((_) => setLoading(false));
    }, [block, height])
    
    if (isLoading) return <Loading />
    if (!block) return <Error />

    return (
        <BlockDisplay block={block[0]} height={height}/>
    )
}