# 教程
##	动态内存分配
这一节是为了之后的两个教程打基础的章节
### 我们的目的:
实现Trait GlobalAlloc
  ```
	unsafe fn alloc(&self, layout: Layout) -> *mut u8;
	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout);
  ```
为了实现这个trait 必须要实现上面两个函数，也就是分配-回收内存
Layout包括两个字段,size是分配的字节数,align是对齐方式,地址必须是align的倍数,align必须是2的整数幂

我们利用伙伴系统来做这件事

##	物理内存管理
第三章可以把前两章的知识都概括了

首先定义PhysicalAddress与PhysicalNumber的结构体，也就是页号和物理地址
它的形式如下：
```
pub struct PhysicalAddress(pub usize);
pub struct PhysicalNumber (pub usize);
```

然后定义一页的大小：
    pub const PAGE_SIZE: usize = 4096; 必须是2的幂次

定义可以访问的内存区域起始地址：
    pub const MEMORY_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x8000_0000);
/// 可以访问的内存区域结束地址
    pub const MEMORY_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x8800_0000);
正好是8M
	实现一个分配器进行分配和回收
	
其中有代码
``` 
pub fn address(&self) -> PhysicalAddress {
	self.0.into()
}
    /// 帧的物理页号
pub fn page_number(&self) -> PhysicalPageNumber {
    self.0
}
```

所以PhysicalPageNumber必须实现into
但是Into有一个默认的实现,如果U实现了From<T>,T类型调用into就可以转换为U 然后就会调用析构函数

封装一个物理页分配器
```
pub trait Allocator {
    /// 给定容量，创建分配器
    fn new(capacity: usize) -> Self;
    /// 分配一个元素，无法分配则返回 `None`
    fn alloc(&mut self) -> Option<usize>;
    /// 回收一个元素
    fn dealloc(&mut self, index: usize);
}
```

FrameAllocator以Allocator为泛型
```
pub struct FrameAllocator<T: Allocator> {
    /// 可用区间的起始
    start_ppn: PhysicalPageNumber,
    /// 分配器
    allocator: T,
}
```
为这个分配器trait实例化
```
impl<T: Allocator> FrameAllocator<T> {
    /// 创建对象
    pub fn new(range: impl Into<Range<PhysicalPageNumber>> + Copy) -> Self {
        FrameAllocator {
            start_ppn: range.into().start,
            allocator: T::new(range.into().len()),
        }
}
```
Impl Trait这种用法就相当于 使用trait限定的泛型
这里又调用了Range的实现
```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Range<T: From<usize> + Into<usize> + Copy> {
    pub start: T,
    pub end: T,
}//满足Into和From Trait以及Copy Trait
pub fn alloc(&mut self) -> MemoryResult<FrameTracker> {
        self.allocator
            .alloc()
            .ok_or("no available frame to allocate")
            .map(|offset| FrameTracker(self.start_ppn + offset))
 }
 

//这里使用了MemoryResult<FrameTracker>
pub type MemoryResult<T> = Result<T, &'static str>;
成功则返回Ok(FrameTracker(self.start_ppn + offset)) 失败Err

    /// 这个函数会在 [`FrameTracker`] 被 drop 时自动调用，不应在其他地方调用
 pub(super) fn dealloc(&mut self, frame: &FrameTracker) {
        self.allocator.dealloc(frame.page_number() - self.start_ppn);
 }

```	

最后来看帧分配器
```
lazy_static!{
	pub static ref FRAME_ALLOCATOR:Mutex<FrameAllocator<AllocatorImpl>> = Mutex::new(FrameAllocator::new(Range::from(PhysicalPageNumber::ceil(PhysicalAddress::from(*KERNEL_END_ADDRESS))..PhysicalPageNumber::floor(MEMORY_END_ADDRESS),)));
}
```	
lazy_static是给静态变量延迟赋值的宏。
使用这个宏,所有 static类型的变量可在执行的代码在运行时被初始化。 这包括任何需要堆分配,如vector或hash map,以及任何非常量函数调用。需要ref才可以使用

这里利用了1..5与Mutex结构   Mutex是给它“上锁的” 同一时间只能有一个线程操作
	进行分解
```
FrameAllocator::new(Range::from(PhysicalPageNumber::ceil(PhysicalAddress::from(*KERNEL_END_ADDRESS))..PhysicalPageNumber::floor(MEMORY_END_ADDRESS),))  是分配内存的命令
	for Range<T> {
    fn from(range: core::ops::Range<U>) -> Self {
        Self {
            start: range.start.into(),
            end: range.end.into(),
        }
    }
}
```
也就是创建一个Range区间
继续分解：
    PhysicalPageNumber::ceil(PhysicalAddress::from(*KERNEL_END_ADDRESS)) ..PhysicalPageNumber::floor(MEMORY_END_ADDRESS)
    pub static ref KERNEL_END_ADDRESS: PhysicalAddress = PhysicalAddress(kernel_end as usize);

    pub const MEMORY_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x8800_0000);

就是从kernel_end -- MEMORY_END_ADDRESS
```	
pub const fn floor(address: $address_type) -> Self {
   Self(address.0 / PAGE_SIZE)
}
            /// 将地址转换为页号，向上取整
pub const fn ceil(address: $address_type) -> Self {
  Self(address.0 / PAGE_SIZE + (address.0 % PAGE_SIZE != 0) as usize)
}
```
其中 floor 就是 0x8800_0000/4096 其实也就是取整 
Ceil就是kernel_end/4096 根据kernel_end的值 决定取得的起始页是否加一
也就是约定好了有多少页
	
整个调用流程
```
Mutex::new(FrameAllocator::new(Range::from(start..end,)));
	let frame_1 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
  };
}
```
	
然后进行Range赋值 start…end  进行new操作 然后lock
```
pub fn new(range: impl Into<Range<PhysicalPageNumber>> + Copy) -> Self {
        FrameAllocator {
            start_ppn: range.into().start,
            allocator: T::new(range.into().len()),
        }
}
```
start_ppn 是range.into().start起始地址
allocator: range.into().len()就是所有可分配的页
	
```
pub fn alloc(&mut self) -> MemoryResult<FrameTracker> {
        self.allocator
            .alloc()
            .ok_or("no available frame to allocate")
            .map(|offset| FrameTracker(self.start_ppn + offset))
}
```
Start_ppn不变  返回Option<usize> 解析得到 offset就是这个usize  然后FrameTracker里的链表改变
```
fn alloc(&mut self) -> Option<usize> {
        if let Some((start, end)) = self.list.pop() {
            if end - start > 1 {
                self.list.push((start + 1, end));
            }
            Some(start)
        } else {
            None
        }
 }
 ```
具体的栈实现方法
	List初始为[(0,len)]
	经过一个alloc=>[(1,len)]  一直递归进行
	每次返回start  也就是0、1、2..相当于offset 	
```
dealloc 
	fn dealloc(&mut self, index: usize) {
    	self.list.push((index, index + 1));
}
```

也就是将frame.page_number()-self.start_ppn  push进去  证明这个空间已经空了   搞明白了！



## 实验
1. 回答：我们在动态内存分配中实现了一个堆，它允许我们在内核代码中使用动态分配的内存，例如 Vec Box 等。那么，如果我们在实现这个堆的过程中使用 Vec 而不是 [u8]，会出现什么结果

2.回答：algorithm/src/allocator 下有一个 Allocator trait，我们之前用它实现了物理页面分配。这个算法的时间和空间复杂度是什么？
具体可以看代码
```
	fn alloc(&mut self) -> Option<usize> {
        if let Some((start, end)) = self.list.pop() {
            if end - start > 1 {
                self.list.push((start + 1, end));
            }
            Some(start)
        } else {
            None
        }
    }
 ```
只有一个pop和push 所以时间复杂度是O(1)
至于空间复杂度  我们考虑一个极端情况  n个空间已经被分配  现在同时析构
所以空间复杂度是O(n)
####	实现基于线段树的物理页面分配算法
我们仿照第一种栈结构  定义一种分配器命名为 lineAlllocator 

我们使用vec来构造线段树
	
	
建立线段树节点struct
```
struct node{
        left_address:usize, //起始地址
        right_address:usize, //结束地址
        max_free_interval:usize, //连续最长空闲长度
        left_available:uize, //由起始地址向右最长连续空闲长度
        right_available:usize, //由结束地址向左
}
```

建立线段树struct 这里利用了栈结构
我们可以发现 这里存的是线段树的有序结构 就是 i的左子树是2i  右子树是2i+1
```
fn createTree(&mut self){
      let Node{left_address:start,right_address:end,..}=self.list[0];
       let mut stack1=vec![(start,end)];
       let mut stack2=vec![];
       loop{
             if let Some((left,right)) = stack1.pop() {//pop 1 to 2
             if(left!=right)    
                     stack2.push((left,(left+right)/2));                                  
			stack2.push(((left+right)/2+1,right));
                 self.list.push(Node::new(left,(left+right)/2));
					self.list.push(Node::new((left+right)/2+1,right));//build a sort 
                                }
             }else{
                     while let Some((left,right)) = stack2.pop(){
                                stack1.push((left,right));
                     }
                      if(stack1.len()==0){break;}
                    }
                }
      }
  ```
首先我们需要进行两个步骤 ：
1.	寻找合适的节点
2.	进行update 改变节点的分配状态

最后进行dealloc 释放指定的节点

例子：从0-4建立线段树 并分配三个内存空间
```
fn main(){
  let mut s=Tree:new(0,4);
  s.createTree();
  s.alloc().ok_or("error");
  s.alloc().ok_or("error");
  s.alloc().ok_or("error");
}
```
此时的输出：
```
Node {
        left_address: 0,
        right_address: 0,
        max_free_interval: 0,
        left_available: 0,
        right_available: 0,
    },
Node {
        left_address: 1,
        right_address: 1,
        max_free_interval: 0,
        left_available: 0,
        right_available: 0,
},
Node {
        left_address: 2,
        right_address: 2,
        max_free_interval: 0,
        left_available: 0,
        right_available: 0,
},//部分输出

```
   
可以看到内存被正确的分配了  此时释放0跟2两个节点的内存
```
Node {
        left_address: 0,
        right_address: 0,
        max_free_interval: 1,
        left_available: 1,
        right_available: 1,
    },
Node {
        left_address: 0,
        right_address: 2,
        max_free_interval: 1,
        left_available: 1,
        right_available: 1,
},
```
可以看到结果是正确的

