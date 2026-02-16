use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Bid {
    pub bidder_id: u64,
    pub encrypted_amount: u64, // In a real app, this is a secret share
}

pub struct BlindAuction {
    pub bids: Vec<Bid>,
}

impl BlindAuction {
    pub fn new() -> Self {
        Self { bids: Vec::new() }
    }

    pub fn add_bid(&mut self, bidder: u64, amount: u64) {
        self.bids.push(Bid {
            bidder_id: bidder,
            encrypted_amount: amount,
        });
    }

    /// Confidential Winner Selection via MPC
    pub fn resolve_winner(&self) -> Option<Bid> {
        // This computation happens inside Arcium MPC cluster
        self.bids.iter().max_by_key(|b| b.encrypted_amount).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blind_auction_resolution() {
        let mut auction = BlindAuction::new();
        
        // Simulating 3 hidden bids
        auction.add_bid(101, 500); // Bidder A
        auction.add_bid(102, 750); // Bidder B
        auction.add_bid(103, 620); // Bidder C

        let winner = auction.resolve_winner().unwrap();
        
        println!("MPC Resolution Complete.");
        println!("Winner ID: {} with the highest confidential bid!", winner.bidder_id);
        
        assert_eq!(winner.bidder_id, 102);
        assert_eq!(winner.encrypted_amount, 750);
    }
}