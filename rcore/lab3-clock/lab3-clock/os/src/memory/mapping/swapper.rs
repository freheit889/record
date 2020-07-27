//! 页面置换算法

use super::*;
use crate::memory::{frame::FrameTracker, *};
use alloc::{collections::VecDeque,vec::Vec};// 管理一个线程所映射的页面的置换操作
pub trait Swapper {
    /// 新建带有一个分配数量上限的置换器
    fn new(quota: usize) -> Self;

    /// 是否已达到上限
    fn full(&self) -> bool;

    /// 取出一组映射
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)>;

    /// 添加一组映射（不会在以达到分配上限时调用）
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, entry: *mut PageTableEntry);
     
//    fn sign(&mut self,vpn:VirtualPageNumber);
    /// 只保留符合某种条件的条目（用于移除一段虚拟地址）
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool);
}

pub type SwapperImpl = CLOCKSwapper;

/// 页面置换算法基础实现：FIFO
pub struct FIFOSwapper {
    /// 记录映射和添加的顺序
    queue: VecDeque<(VirtualPageNumber, FrameTracker)>,
    /// 映射数量上限
    quota: usize,
}

pub struct CLOCKSwapper{
    quota:usize,
    queue:Vec<(VirtualPageNumber,FrameTracker,usize)>
}


impl Swapper for FIFOSwapper {
    fn new(quota: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            quota,
        }
    }
    fn full(&self) -> bool {
        self.queue.len() == self.quota
    }
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)> {
        self.queue.pop_front()
    }
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, _entry: *mut PageTableEntry) {
        self.queue.push_back((vpn, frame));
    }
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.queue.retain(|(vpn, _)| predicate(vpn));
    }
    
  //  fn sign(&mut self,vpn:VirtualPageNumber){
	
    //}
}

impl Swapper for CLOCKSwapper{
   fn new(quota: usize) -> Self {
        Self {
            queue: Vec::new(),
            quota,
        }
    }
    fn full(&self) -> bool {
        self.queue.len() == self.quota
    }
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)> {
        let mut i=0;
	let len=self.queue.len();
	loop{
		unsafe{
			let p=self.queue[i].2 as *mut PageTableEntry;
			let mut flag=(*p).flags().clone();
			if flag.contains(Flags::ACCESSED){
				flag.set(Flags::ACCESSED,false);
				(*p).set_flags(flag);		
			}else{
				let s=self.queue.remove(i);
				return Some((s.0,s.1));
			}
			i=(i+1)%len;
		}
	}
    }
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, _entry: *mut PageTableEntry) {
        self.queue.push((vpn, frame,_entry as usize));

    }
    /*
    fn sign(&mut self,vpn:VirtualPageNumber){
	let len=self.queue.len();
	println!("{}",len);
        println!("xxxxxxxxxxxxxxxxxxxxxx");

	if len>0{
		for i in 0..len{
			if(vpn==self.queue[i].0.clone()){
				self.queue[i].2=1;
			}

		}
	}
    }*/

    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.queue.retain(|(vpn, _,_)| predicate(vpn));
    }

}
