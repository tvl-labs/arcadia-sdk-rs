use alloy::sol;

sol! {
    #[sol(rpc)]
    contract AssetReserves {
        function deposit(address token, uint256 amount, uint32 destChain) external payable;
    }
}

sol! {
    #[sol(rpc)]
    contract ERC20 {
        function allowance(address owner, address spender) external view returns (uint256);
    }
}
