pragma solidity ^0.5.6;
interface GeneralERC20 { function transfer(address to, uint256 value) external; }
contract IdentityProxy {
	constructor()
		public
	{
		GeneralERC20(0xf25186B5081Ff5cE73482AD761DB0eB0d25abfBF).transfer(0x821aEa9a577a9b44299B9c15c88cf3087F3b5544, 250);
	}

	function () external
	{
		address to = address(0xFB88dE099e13c3ED21F80a7a1E49f8CAEcF10df6);
	}
}
