// Deadline calculations with respect to a current epoch.
// "Deadline" refers to the window during which proofs may be submitted.
// Windows are non-overlapping ranges [Open, Close), but the challenge epoch for a window occurs before
// the window opens.
// The current epoch may not necessarily lie within the deadline or proving period represented here.
#[allow(non_snake_case)]
pub struct DeadlineInfo {
    pub CurrentEpoch: u64, // Epoch at which this info was calculated.
    pub PeriodStart: u64,  // First epoch of the proving period (<= CurrentEpoch).
    pub Index: u64, // A deadline index, in [0..d.WPoStProvingPeriodDeadlines) unless period elapsed.
    pub Open: u64,  // First epoch from which a proof may be submitted (>= CurrentEpoch).
    pub Close: u64, // First epoch from which a proof may no longer be submitted (>= Open).
    pub Challenge: u64, // Epoch at which to sample the chain for challenge (< Open).
    pub FaultCutoff: u64, // First epoch at which a fault declaration is rejected (< Open).
    // Protocol parameters
    pub WPoStPeriodDeadlines: u64,
    pub WPoStProvingPeriod: u64, // the number of epochs in a window post proving period
    pub WPoStChallengeWindow: u64,
    pub WPoStChallengeLookback: u64,
    pub FaultDeclarationCutoff: u64,
}
