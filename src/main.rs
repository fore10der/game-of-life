use rand::Rng;

fn main() {
    let width = 10;
    let height = 10;
    let mut rng = rand::thread_rng();

    let grid = vec![vec![0; width]; height];


    for row in grid {
        for mut column in row {
            column = rng.gen_range((0..1))
        }
    }

}