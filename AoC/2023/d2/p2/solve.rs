use std::io;

fn get_game_infos(game: &str) -> Vec<&str> {
    game.split(";").map(|info| info.trim()).collect()
}

fn process_game_string(game: &str) -> Vec<&str> {
    game.split(":").map(|data| data.trim()).collect()
}

fn process_ball(ball: &str) -> (u32, &str) {
    let ball_data: Vec<&str> = ball.split(" ").collect();
    let no_ball = ball_data[0].parse::<u32>().unwrap();

    (no_ball, ball_data[1])
}

fn process_game_info(game_info: &str) -> [u32; 3] {
    let balls: Vec<(u32, &str)> = game_info.split(",").map(|inf| process_ball(inf.trim())).collect();

    let mut max_balls: [u32; 3] = [0, 0, 0]; // red, green, blue

    for (no_ball, ball_name) in balls {
        match ball_name {
            "red" => if no_ball > max_balls[0] { max_balls[0] = no_ball },
            "green" => if no_ball > max_balls[1] { max_balls[1] = no_ball },
            "blue" => if no_ball > max_balls[2] { max_balls[2] = no_ball },
            _ => ()
        };
    }

    max_balls
}

fn process_game_infos(game_infos: Vec<&str>) -> u32 {
    let mut max_balls: [u32; 3] = [0, 0, 0]; // red, green, blue
    for game_info in game_infos {
        let [r, g, b] = process_game_info(game_info);
        if r > max_balls[0] { max_balls[0] = r };
        if g > max_balls[1] { max_balls[1] = g };
        if b > max_balls[2] { max_balls[2] = b };
    }

    max_balls[0] * max_balls[1] * max_balls[2]
}

fn check_balls(game: &str) -> u32 {
    let game_data = process_game_string(game);

    if game_data.len() != 2 {
        panic!("Invalid game data");
    }

    let game_infos = get_game_infos(&game_data[1]);
    
    process_game_infos(game_infos)
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
        let game_removed_str = buffer_clone.trim().strip_prefix("game ").unwrap();

        buffer.clear();
        result += check_balls(game_removed_str);
    }
    println!("{}", result);
}