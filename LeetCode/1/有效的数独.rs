impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row=[[0u8;10];9];
        let mut col=[[0u8;10];9];
        let mut myBox=[[0u8;10];9];
        let mut curNumber=0usize;

        //数组定义完毕
        for i in 0..9{
            for j in 0..9{
                if(board[i][j]=='.'){continue};
                curNumber=board[i][j].to_string().parse::<usize>().unwrap();
                if(row[i][curNumber]!=0)
                {
                    return false;
                } 
                if(col[j][curNumber]!=0) {
                    return false;
                }
                if(myBox[j/3 + (i/3)*3][curNumber]!=0){
                     return false;
                }
                row[i][curNumber] = 1;
                col[j][curNumber] = 1;
                myBox[j/3 + (i/3)*3][curNumber] = 1;
            }
        }
        return true;
    }
}
