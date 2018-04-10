// machine learning for unsupervised kmeans
use std::collections::BTreeMap;

struct Kmeans {
    k: u8,  // k means
    mut sse: u32,  // sse 
    mut center: BTreeMap<u8, Vec<i32>>, // k center
    data: BTreeMap<u32, Vec<i32>>,  // n data
    mut class: Vec<u8>,  // the k class of n data 
}

impl Kmeans {
    fn New(&self) {
        self.k = 2; // defalut 2 kmeans
        self.class = vec![0; 2]; 
    }
    fn input_data(&self, i:u32, one_row:Vec<i32>) { //初始化data输入
 
    }

    fn init(&self) {  //初始化kmeans需要的环境变量

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
