use unftp_sbe_fs::ServerExt;

#[tokio::main]
pub async fn main() {
    
    let ftp_home = std::env::current_dir().unwrap().join("ftp");
    let server = libunftp::Server::with_fs(ftp_home)
        .greeting("eshare - FTP Server - {version}")
        .passive_ports(50000..65535);
    
    server.listen("127.0.0.1:2121").await;
}