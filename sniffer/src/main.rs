use std::io::Write;
use std::net::TcpStream;
use std::{io, process};
use std::sync::mpsc::{Sender, channel};
use std::{env, net::IpAddr};
use std::str::FromStr;
use std::thread;

const MAX:u16 = 65535;

struct Arguments{
    flag: String,
    ipaddr: IpAddr,
    threads: u16
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str>{
        let args_len = args.len();
        if args_len < 2{
            return Err("not enough arguments");
        }
        if args_len > 4{
            return Err("too many arguments");
        }
        let flag = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&flag) {
            return Ok(Arguments{flag: String::from(""), ipaddr, threads: 4})
        }
        if flag.contains("-h") || flag.contains("-help") &&  args_len == 2{
            println!("Usage: -j to select how many threads you want
            \r\n      -h or -help to show this help message");
            return Err("help");
        }
        if flag.contains("-h") || flag.contains("-help"){
            return Err("too many arguments");
        }
        if flag.contains("-j"){
            let ipaddr = match IpAddr::from_str(&args[3]) {
                Ok(s) => s,
                Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6")
            };
            let threads = match args[2].parse::<u16>(){
                Ok(s) => s,
                Err(_) => return Err("failed to parse thread number")
            };
            return Ok(Arguments{flag, ipaddr, threads});
        }
        return Err("invalid syntax");
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err|{
            if err.contains("help"){
                process::exit(0);
            }
            eprintln!("{} program parsing arguments: {}", program, err);
            process::exit(0);
        }
    );
    let num_threads = arguments.threads;
    let ipaddr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads{
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, ipaddr, num_threads);
        });
    };
    let mut out = vec![];
    drop(tx);
    for p in rx{
        out.push(p);
    }
    println!("");
    out.sort();
    for p in &out{
        println!("{} is open", p);
    }
    if out.len() == 0{
        println!("no port is open");
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16){
    let mut port: u16 = start_port + 1;
    loop{
        match TcpStream::connect((addr, port)){
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            },
            Err(_) => {}
        };
        if (MAX - port) <= num_threads{
            break;
        } 
        port += num_threads;
    }
}
