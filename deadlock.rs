//Changed semaphore to deadlock


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
        if(id==0){
        	grab_lock(id+1);
    	}
        else{
        	grab_lock(id-1);
        }
        release_lock;
    }
}

fn main() {
    for num in range(0u, 2) {
        do spawn {
            for _ in range(0u, 100) {
                update_count(num);
            }
        }
    }
}
