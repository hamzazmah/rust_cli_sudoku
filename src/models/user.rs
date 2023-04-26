#[derive(Deserialize, Debug)]
pub struct User {
    name: String,
    stats: Stats,
}

#[derive(Deserialize, Debug)]
pub struct Stats {
    wins: u32,
    losses: u32
}

pub impl User {
    pub fn new(name: &str) -> User {
        User {
            name: String::from(name),
            stats: Stats {
                wins: 0,
                losses: 0
            }
        }
    }

    pub fn win(&mut self) {
        self.stats.wins += 1;
    }

    pub fn lose(&mut self) {
        self.stats.losses += 1;
    }

    pub fn get_stats(&self) -> &Stats {
        &self.stats
    }
}