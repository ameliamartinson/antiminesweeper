use rand::prelude::*;
use rayon::prelude::*;

fn main() {
    let height = 160;
    let width = 200;
    let mut bombs = vec![vec![0; width]; height];

    let mut counts = bombs.clone();

    bombs.par_iter_mut().for_each(|row| {
        let mut rng = rand::thread_rng();
        for item in row.iter_mut().take(width) {
            if rng.gen::<f32>() < 0.1 {
                *item = 1;
            }
        }
    });

    for row in 0..height {
        for col in 0..width {
            counts[row][col] = 0;
            if row == 0 {
                if col == 0 {
                    counts[row][col] = bombs[row][col]
                        + bombs[row + 1][col]
                        + bombs[row][col + 1]
                        + bombs[row + 1][col + 1];
                } else if col == width - 1 {
                    counts[row][col] = bombs[row][col]
                        + bombs[row + 1][col]
                        + bombs[row][col - 1]
                        + bombs[row + 1][col - 1];
                } else {
                    counts[row][col] = bombs[row][col]
                        + bombs[row + 1][col]
                        + bombs[row][col + 1]
                        + bombs[row + 1][col + 1]
                        + bombs[row][col - 1]
                        + bombs[row + 1][col - 1];
                }
            } else if row == height - 1 {
                if col == 0 {
                    counts[row][col] = bombs[row][col]
                        + bombs[row][col + 1]
                        + bombs[row - 1][col + 1]
                        + bombs[row - 1][col];
                } else if col == width - 1 {
                    counts[row][col] = bombs[row][col]
                        + bombs[row][col - 1]
                        + bombs[row - 1][col - 1]
                        + bombs[row - 1][col];
                } else {
                    counts[row][col] = bombs[row][col]
                        + bombs[row][col + 1]
                        + bombs[row - 1][col + 1]
                        + bombs[row - 1][col]
                        + bombs[row - 1][col - 1]
                        + bombs[row][col - 1];
                }
            } else if col == 0 {
                counts[row][col] = bombs[row][col]
                    + bombs[row - 1][col]
                    + bombs[row - 1][col + 1]
                    + bombs[row][col + 1]
                    + bombs[row + 1][col + 1]
                    + bombs[row + 1][col];
            } else if col == width - 1 {
                counts[row][col] = bombs[row][col]
                    + bombs[row - 1][col]
                    + bombs[row - 1][col - 1]
                    + bombs[row][col - 1]
                    + bombs[row + 1][col - 1]
                    + bombs[row + 1][col];
            } else {
                counts[row][col] = bombs[row][col]
                    + bombs[row - 1][col]
                    + bombs[row - 1][col + 1]
                    + bombs[row][col + 1]
                    + bombs[row + 1][col + 1]
                    + bombs[row + 1][col]
                    + bombs[row - 1][col - 1]
                    + bombs[row][col - 1]
                    + bombs[row + 1][col - 1];
            }
            if bombs[row][col] == 1 {
                counts[row][col] = -1;
            }
        }
    }

    for i in counts {
        for j in i {
            if j == 0 {
                print!(" ");
            } else if j == -1 {
                print!("B");
            } else {
                print!("{}", j);
            }
        }
        println!();
    }
}
