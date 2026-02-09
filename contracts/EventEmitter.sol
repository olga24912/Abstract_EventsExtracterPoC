// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * Minimal contract that emits an event. Used for Event Proofs PoC on Abstract.
 * Analog of StarkNet event_emitter: emit_value(value) -> ValueEmitted(from, value).
 */
contract EventEmitter {
    event ValueEmitted(address indexed from, uint256 value);

    function emitValue(uint256 value) external {
        emit ValueEmitted(msg.sender, value);
    }
}
