use std::{net::TcpListener, io::Read};


pub struct Server{
    address:String
}

impl Server {
    // assiociated functions
   pub fn new (address:String)->Self{
       Self { address }
    }
 //sort of statics functions
    pub fn run(self){

     print!("Listening to {}..",self.address);
     let listener=TcpListener::bind(&self.address).unwrap();
     loop {
        match listener.accept()  {
            Ok((mut stream,_))=>{
            let mut buffer=[0;1024];
            match stream.read(&mut buffer){
                Ok(_) => {
                    println!("Hey am streaming along {}",String::from_utf8_lossy(&buffer));
                }
                Err(e)=>{ println!("Failed to read from the connection... {}",e)}
            }

            }
            Err(e)=>{ println!("hey whats the error {}",e)}
        } 
       
     }
    }
}
