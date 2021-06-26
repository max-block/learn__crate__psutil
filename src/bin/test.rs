use psutil::process::processes;

fn main() {
    for p in processes().unwrap() {
        if let Ok(p) = p {
            let cmd = p
                .cmdline() // not implemented on macOS!!!
                .unwrap()
                .unwrap_or_else(|| format!("[{}]", p.name().unwrap()));
            println!("pid: {}, cmd: {:?}", p.pid(), cmd);
        }
    }
}
