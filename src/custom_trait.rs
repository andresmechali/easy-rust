use std::fmt::Debug;

pub fn run() {
    let wizard = Wizard {};
    let ranger = Ranger {};

    let mut monster1 = Monster { health: 40 };

    wizard.attack_with_sword(&mut monster1);
    ranger.attack_with_sword(&mut monster1);
    ranger.attack_with_bow(&mut monster1, 8);
}

struct Monster {
    health: i32,
}
#[derive(Debug)]
struct Wizard {}
#[derive(Debug)]
struct Ranger {}

trait FightClose {
    fn attack_with_sword(&self, opponent: &mut Monster)
    where
        Self: Debug,
    {
        opponent.health -= 10;
        println!(
            "You attack with sword. Your opponent has {} life left.",
            opponent.health
        );
        println!("Current status: {:?}", self);
    }

    fn attack_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!(
            "You attack with sword. Your opponent has {} life left.",
            opponent.health
        );
    }
}

impl FightClose for Wizard {}

impl FightClose for Ranger {}

trait FightFromDistance {
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!(
                "You attack with bow. Your opponent has {} life left.",
                opponent.health
            );
        }
    }

    fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
            println!(
                "You attack with rock. Your opponent has {} life left.",
                opponent.health
            );
        }
    }
}

impl FightFromDistance for Ranger {}
