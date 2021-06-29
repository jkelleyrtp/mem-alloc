use argh::FromArgs;

#[derive(FromArgs)]
/// Allocate memory
struct Cfg {
    /// amount of memory to allocate (in megabyes)
    #[argh(option, default = "200")]
    mb: usize,

    /// chunk size in megabytes
    #[argh(option, default = "10")]
    chunk: usize,
}

fn main() {
    let cfg: Cfg = argh::from_env();

    let mut datas = Vec::new();

    let mut total_alloc = 0;
    let iters = cfg.mb / cfg.chunk + 1;
    for _ in 0..iters {
        // 1kb is 256 i32
        // 1mb is 256,000 i32
        let size = 1024 * 1024 * cfg.chunk;
        println!("Trying to push {} mb onto the heap", cfg.chunk);
        datas.push(vec![0_u8; size]);
        total_alloc += cfg.chunk;
        println!("Succeeded. Total: {}  \n", total_alloc);
        std::thread::sleep_ms(50);
    }
    println!("Sleeping for 100s before memory is cleaned up");
    std::thread::sleep_ms(100000);
    println!("{}", datas.len());
}
