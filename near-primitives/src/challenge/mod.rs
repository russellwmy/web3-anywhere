mod challenge;
mod challenge_body;

pub use challenge::Challenge;
pub use challenge_body::ChallengeBody;

use crate::network::SlashedValidator;

/// Result of checking challenge, contains which accounts to slash.
/// If challenge is invalid this is sender, otherwise author of chunk (and possibly other participants that signed invalid blocks).
pub type ChallengesResult = Vec<SlashedValidator>;
pub type Challenges = Vec<Challenge>;
