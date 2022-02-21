import Image from 'next/image';
import Link from 'next/link';
import logo from '../public/logo.png';

export default function Header() {
    return (
        <div className='header'>
            <div className='logo'>
                <Link  href='/' passHref>
                    <div>
                        <Image src={logo} width={170} heigh={68} alt='' className='logo'/>
                    </div>
                </Link>
            </div>
            <div className='link'>
                <Link href='/explorer' passHref>
                    <a className='route-link-text'>Explorer</a>
                </Link>
            </div>
            <div className='link'>
                <Link href='/wallet' passHref>
                    <a className='route-link-text'>Wallet</a>
                </Link>
            </div>
            {/*
            <div className='align-right'>
                <Link href='/about' passHref>
                    <a className='route-link-text'>About</a>
                </Link>
            </div>
            */}
        </div>
    )
}