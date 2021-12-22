import React from "react";
import { Link } from "react-router-dom";

function Header() {
	return (
		<div>
			<ul>
				<li>
					<Link to="/">Explorer</Link>
				</li>
				<li>
					<Link to="/faucet">Faucet</Link>
				</li>
				<li>
					<Link to="/wallet">Wallet</Link>
				</li>
			</ul>
		</div>
	);
}

export default Header;