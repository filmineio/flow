use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ExitCode {
    Ok,
    // Indicates that the actor identified as the sender of a message is not valid as a message sender:
    // - not present in the state tree
    // - not an account actor (for top-level messages)
    // - code CID is not found or invalid
    // (not found in the state tree, not an account, has no code).
    SysErrSenderInvalid,
    // Indicates that the sender of a message is not in a state to send the message:
    // - invocation out of sequence (mismatched CallSeqNum)
    // - insufficient funds to cover execution
    SysErrSenderStateInvalid,

    // Indicates failure to find a method in an actor.
    SysErrInvalidMethod,

    // Indicates the message receiver trapped (panicked
    SysErrIllegalInstruction,

    // Indicates that the receiver of a message is not valid (and cannot be implicitly created).
    SysErrInvalidReceiver,

    // Indicates that a message sender has insufficient balance for the value being sent.
    // Note that this is distinct from SysErrSenderStateInvalid when a top-level sender can't cover
    // value transfer + gas. This code is only expected to come from inter-actor sends.
    SysErrInsufficientFunds,

    // Indicates message execution (including subcalls) used more gas than the specified limit.
    SysErrOutOfGas,

    // Indicates message execution is forbidden for the caller by runtime caller validation.
    SysErrForbidden,

    // Indicates actor code performed a disallowed operation. Disallowed operations include:
    // - mutating state outside of a state acquisition block
    // - failing to invoke caller validation
    // - aborting with a reserved exit code (including success or a system error).
    SysErrorIllegalActor,

    // Indicates an invalid argument passed to a runtime method.
    SysErrorIllegalArgument,

    // Indicates the actor returned a block handle that doesn't exist
    SysErrMissingReturn,

    // Unused
    SysErrReserved3,
    SysErrReserved4,
    SysErrReserved5,
    SysErrReserved6,
}

impl fmt::Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
