use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt,AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").await.unwrap();

    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((mut stream,_)) => {
                println!("accepted new connection : {:?}",stream);
                tokio::spawn(async move {

                
                let mut buf = [0;256];
                loop{
                    //println!("{:?}",buf);
                    let read_count = stream.read(&mut buf).await.unwrap();
                    //println!("{}",read_count);
                    if read_count == 0 {
                        break;
                    }
                    stream.write(b"+PONG\r\n").await.unwrap();
                }
                });
            }
            Err(e) => {
                println!("error : {}",e);
            }
        
    };
}
}

