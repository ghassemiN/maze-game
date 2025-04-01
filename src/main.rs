use std::io::{self, Write};

fn main() {
    let mut maze = vec![
        vec!['#', '#', '#', '#', '#', '#', '#'],
        vec!['#', 'P', '.', '.', '#', 'E', '#'],
        vec!['#', '.', '#', '.', '#', '.', '#'],
        vec!['#', '.', '#', '.', '.', '.', '#'],
        vec!['#', '#', '#', '#', '#', '#', '#'],
    ];

    let mut player_pos = (1, 1);
    let exit_pos = (1, 5);

    loop {
        print_maze(&maze);

        if player_pos == exit_pos {
            println!(" You win, hooray! ");
            break;
        }

        let mut input = String::new();
        print!(" Start (WASD): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_uppercase();

        let (new_x, new_y) = match input.as_str() {
            "W" => (player_pos.0 - 1, player_pos.1),
            "S" => (player_pos.0 + 1, player_pos.1),
            "A" => (player_pos.0, player_pos.1 - 1),
            "D" => (player_pos.0, player_pos.1 + 1),
            _ => {
                println!(" Enter a valid input. ");
                continue;
            }
        };

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
