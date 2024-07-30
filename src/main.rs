use std::cmp::{max, min};
use std::env;
use std::io::Read;
use std::process::{Command, Stdio};

use config::Config;
use dotool::DoTool;

mod config;
mod dotool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    test_hyprland();

    let config = Config::new()?;
    let mut dotool = DoTool::new()?;

    const WAIT_SECONDS: u64 = 4;
    println!("program starts in: {} seconds", WAIT_SECONDS);
    std::thread::sleep(std::time::Duration::from_secs(WAIT_SECONDS));
    println!("program has started!");

    loop {
        match spawn_task("hyprctl", vec!["cursorpos"]) {
            Some(cursor_pos) => {
                let cursor_str: Vec<&str> = cursor_pos.split(", ").collect();
                let cur_x = cursor_str[0]
                    .parse::<isize>()
                    .expect("Couldn't parse cursor's X position");
                let cur_y = cursor_str[1]
                    .parse::<isize>()
                    .expect("Couldn't parse cursor's Y position");

                // Calculate relative cursor position
                let relative_x = cur_x - config.monitor_x;
                let relative_y = cur_y - config.monitor_y;

                let clamped_x = min(max(relative_x, 0), config.screen_width as isize);
                let clamped_y = min(max(relative_y, 0), config.screen_height as isize);

                println!("");
                println!("clamped_x: {}", clamped_x);
                println!("clamped_y: {}", clamped_y);
                println!("cur_x: {}", cur_x);
                println!("cur_y: {}", cur_y);

                if clamped_x <= config.edge_offset {
                    println!("outside of clamp_x border (left)");
                    let y_percent = clamped_y as f32 / config.screen_height as f32;
                    dotool.write(format!("mouseto 0.8 {}", y_percent).as_str())?
                }
                if clamped_x + config.edge_offset >= config.screen_width {
                    println!("outside of clamp_x border (right)");
                    let y_percent = clamped_y as f32 / config.screen_height as f32;
                    dotool.write(format!("mouseto 0.2 {}", y_percent).as_str())?
                }

                if clamped_y <= config.edge_offset {
                    println!("outside of clamp_y border (up)");
                    let x_percent = clamped_x as f32 / config.screen_width as f32;
                    dotool.write(format!("mouseto {} 0.8", x_percent).as_str())?
                }

                if clamped_y + config.edge_offset >= config.screen_height {
                    println!("outside of clamp_y border (down)");
                    let x_percent = clamped_x as f32 / config.screen_width as f32;
                    dotool.write(format!("mouseto {} 0.2", x_percent).as_str())?
                }
            }
            None => {}
        }
    }
}

fn spawn_task(command: &str, args: Vec<&str>) -> Option<String> {
    let mut output = Command::new(command)
        .stdout(Stdio::piped())
        .args(args)
        .spawn()
        .expect("failed to spawn dotool");

    output.wait().expect("Failed to wait for task");
    if let Some(mut out) = output.stdout {
        let mut output_str = String::new();
        out.read_to_string(&mut output_str)
            .expect("failed to read stdout");

        Some(output_str.trim().to_string())
    } else {
        None
    }
}

fn test_hyprland() {
    match env::var_os("HYPRLAND_INSTANCE_SIGNATURE") {
        Some(env) => {
            if env.is_empty() {
                panic!("WARNING: You need Hyprland running to use this program.")
            }
        }
        None => panic!("WARNING: You need Hyprland running to use this program."),
    }
}
