use std::io;

const MAX_BALLS: [i32; 3] = [12, 13, 14]; // red, green, blue

fn get_game_infos(game: &str) -> Vec<&str> {
    game.split(";").map(|info| info.trim()).collect()
}

fn process_game_string(game: &str) -> Vec<&str> {
    game.split(":").map(|data| data.trim()).collect()
}

fn process_ball(ball: &str) -> (i32, &str) {
    let ball_data: Vec<&str> = ball.split(" ").collect();
    let no_ball = ball_data[0].parse::<i32>().unwrap();

    (no_ball, ball_data[1])
}

fn process_game_info(game_info: &str) -> bool {
    let balls: Vec<(i32, &str)> = game_info.split(",").map(|inf| process_ball(inf.trim())).collect();

    for (no_ball, ball_name) in balls {
        let expected = match ball_name {
            "red" => no_ball <= MAX_BALLS[0],
            "green" => no_ball <= MAX_BALLS[1],
            "blue" => no_ball <= MAX_BALLS[2],
            _ => false
        };

        if !expected {
            return false;
        }
    }

    true
}

fn process_game_infos(game_infos: Vec<&str>) -> bool {
    for game_info in game_infos {
        if !process_game_info(game_info) {
            return false;
        }
    }

    true
}

fn check_balls(game: &str) -> u32 {
    let game_data = process_game_string(game);

    if game_data.len() != 2 {
        panic!("Invalid game data");
    }

    let game_infos = get_game_infos(&game_data[1]);
    if !process_game_infos(game_infos) {
        return 0;
    }

    let game_round = game_data[0].parse::<u32>().unwrap();
    game_round
}


fn main() {
    let mut buffer = String::new();
    let mut result = 0;
    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer == "\n" || buffer.len() == 0 {
            break;
        }

        buffer.make_ascii_lowercase();

        let buffer_clone = buffer.clone();
        let striped_game = buffer_clone.strip_suffix("\n").unwrap();
        let game_removed_str = striped_game.strip_prefix("game ").unwrap();

        buffer.clear();
        result += check_balls(game_removed_str);
    }
    println!("{}", result);
}