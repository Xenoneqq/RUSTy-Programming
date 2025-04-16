mod models;

fn main(){
    let name = "The Emperor";
    let player = models::player::make_player(name.to_string(), 23, 156);
    models::player::print_player_data(&player);
}