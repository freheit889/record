//! 进程相关的内核功能

use super::*;

pub(super) fn sys_exit(code: usize) -> SyscallResult {
    println!(
        "thread {} exit with code {}",
        PROCESSOR.lock().current_thread().id,
        code
    );
    SyscallResult::Kill
}



pub (super) fn sys_pid()->SyscallResult{
	SyscallResult::Proceed(PROCESSOR.lock().current_thread().id)	
}


pub (super) fn sys_fpid()->SyscallResult{
        SyscallResult::Proceed(1)
}


pub (super) fn sys_clone(context:Context)->SyscallResult{
	let id=PROCESSOR.lock().current_thread().id;
	let thread=PROCESSOR.lock().current_thread().Clone(&context);
	PROCESSOR.lock().add_thread(thread.unwrap());
	SyscallResult::Proceed(id)
}

