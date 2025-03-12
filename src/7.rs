use rand::Rng;

fn generate_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1, 101)
}
