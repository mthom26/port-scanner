use clap::{App, Arg};

pub struct Config {
    pub url: String,
    pub threads: u16,
    pub start_port: u16,
    pub end_port: u16
}

impl Config {
    pub fn new() -> Self {
        let matches = App::new("Port Scanner")
            .arg(Arg::with_name("url")
                    .help("The URL to scan")
                    .required(true))
            .arg(Arg::with_name("threads")
                    .help("Number of threads to use")
                    .short("t")
                    .long("threads")
                    .takes_value(true)
                    .value_name("threads")
                    .default_value("100"))
            .arg(Arg::with_name("ports")
                    .help("Port range to scan")
                    .short("p")
                    .long("ports")
                    .takes_value(true)
                    .number_of_values(2)
                    .value_names(&["Start Port", "End Port"]))
            .get_matches();
        
        println!("{:?}", matches);

        let url = matches.value_of("url").unwrap().to_owned();
        let threads: u16 = matches.value_of("threads").unwrap().parse().unwrap();
        let mut port_range = matches.values_of("ports").unwrap();
        let start_port: u16 = port_range.next().unwrap().parse().unwrap();
        let end_port: u16 = port_range.next().unwrap().parse().unwrap();
        println!("Url: {}", url);
        println!("Num Threads: {}", threads);
        println!("Ports: {} - {}", start_port, end_port);

        Config {
            url,
            threads,
            start_port,
            end_port
        }
    }
}
