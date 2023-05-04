use rand::Rng;

const BOMB_FREQUENCY: f64 = 0.3;

pub(crate) struct Tile {
    is_bomb: bool,
    neighbour_bombs: u8,
    state: TileState,
}

pub(crate) enum TileState {
    INACTIVE,
    ACTIVE,
    SOLVED,
}

impl Tile {
    fn click_tile(&mut self) {
        if self.neighbour_bombs == 0 {
            self.state = TileState::SOLVED;
        } else {
            self.state = TileState::ACTIVE;
        }
    }
}

fn count_adjacent_bombs(game_map: Vec<Vec<u8>>, row: usize, column: usize) -> u8 {
    let mut count: u8 = 0;

    for i in (row.saturating_sub(1))..=(row + 1).min(game_map.len() - 1) {
        for j in (column.saturating_sub(1))..=(column + 1).min(game_map[i].len() - 1) {
            if i == row && j == column {
                continue;
            }
            if game_map[i as usize][j as usize] == 1 {
                count += 1;
            }
        }
    }

    count as u8
}

fn convert_map_to_tiles(game_map: Vec<Vec<u8>>) -> Vec<Vec<Tile>> {
    let mut result: Vec<Vec<Tile>> = Vec::new();

    for i in 0..game_map.len() {
        let mut new_array: Vec<Tile> = Vec::new();

        for j in 0..game_map[i].len() {
            new_array.push(Tile {
                is_bomb: game_map[i][j] == 1,
                neighbour_bombs: count_adjacent_bombs(game_map.clone(), i, j),
                state: TileState::INACTIVE,
            });
        }

        result.push(new_array);
    }

    result
}

pub(crate) fn create_game_instance_map(columns: u8, rows: u8) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut result: Vec<Vec<u8>> = Vec::new();

    for _ in 0..rows {
        let mut array: Vec<u8> = Vec::new();

        for _ in 0..columns {
            let random_float: f64 = rng.gen();

            if BOMB_FREQUENCY < random_float {
                array.push(0);
            } else {
                array.push(1);
            }
        }

        result.push(array);
    }

    result
}
