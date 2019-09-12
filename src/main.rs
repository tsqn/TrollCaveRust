use rand::Rng;
use std::io::{stdin, stdout, Write};

#[derive(Debug, Copy, Clone)]
enum ArenaProgress {
    InProgress,
    TrollWin,
    KnightWin,
}

#[derive(Debug, Copy, Clone)]
struct Troll {
    health: i8,
    damage: i8
}

#[derive(Debug, Copy, Clone)]
struct Weapon {
    damage: i8,
    durability: i8
}

#[derive(Debug, Copy, Clone)]
struct Knight {
    health: i8,
    weapon: Weapon
}

#[derive(Debug, Copy, Clone)]
struct ArenaState {
    knight: Knight,
    troll: Troll,
    progress: ArenaProgress,
    completed: bool
}

fn fight(mut arena_state: ArenaState) -> ArenaState {

    if arena_state.knight.weapon.durability > 0
    {
        arena_state.troll.health = arena_state.troll.health - arena_state.knight.weapon.damage;
        arena_state.knight.weapon.durability = arena_state.knight.weapon.durability - 1;
    }
    
    if arena_state.troll.health <= 0
    {
        arena_state.progress = ArenaProgress::KnightWin;
        arena_state.completed = true;
        
        return arena_state;
    }
    else 
    {
        arena_state.knight.health = arena_state.knight.health - arena_state.troll.damage;

        if arena_state.knight.health > 0
        {
            return arena_state;
        }

        arena_state.progress = ArenaProgress::TrollWin;
        arena_state.completed = true;

        return arena_state;
    }
}

fn console_dialog(message: String) -> bool{
    
    println!("{}",message);
    println!("[Y/N]");

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Input");

    if input.trim() == "Y" || input.trim() == "y"{
        return true;
    }
    else if input.trim() == "N" || input.trim() == "n" {
        return false;
    }
    else {
        return false;
    }
}

fn get_new_weapon() -> Weapon{
    let mut rng = rand::thread_rng();
    let weapon = Weapon{damage: rng.gen_range(1, 10), durability: rng.gen_range(2, 10)};
    return weapon;
}

fn game_loop(){

    let mut battle_number = 0;

    let weapon = Weapon{damage: 1, durability: 2};

    let mut knight = Knight{health: 3, weapon: weapon};

    loop {

        battle_number = battle_number + 1;


        let troll = Troll{damage: 1, health: 2};

        let mut arena_state = ArenaState{knight: knight, troll: troll, completed: false, progress: ArenaProgress::InProgress};

        let mut fight_completed = false;
        let mut round_number = 0;

        println!("Battle {}", battle_number);
        println!("Troll HP: {}",arena_state.troll.health);
        println!("Knight HP: {}",arena_state.knight.health);
        while !fight_completed {
            round_number = round_number + 1;
            println!("Round {}", round_number);
            arena_state = fight(arena_state);
            fight_completed = arena_state.completed;
            println!("Troll HP after {} round:{}",round_number,arena_state.troll.health);
            println!("Knight HP after {} round:{}",round_number,arena_state.knight.health);
        }

        let who_wins;

        match arena_state.progress {
            ArenaProgress::KnightWin => who_wins = "Knight Wins!".to_string(),
            ArenaProgress::TrollWin => who_wins = "Troll Wins!".to_string(),
            ArenaProgress::InProgress => who_wins = "Someshit error".to_string()
        }

        println!("Battle {} completed after {} rounds, {}",battle_number, round_number, who_wins);

        if let ArenaProgress::TrollWin = arena_state.progress{
            return;
        }

        let new_weapon = get_new_weapon();

        println!("New weapon found - Damage: {}, Durability: {}", new_weapon.damage, new_weapon.durability);

        let x = console_dialog(format!("Change you current weapon - Damage: {}, Durability: {}?", arena_state.knight.weapon.damage, arena_state.knight.weapon.durability));

        println!("{}", x);

        if x {
            arena_state.knight.weapon = new_weapon;
        }

        knight = arena_state.knight;

        let _=stdout().flush();
    }
}

fn main() {
    let mut game_over = false;

    while !game_over {
        
        println!("Let the battle begin!");
        game_loop();

        game_over = !console_dialog("Try again?".to_string());

        println!("Game Over");
      
    }
}
