// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * Minimal contract that emits an event. Used for Event Proofs PoC on Abstract.
 * Analog of StarkNet event_emitter: emit_value(value) -> ValueEmitted(from, value).
 */
interface IL1Messenger {
    function sendToL1(bytes calldata _message) external returns (bytes32);
}

contract EventEmitter {
    event ValueEmitted(address indexed from, uint256 value);
    IL1Messenger internal constant L1_MESSENGER =
        IL1Messenger(address(0x0000000000000000000000000000000000008008));

    function emitValue(uint256 value) external {
        emit ValueEmitted(msg.sender, value);
    }

    function emitValueWithL1Log(uint256 value) external {
        emit ValueEmitted(msg.sender, value);
        L1_MESSENGER.sendToL1(abi.encode(msg.sender, value));
    }
}
