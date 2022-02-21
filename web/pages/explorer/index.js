import { useRouter } from "next/router"
import { useState, useEffect } from "react";
import Settings from "../../components/settings";

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

export default function Explorer() {
    const router = useRouter();

    const [height, setHeight] = useState(null);
    const [isLoading, setLoading] = useState(false);
    
    useEffect(() => {
        if (height) return;
        
        setLoading(true);
        fetch(`${Settings.apiUrl}/height`)
        .then((res) => res.text())
        .then((b) => {
            setHeight(b);
            setLoading(false);
        })
        .catch((_) => setLoading(false));
    }, [height])

    if (isLoading) return <Loading />
    if (!height) return <Error />

    const search = () => {
        let hash = document.getElementById('search').value;
        
        if (!hash) return;

        router.push(`/explorer/transaction?hash=${hash}`)
    }

    const go = () => {
        let height = document.getElementById('height').value;
        
        if (!height) return;

        router.push(`/explorer/block?height=${height}`)
    }

    return (
        <div className='content'>
            <div>
                <div className='data'>
                    <h1>Current Block height: {height}</h1>
                    <div className='yes'>
                        <input id='search' placeholder='Enter a transaction hash:' spellCheck={false} autoComplete={'off'} />
                        <button onClick={search}>Search!</button>
                    </div>
                    <div className='yes'>
                        <input id='height' placeholder='Enter block height:' spellCheck={false} autoComplete={'off'} />
                        <button onClick={go}>Go!</button>
                    </div>
                </div>
            </div>
        </div>
    )
}