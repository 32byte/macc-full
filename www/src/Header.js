import React from "react";
import { Link } from "react-router-dom";

function Header() {
	return (
		<div id="heading">
			<Link className="link" to="/explorer">Explorer</Link>
			<Link className="link" to="/faucet">Faucet</Link>
			<Link className="link" to="/wallet">Wallet</Link>
		</div>
	);
}

export default Header;