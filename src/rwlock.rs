use rand::Rng;
use std::sync::{Arc, RwLock};

struct LotterySystem {
    participants: Arc<RwLock<Vec<String>>>,
}

impl LotterySystem {
    fn new() -> Self {
        Self {
            participants: Arc::new(RwLock::new(Vec::new())),
        }
    }

    fn add_participant(&self, name: String) {
        let mut participants = self.participants.write().unwrap();
        participants.push(name);
    }

    fn draw_winner(&self) -> Option<String> {
        let mut participants = self.participants.write().unwrap();

        let winner: Option<String> = if participants.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            let idx = rng.gen_range(0..participants.len());
            Some(participants.remove(idx))
        };

        winner
    }

    fn get_participants(&self) -> Vec<String> {
        let participants = self.participants.read().unwrap();
        participants.clone()
    }
}

#[allow(dead_code)]
pub fn main() {
    let lottery_system = LotterySystem::new();

    lottery_system.add_participant("Alice".to_string());
    lottery_system.add_participant("Bob".to_string());
    lottery_system.add_participant("Charlie".to_string());
    lottery_system.add_participant("David".to_string());

    let participants = lottery_system.get_participants();
    println!("Participants: {:?}", participants);

    let winner = lottery_system.draw_winner();
    println!("Winner: {:?}", winner);

    let participants = lottery_system.get_participants();
    println!("Participants: {:?}", participants);
}
