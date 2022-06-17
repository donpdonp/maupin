// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Snipe {
  constructor() {
  }

  function liquidate(
       address _borrower,
       address _repayCToken,
       address _seizeCToken,
       uint _repay,
       uint _seize
  ) private returns (uint) {
    return _repay + _seize;
  }      
}
