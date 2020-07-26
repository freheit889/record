use super::*;
use alloc::{vec,vec::Vec};
pub static BigStride:usize=100;
pub static BigTime:usize=10;


struct StriThread<ThreadType:Clone+Eq>{
	birth_time:usize,
	stride:usize,
	pass:usize,
	pub thread:ThreadType,
}

pub struct StriScheduler<ThreadType:Clone+Eq>{
	time:usize,
	pool:Vec<StriThread<ThreadType>>
}

impl <ThreadType:Clone+Eq> Default for StriScheduler<ThreadType>{
	fn default()->Self{
		Self{
			time:0,
			pool:vec![]
		}
	}
}

impl <ThreadType:Clone+Eq> Scheduler<ThreadType> for StriScheduler<ThreadType>{
	type Priority=usize;
	fn add_thread(&mut self,thread:ThreadType,priority:usize){
		self.pool.push(StriThread{
			birth_time:self.time,
			stride:BigStride/priority,
			pass:BigStride/priority,
			thread,
		})
	}
	
	fn get_next(&mut self)->Option<ThreadType>{
		self.time+=1;
		let mut min=0;
		if self.pool.len()>0{
			for i in 1..self.pool.len(){
				if self.pool[min].stride>self.pool[i].stride{
					min=i;
				}
			}	
			self.pool[min].stride+=self.pool[min].pass;
			return Some(self.pool[min].thread.clone());
		}
		None
	}
	
	fn remove_thread(&mut self,thread:&ThreadType){
		for i in 0..self.pool.len(){
                        if self.pool[i].thread==*thread{
				self.pool.remove(i);
				break;
			}
                                
                }
	}

	fn set_priority(&mut self, _thread: ThreadType, _priority: usize) {
	} 
}


