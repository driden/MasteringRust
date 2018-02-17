
struct Character {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    wisdom: u8,
    intelligence: u8,
    charisma: u8,
    name: String
}

impl Character {
    fn new_named(name: String) -> Character {
        Character {
            strength: 9,
            constitution: 9,
            dexterity: 9,
            wisdom: 9,
            intelligence: 9,
            charisma: 9,
            name: name
        }
    }

    fn get_strength(&self) -> u8 {
        self.strength
    }
}

#[derive(Debug)]
enum Direction {
    N,
    NE,
    E,
    SE,
    SW,
    W,
    NW
}

enum PlayerAction {
        Move {direction: Direction, speed: u8},
        Wait,
        Attack {direction: Direction}
}

fn main() {
    let simulated_player_action = PlayerAction::Move {
        direction: Direction::NE,
        speed: 2
    };

    match simulated_player_action {
        PlayerAction::Wait => println!("Player wants to wait"),
        PlayerAction::Move { direction, speed} => {
            println!("Player wants to move in direction {:?} with speed {}",direction,speed)
        },
        PlayerAction::Attack{direction} => {
            print!("Player wants to attack direction {:?}", direction)
        }
    };

    let character:Character = Character::new_named("Dave".to_string());
    let _strength:u8 = character.get_strength();

}
