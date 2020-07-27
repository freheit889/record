pub trait LineAllocator{
        fn new(start:usize,end:usize)->Self;
        fn alloc(&mut self)->Option<usize>;
        fn dealloc(&mut self,index:usize);
	fn createTree(&mut self);
}
#[derive(Debug)]
struct Node{
	left_address:usize,
	right_address:usize,
	max_free_interval:usize,
	left_available:usize,
	right_available:usize,
}

struct Tree{
	list:Vec<Node>
}

impl LineAllocator for Tree{
	fn new(start:usize,end:usize)->Self{
		Self {
            		list: vec![Node::new(start,end)],//root
        	}
	}

	fn alloc(&mut self)->Option<usize>{
		let mut stack=vec![0];
		if(self.list[0].max_free_interval<1){
			return None;
		}else{
			let mut i=1;
			loop{
				if(self.list[i].length()>=1&&self.list[i].max_free_interval>=1){
                                        stack.push(i);//record the father
                                        i=i*2+1;
                                        continue;
                               	}
				
                                if(self.list[i+1].length()>=1&&self.list[i+1].max_free_interval>=1){
                                        i=i+1;
                                        stack.push(i);
                                        i=i*2+1;
                                        continue;
                                }
				if(self.list[i].max_free_interval>0){
					self.list[i].left_available=0;
                             		self.list[i].right_available=0;
	                                self.list[i].max_free_interval=0;
				}else{
					i+=1;
					self.list[i].left_available=0;
                                        self.list[i].right_available=0;
                                        self.list[i].max_free_interval=0;
				}
				break;
			}
			let t=i;
			loop{//update father
				if let Some(j)=stack.pop(){
					i=i+i%2;
					self.list[j].left_available=self.list[i-1].left_available;
					self.list[j].right_available=self.list[i].right_available;	
					self.list[j].max_free_interval=self.list[i-1].right_available+self.list[i].left_available;
					if(self.list[i-1].right_available+self.list[i].left_available>self.list[i].right_available){
						 self.list[j].max_free_interval=self.list[i-1].right_available+self.list[i].left_available;
					}else{
                                                 self.list[j].max_free_interval=self.list[i].right_available;
					}
					i=j; //son become father to find the grandfather
				}else{
					break;
				}
			}
			
			return 	Some(self.list[t].left_address);
		}
	}
	fn createTree(&mut self){
		let Node{left_address:start,right_address:end,..}=self.list[0];
		let mut stack1=vec![(start,end)];
		let mut stack2=vec![];
		loop{
			if let Some((left,right)) = stack1.pop() {//pop 1 to 2
				if(left!=right){
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
	fn dealloc(&mut self,start:usize){
		let mut i=1;// same alloc
		let mut stack=vec![0]; 
		loop{//like alloc   need to update father
			if(start<=self.list[i].right_address){
			      //find the bottom 
				if(self.list[i].length()==0){
					//find it 
					self.list[i].max_free_interval=1;
					self.list[i].left_available=1;
					self.list[i].right_available=1;
					break;
				}
				stack.push(i);
				i=2*i+1;		
			}else{
				i+=1
			}
		}
		 let t=i;
                 loop{//update father
                     if let Some(j)=stack.pop(){
                          i=i+i%2;
                          self.list[j].left_available=self.list[i-1].left_available;
                          self.list[j].right_available=self.list[i].right_available;
                          self.list[j].max_free_interval=self.list[i-1].right_available+self.list[i].left_available;
                          if(self.list[i-1].right_available+self.list[i].left_available>self.list[i].right_available){
                          	self.list[j].max_free_interval=self.list[i-1].right_available+self.list[i].left_available;
                          }else{
                               	self.list[j].max_free_interval=self.list[i].right_available;
                          }
                          i=j; //son become father to find the grandfather
                     }else{
                          break;
                         }
               }
	}
}
	
impl Node{
	fn new(start:usize,end:usize)->Self{
		Self{
			left_address:start,
			right_address:end,
			max_free_interval:end-start+1,
			left_available:end-start+1,
			right_available:end-start+1,
		}
	}
	fn length(&self)->usize{
		self.right_address-self.left_address
	}	
}

fn main(){
	let mut s=Tree::new(0,4);
	s.createTree();
	s.alloc().ok_or("error");
	s.alloc().ok_or("error");
	s.alloc().ok_or("error");
	s.dealloc(2);
	s.dealloc(0);
	println!("{:#?}",s.list);	
}
