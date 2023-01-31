use rand::Rng;

fn main() {
    let width = 10;
    let height = 10;
    let mut rng = rand::thread_rng();

    let grid = vec![vec![0; width]; height];

    let grid_to_init = &grid[..];

    for row in grid {
        for mut column in row {
            column = rng.gen_range((0..1))
        }
    }

    println!("{:?}", grid_to_init);
}