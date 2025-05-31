use ssh;
use crate::transport::traits::Transport;


// Структура отвечающая за подключение с помощью ssh
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
