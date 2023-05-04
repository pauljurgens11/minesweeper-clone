mod map;

fn main() {
    let value: Vec<Vec<u8>> = map::create_game_instance_map(5, 10);
    for array in value {
        println!("{:?}", array)
    }
}
