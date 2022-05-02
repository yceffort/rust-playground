struct URL {
    protocol: String,
    hostname: String,
    pathname: String,
}

// struct를 구현할때 사용
impl URL {
    fn toString(&self) -> String {
        format!("{}://{}/{}", self.protocol, self.hostname, self.pathname)
    }
}

fn main() {
    let app = URL {
        protocol: String::from("https"),
        hostname: String::from("yceffort.kr"),
        pathname: String::from("2022/04/chrome-memory-profiler"),
    };
    println!("{}", app.toString());
}
