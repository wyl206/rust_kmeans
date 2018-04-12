// machine learning for unsupervised kmeans
extern crate rand;

use std::collections::BTreeMap;
use rand::{Rng, thread_rng};

struct Kmeans {
    k: u8,  // k means
    sse: u32,  // sse 
    center: BTreeMap<u8, Vec<i32>>, // k center
    data: BTreeMap<u32, Vec<i32>>,  // n data
    datanum: u32, // number of data
    class: Vec<u8>,  // the k class of n data 
}

impl Kmeans {
    fn New(&mut self, def_k:u8) {
        self.k = def_k; // defalut 2 kmeans
        self.class = Vec::with_capacity(def_k as usize);
        self.datanum = 0; 
        self.sse = 1;
    }
    fn input_data(&mut self, onedata:&Vec<i32>) { //初始化data输入
        self.data.entry(self.datanum).or_insert(*onedata);
        self.datanum += 1;
    }

    //初始化类别以及中心轴
    fn init(&self) {   
        let mut rng = thread_rng();
        let mut center_loc:u32 = 0;
        for i in 0..self.k {
            self.class.push(0);
            center_loc = rng.gen()*self.datanum as u32; // 随机产生每个类的初始中心轴的序号
            let Some(temp_center) = self.data.get(&center_loc); //找出序号对应的数据
            self.center.entry(i).or_insert(*temp_center); //放置到对应的类中
        }
    }


    fn compute_means(&self) -> i32 {  // 距离metric计算函数
        let mut sum: i32 = 0;
        for i in self.data.iter() {
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
