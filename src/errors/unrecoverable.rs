//note: commented out for now so including the module doesn't cause panics when running
// alongside other modules

pub fn run() {
    unhandled_panic();
    access_beyond_index();
}

fn access_beyond_index() {
//     let v = vec![1, 2, 3];
//     v[99];
}

fn unhandled_panic() {
//     panic!("crash and burn");
}