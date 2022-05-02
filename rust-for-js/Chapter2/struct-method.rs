#[derive(Debug)]
struct URL {
    protocol: String,
    hostname: String,
    pathname: String,
}

// struct를 구현할때 사용

impl URL {
    // &self는 자신을 가리킨다
    // &는 참조값
    // self는 this
    fn to_string(&self) -> String {
        format!("{}://{}/{}", self.protocol, self.hostname, self.pathname)
    }

    fn from(url: &str) -> URL {
        let string = String::from(url);
        let vec: Vec<&str> = string.split("://").collect();
        let protocol = String::from(vec[0]);
        let rest = String::from(vec[1]);
        let vec2: Vec<&str> = rest.split("/").collect();
        let hostname = String::from(vec2[0]);
        let pathname = String::from(vec2[1]);
        URL {
            protocol,
            hostname,
            pathname,
        }
    }
}

fn main() {
    let app = URL {
        protocol: String::from("https"),
        hostname: String::from("yceffort.kr"),
        pathname: String::from("2022/04/chrome-memory-profiler"),
    };
    println!("{}", app.to_string());

    let app2 = URL::from("https://yceffort.kr/2022/04/chrome-memory-profiler");
    println!("{:?}", app2);
    println!("{}", app2.to_string());
}
