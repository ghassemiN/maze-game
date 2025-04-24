use rand::seq::SliceRandom;
use rand::Rng;
use std::io::{self, Write};

const WIDTH: usize = 9;
const HEIGHT: usize = 9;

fn main() {
    let mut maze = vec![vec!['#'; WIDTH]; HEIGHT];
    let mut rng = rand::thread_rng();

    let mut player_pos = (1, 1);
    let mut current = player_pos;
    maze[current.0][current.1] = '.';

    let directions = [(0, 2), (0, -2), (2, 0), (-2, 0)];

    for _ in 0..(WIDTH * HEIGHT) {
        let mut dir = directions.choose(&mut rng).unwrap();
        let new_x = (current.0 as isize + dir.0) as usize;
        let new_y = (current.1 as isize + dir.1) as usize;

        if new_x > 0 && new_x < HEIGHT - 1 && new_y > 0 && new_y < WIDTH - 1 {
            if maze[new_x][new_y] == '#' {
                maze[(current.0 + new_x) / 2][(current.1 + new_y) / 2] = '.';
                maze[new_x][new_y] = '.';
                current = (new_x, new_y);
            }
        }
    }

    let exit_pos = current;
    maze[player_pos.0][player_pos.1] = 'P';
    maze[exit_pos.0][exit_pos.1] = 'E';

    loop {
        print_maze(&maze);

        if player_pos == exit_pos {
            println!(" You win, hooray! ");
            break;
        }

        let mut input = String::new();
        print!(" Move (WASD): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_uppercase();

        let (dx, dy) = match input.as_str() {
            "W" => (-1, 0),
            "S" => (1, 0),
            "A" => (0, -1),
            "D" => (0, 1),
            _ => {
                println!(" Invalid input. Use W, A, S, D ");
                continue;
            }
        };

        let new_x = (player_pos.0 as isize + dx) as usize;
        let new_y = (player_pos.1 as isize + dy) as usize;

        if maze[new_x][new_y] != '#' {
            maze[player_pos.0][player_pos.1] = '.';
            player_pos = (new_x, new_y);
            maze[player_pos.0][player_pos.1] = 'P';
        }
    }
}

fn print_maze(maze: &Vec<Vec<char>>) {
    print!("{}[2J", 27 as char); 
    for row in maze {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}
