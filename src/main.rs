use rand::Rng;

fn main() {
    let width = 10;
    let height = 10;
    let mut rng = rand::thread_rng();

    let mut grid = vec![vec![0; width]; height];

    for i in 0..width {
        for j in 0..width {
            grid[i][j] = rng.gen_range(0..2);
        }
    }

    println!("{:?}", &grid)
}