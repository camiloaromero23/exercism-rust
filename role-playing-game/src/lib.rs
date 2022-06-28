use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }

        let revive_mana = match self.level >= 10 {
            true => Some(100),
            _ => None,
        };
        Some(Player {
            health: 100,
            level: self.level,
            mana: revive_mana,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana < mana_cost {
                    0
                } else {
                    self.mana = Some(mana - mana_cost);
                    mana_cost * 2
                }
            }
            None => {
                self.health -= min(self.health, mana_cost);
                0
            }
        }
    }
}
