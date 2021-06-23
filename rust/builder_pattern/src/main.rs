mod cppean_way;
mod human_body;
mod rustean_way;

use cpp::Builder;
use cppean_way as cpp;
use rustean_way as rustean;

fn main() {
    // The C++ style
    let mut builder = cpp::BodyBuilder::default();
    cpp::Director::create_human(&mut builder);
    let body = builder.get_body();
    body.analyze();

    // The Rustean Self consuming style
    let human_body = rustean::BodyBuilder::new()
        .add_arm()
        .add_arm()
        .add_head()
        .add_leg()
        .add_leg()
        .get_body();

    human_body.analyze();
}

// This body has 2 arms, 1 heads and 2 legs
// This body has 2 arms, 1 heads and 2 legs
