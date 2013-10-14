use std::{io, run};
use std::str;
use std::path;

/*

type Semaphore = Option<uint> ; // either None (available) or owner

static mut count: uint = 0; // protected by lock
static mut lock: Semaphore = None; 

fn grab_lock(id: uint) {
    unsafe {
        while (lock.is_some()) {
            ; // wait for lock 
        }
        lock = Some(id);
    }
}

fn release_lock() {
    unsafe {
        lock = None;
    }
}

fn update_count(id: uint) {
    unsafe {
        grab_lock(id);
        count += 1;
        println(fmt!("Count updated by %?: %?", id, count));
        release_lock();
    }
}

fn main() {
        do spawn {
            for num in range(0u, 1000) {
                update_count(num);
            }
        }
}


/*

fn main() {
		
	let (port, chan) : (Port<int>, Chan<int>) = stream();

	let val1 = 3;
	let val2 = 4;
   	
   	do spawn {
    	chan.send(val1);
   	}

   	do spawn{
   		chan.send(val2);
   	}

   	let newval1 = port.recv();
   	let newval2 = port.recv();
   	println(fmt!("%?",newval1));
   	println(fmt!("%?",newval2));
   	

	//do std::task::spawn_sched(std::task::SingleThreaded){
	//	loop{
	//		println("task 1");
	//	}
	//}

	//do std::task::spawn_sched(std::task::SingleThreaded){
	//	loop{
	//		println("task 2");
	//	}
	//}
}
*/
