impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n=nums.len();
        let mut num:Vec<i32>=vec![];
        let mut res:Vec<Vec<i32>>=vec![];
        if(n<3){
            return res;
        }
        //首先对nums进行排序  因为不是mut定义  所以定义一个num数组接受nums排序
        for i in nums.iter(){
            num.push(*i);
        }
        num.sort();
        //排序完毕
        let mut L=0;
        let mut R=0;
        for i in 0..n{
            if(num[i]>0){//最小值大于0
                return res;
            }
            if(i>0 && num[i]==num[i-1]){//有重复元素
                continue;
            }
            L=i+1;
            R=n-1;
            while(L<R){
                if(num[i]+num[L]+num[R]==0){
                    res.push([num[i],num[L],num[R]].to_vec());
                    while(L<R && num[L]==num[L+1]){
                        L=L+1;
                    }
                    while(L<R && num[R]==num[R-1]){
                        R=R-1;
                    }
                    L=L+1;
                    R=R-1
                }else if(num[i]+num[L]+num[R]>0){
                    R=R-1;
                }else{
                    L=L+1;
                }
            }
        }    
        res

    }
}
