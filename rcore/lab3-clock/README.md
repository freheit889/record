## 已实现算法
1 时钟置换算法  
2 改进的时钟置换算法

代码都放在了一起
### 时钟置换算法
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


### 改进的时钟置换算法
这个修改起来很简单，只需要用到修改位即可

D 表示 Dirty，如果为 1 表示自从上次 D 被清零后，有虚拟地址通过这个页表项进行写入

### 代码

```
fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)> {
        let mut i=0;
        let len=self.queue.len();
        loop{
                unsafe{
                        let p=self.queue[i].2 as *mut PageTableEntry;
                        let mut flag=(*p).flags().clone();
                        if flag.contains(Flags::DIRTY){
                                flag.set(Flags::DIRTY,false);
                                (*p).set_flags(flag);
                        }else if flag.contains(Flags::ACCESSED){
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

```


### 测试结果
```
...
page_fault 187
page_fault 188
page_fault 189
page_fault 190
page_fault 191
page_fault 192
page_fault 193
page_fault 194
page_fault 195
page_fault 196
```

相比于时钟置换算法又有一定的性能提升
>>>>>>> 91171184057354f2ef65e3917e57546ed64c9daa
