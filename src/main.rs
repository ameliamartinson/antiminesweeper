use colored::*;
use rand::prelude::*;
use rayon::prelude::*;

fn main() {
    let height = 30;
    let width = 50;
    let mut bombs = vec![0; width * height];

    let mut counts = bombs.clone();

    bombs.par_iter_mut().for_each(|item| {
        let mut rng = rand::thread_rng();
        let seed = rng.gen::<f32>();
        if seed < 0.1 {
            *item = 1;
        }
    });

    counts.par_iter_mut().enumerate().for_each(|(i, value)| {
        let col = i % width;
        let row = i / width;
        *value = 0;
        if row == 0 {
            if col == 0 {
                *value = bombs[row * width + col]
                    + bombs[row * width + col + 1]
                    + bombs[(row + 1) * width + col]
                    + bombs[(row + 1) * width + col + 1];
            } else if col == width - 1 {
                *value = bombs[row * width + col]
                    + bombs[(row + 1) * width + col]
                    + bombs[row * width + col - 1]
                    + bombs[(row + 1) * width + col - 1];
            } else {
                *value = bombs[row * width + col]
                    + bombs[(row + 1) * width + col]
                    + bombs[row * width + col + 1]
                    + bombs[(row + 1) * width + col + 1]
                    + bombs[row * width + col - 1]
                    + bombs[(row + 1) * width + col - 1];
            }
        } else if row == height - 1 {
            if col == 0 {
                *value = bombs[row * width + col]
                    + bombs[row * width + col + 1]
                    + bombs[(row - 1) * width + col + 1]
                    + bombs[(row - 1) * width + col];
            } else if col == width - 1 {
                *value = bombs[row * width + col]
                    + bombs[row * width + col - 1]
                    + bombs[(row - 1) * width + col - 1]
                    + bombs[(row - 1) * width + col];
            } else {
                *value = bombs[row * width + col]
                    + bombs[row * width + col + 1]
                    + bombs[(row - 1) * width + col + 1]
                    + bombs[(row - 1) * width + col]
                    + bombs[(row - 1) * width + col - 1]
                    + bombs[row * width + col - 1];
            }
        } else if col == 0 {
            *value = bombs[row * width + col]
                + bombs[(row - 1) * width + col]
                + bombs[(row - 1) * width + col + 1]
                + bombs[row * width + col + 1]
                + bombs[(row + 1) * width + col + 1]
                + bombs[(row + 1) * width + col]
        } else if col == width - 1 {
            *value = bombs[row * width + col]
                + bombs[(row - 1) * width + col]
                + bombs[(row - 1) * width + col - 1]
                + bombs[row * width + col - 1]
                + bombs[(row + 1) * width + col - 1]
                + bombs[(row + 1) * width + col]
        } else {
            *value = bombs[row * width + col]
                + bombs[(row - 1) * width + col]
                + bombs[(row - 1) * width + col + 1]
                + bombs[row * width + col + 1]
                + bombs[(row + 1) * width + col + 1]
                + bombs[(row + 1) * width + col]
                + bombs[(row - 1) * width + col - 1]
                + bombs[row * width + col - 1]
                + bombs[(row + 1) * width + col - 1];
        }
        if bombs[i] == 1 {
            *value = -1;
        }
    });

    for (i, count) in counts.iter().enumerate() {
        match count {
            //-1 => print!("{}", "B".black().bold().on_color("red")),
            //0 => print!("{}", " ".on_color("white")),
            //1 => print!("{}", "1".blue().bold().on_color("white")),
            //2 => print!("{}", "2".green().bold().on_color("white")),
            //3 => print!("{}", "3".yellow().bold().on_color("white")),
            //4 => print!("{}", "4".red().bold().on_color("white")),
            //5 => print!("{}", "5".black().bold().on_color("white")),
            //_ => print!("{}", counts[i]),
            -1 => print!("{}", "  ".on_color("black")),
            0 => print!("{}", "  ".on_color("white")),
            1 => print!("{}", "  ".on_color("blue")),
            2 => print!("{}", "  ".on_color("green")),
            3 => print!("{}", "  ".on_color("yellow")),
            4 => print!("{}", "  ".on_color("red")),
            5 => print!("{}", "  ".black().bold().on_color("white")),
            _ => print!("{}", counts[i]),
        }
        if (i + 1) % width == 0 && i > 0 {
            println!();
        }
    }
}
