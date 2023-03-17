use std::{
    env,
    io::{self, Write},
    net::{IpAddr, TcpStream},
    str::FromStr,
    sync::mpsc::Sender,
};

const MAX: u16 = 65535;

struct Arguments {
    flag: String,
    ipaddress: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddress) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddress,
                threads: 4,
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: {} IPADDRESS [FLAGS] [THREADS]", args[0]);
                println!("-j to select how many threads you want \r\n-h or -help to show this help message.");
                return Err("Help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many arguments");
            } else if flag.contains("-j") {
                let ipaddress = match IpAddr::from_str(&args[3]) {
                    Ok(ip) => ip,
                    Err(_) => {
                        return Err("Not a valid IP address, must be a valid IPv4 or IPv6 address.")
                    }
                };
                let threads =
                    match args[2].parse::<u16>() {
                        Ok(t) => t,
                        Err(_) => return Err(
                            "Not a valid number of threads, must be a number between 1 and 65535.",
                        ),
                    };
                return Ok(Arguments {
                    flag,
                    ipaddress,
                    threads,
                });
            } else {
                return Err("Not a valid flag, must be -h, -help, or -j");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err == "Help" {
            std::process::exit(0);
        } else {
            eprintln!("{} Problem parsing arguments: {}", program, err);
            std::process::exit(0);
        }
    });
    println!("Flag: {}", arguments.flag);
    println!(
        "Scanning {} with {} threads",
        arguments.ipaddress, arguments.threads
    );
    let num_threads = arguments.threads;
    let (tx, rx) = std::sync::mpsc::channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        std::thread::spawn(move || {
            scan(tx, i, arguments.ipaddress, num_threads);
        });
    }
    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for p in out {
        println!("{} is open", p);
    }
}

// Scan ports for tcp connection
fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }
        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}
