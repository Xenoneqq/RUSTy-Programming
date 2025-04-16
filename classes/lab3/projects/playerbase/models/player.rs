pub struct Player {
    pub name: String,
    pub level: u32,
    pub health: u32,
}

pub fn make_player(name: String, level: u32, health: u32) -> Player {
    Player {
        name,
        level,
        health,
    }
}

pub fn print_player_data(player: &Player) {
    println!("Stats of player {}", player.name);
    println!("Level: {}", player.level);
    println!("Health: {}", player.health);
}