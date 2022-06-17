// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol";

contract Snipe {
    ISwapRouter public immutable swapRouter;

    address public constant DAI = 0x6B175474E89094C44Da98b954EedeAC495271d0F;
    address private constant CETH = 0x4Ddc2D193948926D02f9B1fE9e1daa0718270ED5;

    constructor(ISwapRouter _swapRouter) {
        swapRouter = _swapRouter;
    }

    function liquidate(
        address _borrower,
        address _repayCToken,
        address _seizeCToken,
        uint256 _repay,
        uint256 _seize
    ) private returns (uint256) {
        // For this example, we will set the pool fee to 0.3%.
        uint24 poolFee = 3000;

        ISwapRouter.ExactInputSingleParams memory params = ISwapRouter
            .ExactInputSingleParams({
                tokenIn: DAI,
                tokenOut: _repayCToken,
                fee: poolFee,
                recipient: msg.sender,
                deadline: block.timestamp,
                amountIn: _repay,
                amountOutMinimum: 0,
                sqrtPriceLimitX96: 0
            });

        swapRouter.exactInputSingle(params);

        return _repay + _seize;
    }
}
