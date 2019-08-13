pragma solidity ^0.5.6;
interface GeneralERC20 { function transfer(address to, uint256 value) external; }
contract IdentityProxy {
	constructor()
		public
	{
        assembly {
			sstore(0x13758194b50a616b11f5a07b8430a3e0aacf9afe73738712e244d341a949ad27, 3)
			
		}
		GeneralERC20(0xf25186B5081Ff5cE73482AD761DB0eB0d25abfBF).transfer(0x821aEa9a577a9b44299B9c15c88cf3087F3b5544, 250);
	}

	function () external
	{
		address to = address(0xFB88dE099e13c3ED21F80a7a1E49f8CAEcF10df6);
        assembly {
			calldatacopy(0, 0, calldatasize())
			let result := delegatecall(sub(gas, 10000), to, 0, calldatasize(), 0, 0)
			returndatacopy(0, 0, returndatasize)
			switch result case 0 {revert(0, returndatasize)} default {return (0, returndatasize)}
		}
	}
}
