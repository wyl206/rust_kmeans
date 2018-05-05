extern crate rand;

use std::collections::BTreeMap;
use rand::{thread_rng, Rng};


/// Kmeans类别要求
pub struct Kmeans<'a> {
    pub k: u8,                                  // k means
    pub sse: u32,                               // sse
    pub center: &'a mut BTreeMap<u8, Vec<f32>>, // k center
    pub data: &'a mut BTreeMap<u32, Vec<f32>>,  // n data
    pub datanum: u32,                           // number of data
    pub datadim: u8,                            //dimension of data
    pub class: &'a mut Vec<u8>,                 // the k class of n data
}

impl<'a> Kmeans<'a> {
    pub fn new(&mut self, def_k: u8, def_dim: u8) {
        self.k = def_k; // defalut 2 kmeans
        self.datanum = 0;
        self.sse = 1;
        self.datadim = def_dim;
    }
    pub fn input_data(&mut self, onedata: &Vec<f32>) {
        //初始化data输入
        self.data.entry(self.datanum).or_insert(onedata.to_vec());
        self.datanum += 1;
        self.class.push(0);
    }

    //初始化类别以及中心轴
    pub fn init(&mut self) {
        let mut rng = thread_rng();
        for i in 0..self.k {
            let center_loc = rng.gen_range(0, self.datanum); // 随机产生每个类的初始中心轴的序号
            let temp_center: Vec<f32> = self.data.get(&center_loc).unwrap().to_vec(); //找出序号对应的数据
            self.center.entry(i).or_insert(temp_center); //放到对应的类中
        }
    }

    pub fn compute_sse(&self) -> f32 {
        // 距离metric计算函数
        let mut sum: f32 = 0.0;
        for i in 0..self.datanum {
            let class_i = self.class[i as usize];
            let data_i = self.data.get(&i).unwrap();
            sum += self.compute_distance(data_i, self.center.get(&class_i).unwrap());
        }
        sum
    }

    fn compute_distance(&self, v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
        //计算两个数据之间的距离
        let mut sum: f32 = 0.0;
        for i in 0..self.datadim {
            let j = i as usize;
            sum += (v1[j] - v2[j]).powf(2.0);
        }
        sum
    }

    pub fn determine_class(&mut self) {
        // 决定每个样本属于哪一类
        for i in 0..self.datanum {
            let mut min_distance: f32 =
                self.compute_distance(self.data.get(&i).unwrap(), self.center.get(&0).unwrap());
            self.class[i as usize] = 0;
            for j in 1..self.k {
                let temp_distance =
                    self.compute_distance(self.data.get(&i).unwrap(), self.center.get(&j).unwrap());
                if temp_distance < min_distance {
                    min_distance = temp_distance;
                    self.class[i as usize] = j; // 如果到第j个轴最短，则是属于第j个类
                }
            }
        }
    }

    fn add_data(&self, v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
        // 对两个数据进行相加
        let mut v = Vec::new();
        for i in 0..self.datadim {
            let j: usize = i as usize;
            v.push(v1[j] + v2[j]);
        }
        v
    }

    fn divide_num(&self, v1: &Vec<f32>, total: i32) -> Vec<f32> {
        let mut v = Vec::new();
        for i in v1 {
            v.push(*i / total as f32);
        }
        v
    }

    pub fn compute_centroid(&mut self) {
        // 计算每个类的中心轴
        let mut classnum = Vec::with_capacity(self.k as usize); // 每个类里的元素，总共k个类
        for i in 0..self.k {
            self.center.insert(i, vec![0.0; self.datadim as usize]);
            classnum.push(0);
        }
        for i in 0..self.datanum {
            let k = self.class[i as usize]; // 找到是第几类
            let v = self.add_data(self.center.get(&k).unwrap(), self.data.get(&i).unwrap()); // 把数据加起来
            self.center.insert(k, v);
            classnum[k as usize] += 1; // 计算类的元素个数+1
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