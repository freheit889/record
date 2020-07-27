在群里各位大佬的帮助下，以及借鉴了@yunwei37同学的代码 最终实现了时钟置换算法

代码如下
```
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
}
```

之前一直被访问卡着，不知道从哪里访问了页面，后来才发现这件事是cpu做的，谜题解开了

## 测试结果

### FiFo
```
...
page_fault 205
page_fault 206
page_fault 207
page_fault 208
page_fault 209
```

### Clock
```
...
page_fault 196
page_fault 197
page_fault 198
page_fault 199
page_fault 200
page_fault 201
```

性能确实有了提升
