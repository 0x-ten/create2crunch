use create2crunch::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed parsing arguments: {err}");
        process::exit(1);
    });

    #[cfg(feature = "gpu")]
    {
        if config.gpu_device != 255 {
            if let Err(e) = create2crunch::gpu(config) {
                eprintln!("GPU application error: {e}");
                process::exit(1);
            }
            return;
        }
    }

    // If GPU is not enabled or gpu_device is 255, use CPU mode
    if let Err(e) = create2crunch::cpu(config) {
        eprintln!("CPU application error: {e}");
        process::exit(1);
    }
}
