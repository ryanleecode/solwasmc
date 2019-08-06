
pragma solidity ^0.5.6;

contract IdentityProxy {
	constructor()
		public
	{
		assembly {
			sstore(0x13758194b50a616b11f5a07b8430a3e0aacf9afe73738712e244d341a949ad27, 3)
			
		}
		
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