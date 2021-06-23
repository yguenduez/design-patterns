use crate::human_body::{Arm, Head, HumanBody, Leg};

pub struct Director;
impl Director {
    pub fn create_human(builder: &mut dyn Builder) {
        builder.build_corpus();
        builder.build_head();
        builder.build_arm();
        builder.build_arm();
        builder.build_leg();
        builder.build_leg();
    }
}

pub trait Builder {
    fn build_head(&mut self);
    fn build_arm(&mut self);
    fn build_corpus(&mut self);
    fn build_leg(&mut self);

    fn get_body(&self) -> &HumanBody;
}

#[derive(Default)]
pub struct BodyBuilder {
    body: HumanBody,
}

impl Builder for BodyBuilder {
    fn build_corpus(&mut self) {
        self.body = HumanBody::default();
    }

    fn build_arm(&mut self) {
        self.body.add_arm(Arm);
    }

    fn build_head(&mut self) {
        self.body.add_head(Head);
    }

    fn build_leg(&mut self) {
        self.body.add_leg(Leg)
    }

    fn get_body(&self) -> &HumanBody {
        &self.body
    }
}
