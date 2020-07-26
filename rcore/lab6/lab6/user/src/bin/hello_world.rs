#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::syscall::*;
use user_lib::alloc::string::String;
use user_lib::console::*;
use user_lib::pipe::*;
#[no_mangle]
pub fn main() -> usize {
  //  let (mut write_fd, mut read_fd) = pipe();
    let s=sys_clone();
    if(s==0){
	println!("I am child ,my id is {}",sys_pid());
//	let mut buffer = [0u8; 64];
//	let len = sys_read(read_fd, &mut buffer);
//	println!("{}", core::str::from_utf8(&buffer).unwrap());
    }else{
        println!("I am fathre ,my id is {}",s);
//	sys_write(write_fd, "hello_rcore".as_bytes());
    } 

  //  println!("{}",sys_pid());
    let fd=open("test.txt");    
    let mut buff=[0u8;1024];

 
    sys_read(fd,&mut buff);

    let s=String::from_utf8_lossy(&buff);
    println!("{}",s);
    0
}
