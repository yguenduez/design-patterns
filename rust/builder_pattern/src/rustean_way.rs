use crate::human_body::{Arm, Head, HumanBody, Leg};

pub struct BodyBuilder {
    body: HumanBody,
}

impl BodyBuilder {
    pub fn new() -> Self {
        BodyBuilder {
            body: HumanBody::default(),
        }
    }

    pub fn add_leg(mut self) -> Self {
        self.body.add_leg(Leg);
        self
    }

    pub fn add_arm(mut self) -> Self {
        self.body.add_arm(Arm);
        self
    }

    pub fn add_head(mut self) -> Self {
        self.body.add_head(Head);
        self
    }

    pub fn get_body(self) -> HumanBody {
        self.body
    }
}
