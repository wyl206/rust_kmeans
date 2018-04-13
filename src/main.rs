// machine learning for unsupervised kmeans
extern crate rand;

use std::collections::BTreeMap;
use rand::{thread_rng, Rng};

struct Kmeans {
    k: u8,                          // k means
    sse: u32,                       // sse
    center: BTreeMap<u8, Vec<i32>>, // k center
    data: BTreeMap<u32, Vec<i32>>,  // n data
    datanum: u32,                   // number of data
    datadim: u8,                    //dimension of data
    class: Vec<u8>,                 // the k class of n data
}

impl Kmeans {
    fn new(&mut self, def_k: u8, def_dim: u8) {
        self.k = def_k; // defalut 2 kmeans
        self.class = Vec::with_capacity(def_k as usize);
        self.datanum = 0;
        self.sse = 1;
        self.datadim = def_dim;
    }
    fn input_data(&mut self, onedata: &Vec<i32>) {
        //初始化data输入
        self.data.entry(self.datanum).or_insert(onedata.to_vec());
        self.datanum += 1;
    }

    //初始化类别以及中心轴
    fn init(&mut self) {
        let mut rng = thread_rng();
        let mut center_loc: u32 = 0;
        for i in 0..self.k {
            self.class.push(0);
            center_loc = rng.gen_range(0, self.datanum); // 随机产生每个类的初始中心轴的序号
            let temp_center: Vec<i32> = self.data.get(&center_loc).unwrap().to_vec(); //找出序号对应的数据
            self.center.entry(i).or_insert(temp_center); //放到对应的类中
        }
    }

    fn compute_sse(&self) -> u32 {
        // 距离metric计算函数
        let mut sum: u32 = 0;
        for i in 0..self.datanum {
            let class_i = self.class[i as usize];
            let data_i = self.data.get(&i).unwrap();
            sum += self.compute_distance(data_i, self.center.get(&class_i).unwrap());
        }
        sum
    }

    fn compute_distance(&self, v1: &Vec<i32>, v2: &Vec<i32>) -> u32 {
        //计算两个数据之间的距离
        let mut sum: i32 = 0;
        for i in 0..self.datadim {
            let j = i as usize;
            sum += (v1[j] - v2[j]).pow(2);
        }
        sum as u32
    }

    fn determine_class(&mut self) {
        // 决定每个样本属于哪一类
        let mut min_distance: u32 =
            self.compute_distance(self.data.get(&0).unwrap(), self.center.get(&0).unwrap());
        self.class[0] = 0;
        for i in 0..self.datanum {
            for j in 0..self.k {
                let temp_distance =
                    self.compute_distance(self.data.get(&0).unwrap(), self.center.get(&0).unwrap());
                if temp_distance < min_distance {
                    min_distance = temp_distance;
                    self.class[i as usize] = j; // 如果到第j个轴最短，则是属于第j个类
                }
            }
        }
    }

    fn add_data(&self, v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
        // 对两个数据进行相加
        let mut v = Vec::new();
        for i in 0..self.datadim {
            let j: usize = i as usize;
            v[j] = v1[j] + v2[j];
        }
        v
    }

    fn divide_num(&self, v1: &Vec<i32>, total: i32) -> Vec<i32> {
        let mut v = Vec::new();
        let mut u: usize = 0;
        for i in v1 {
            v[u] = *i / total;
            u += 1;
        }
        v
    }

    fn compute_centroid(&mut self) {
        // 计算每个类的中心轴
        let mut classnum = Vec::new(); // 每个类里的元素，总共k个类
        self.center.clear();
        for i in 0..self.datanum {
            for j in 0..self.datadim {
                let k = self.class[i as usize]; // 找到是第几类
                let v = self.add_data(self.center.get(&k).unwrap(), self.data.get(&i).unwrap()); // 把数据加起来
                self.center.insert(k, v);
                classnum[self.class[i as usize] as usize] += 1; // 计算类的元素个数+1
            }
        }
        for i in 0..self.k {
            //把平均值算出来
            let total = classnum.get(i as usize).unwrap();
            if *total != 0 {
                let v = self.divide_num(self.center.get(&i).unwrap(), *total);
                self.center.insert(i, v);
            }
        }
    }
}

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7];
}
