use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{fork, getpid, getppid};

fn main() {
    let mut nums = [0, 1, 2, 3, 4];
    const SIZE: usize = 5;
    let pid = fork();

    match pid.expect("Fork Failed: Unable to create child process!") {
        Child => 
        for i in 0..SIZE {
            nums[i] *= -(i as i32);
            println!("CHILD: {}", nums[i]  
        )}
        ,
        
        Parent { child } => {
            wait();
            for i in 0..SIZE {
            println!(
                "PARENT: {}", nums[i]                
            )};
        }
    }
}


