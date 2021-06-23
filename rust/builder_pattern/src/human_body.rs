pub struct Leg;
pub struct Head;
pub struct Arm;

#[derive(Default)]
pub struct HumanBody {
    legs: Vec<Leg>,
    arms: Vec<Arm>,
    heads: Vec<Head>,
}

impl HumanBody {
    pub fn analyze(&self) {
        println!(
            "This body has {} arms, {} heads and {} legs",
            self.arms.len(),
            self.heads.len(),
            self.legs.len()
        )
    }
    pub fn add_leg(&mut self, leg: Leg) {
        self.legs.push(leg)
    }
    pub fn add_head(&mut self, head: Head) {
        self.heads.push(head)
    }
    pub fn add_arm(&mut self, arm: Arm) {
        self.arms.push(arm)
    }
}
