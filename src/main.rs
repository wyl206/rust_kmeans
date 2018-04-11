// machine learning for unsupervised kmeans
use std::collections::BTreeMap;

struct Kmeans {
    k: u8,  // k means
    datanum: u32,
    sse: u32,  // sse 
    center: BTreeMap<u8, Vec<i32>>, // k center
    data: BTreeMap<u32, Vec<i32>>,  // n data
    class: Vec<u8>,  // the k class of n data 
}

impl Kmeans {
    fn New(&mut self) {
        self.k = 2; // defalut 2 kmeans
        self.class = vec![0; 2]; 
    }
    fn input_data(&mut self, i:u32, one_row:Vec<i32>) { //初始化data输入
        self.data.entry(i).or_insert(one_row);
    }

    fn init(&mut self) {  //初始化kmeans需要的环境变量
        self.sse = 0;
        
    }


    fn compute_means(&self, da: &Vec<i32>) -> i32 {  // 距离metric计算函数
        let mut sum: i32 = 0;
        for i in da.iter() {
            sum += i.pow(2);
        }
        sum
    }

    fn determine_class(&self) {  // 决定每个样本属于哪一类

    }
    
    fn compute_centroid(&self)  { // 计算每个类的中心轴
        
    }

    fn compute_sse(&self)  {   // 计算sse 

    
    }

    fn converge(&self)  {  //判断是否收敛

    }
}


fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7];
}
