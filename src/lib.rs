#![feature(trace_macros)]

trace_macros!(true);

#[macro_use]
mod lib {
    #[macro_export]
    macro_rules! http_status {
        ($s:expr, $e:expr) => {
            format!("HTTP/1.1 {} {}\r\n", $s, $e)
        };
    }

    #[macro_export]
    macro_rules! http_header {
        ($h:expr, $v:expr) => {
            format!("{}: {}\r\n", $h, $v)
        };
    }

    #[macro_export]
    macro_rules! import_file {
        ($f:expr) => {{
            use std::fs::File;
            use std::io::Read;
            use std::path::Path;

            let path_str = format!("templates/{}", $f);
            let path = Path::new(&path_str);

            let mut f = File::open(path).expect(&format!(
                "File not found on path {}",
                path.to_str().unwrap()
            ));

            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();

            format!("\r\n{}", contents)
        }};
    }

    #[macro_export]
    macro_rules! http_response {
        ($s:expr, $e:expr, [$(($h:expr, $v:expr)), *], $f:expr) => {{
            let mut response = http_status!($s, $e);

            $(
                response.push_str(&http_header!($h, $v));
            )*

            response.push_str(&import_file!($f));
            
            response
        }};
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn http_status() {
        assert_eq!("HTTP/1.1 200 OK\r\n", http_status!(200, "OK"));
        assert_eq!("HTTP/1.1 404 Not Found\r\n", http_status!(404, "Not Found"));
    }

    #[test]
    fn http_header() {
        assert_eq!(
            "Server: Workshop Rust NIAEFEUP Web Server v0.1\r\n",
            http_header!("Server", "Workshop Rust NIAEFEUP Web Server v0.1")
        );
    }

    #[test]
    fn import_file() {
        assert_eq!(
            format!("\r\n{}", include_str!("../templates/index.html")),
            import_file!("index.html")
        );
    }

    #[test]
    fn http_response() {
        assert_eq!(
            "HTTP/1.1 200 OK\r\nServer: MyServer\r\nAuthorization: JWT Token\r\n\r
<head>
    <title>Workshop Rust by NIAEFEUP</title>
</head>
<body>
    Hello, World!
</body>",
            http_response!(
                200,
                "OK",
                [("Server", "MyServer"), ("Authorization", "JWT Token")],
                "index.html"
            )
        );
    }
}
