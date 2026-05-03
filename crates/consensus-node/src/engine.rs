pub fn check_consensus_reached(votes: usize, threshold: usize) -> bool {
    votes >= threshold
}
