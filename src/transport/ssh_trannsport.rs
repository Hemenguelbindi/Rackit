use ssh;
use crate::transport::traits::Transport;



pub struct SSHTransport{
    port: String,
    host: String,
}



impl SSHTransport{
    fn new(self, port: String, host: String)-> Self{
        Self{
            port,
            host,
        }
    }
}
