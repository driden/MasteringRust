
struct Character {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    wisdom: u8,
    intelligence: u8,
    charisma: u8,
    name: String
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
}
