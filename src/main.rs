// machine learning for unsupervised kmeans
use std::collections::BTreeMap;

struct kmeans {
    k: u8,
    sse: Vec<i32>,
    center: BTreeMap<u8, Vec<i32>>,
    data: BTreeMap<u32, Vec<i32>>,
    class: Vec<u8>,
}

impl kmeans {
    fn compute_means(&self, da: &Vec<i32>) -> i32 {
        let mut sum: i32 = 0;
        for i in da.iter() {
            sum += i.pow(2);
        }
        sum
    }

    fn compute_sse(&self)  {
    
    }

    fn compute_centroid(&self)  {
        
    }

    fn converge(&self)  {

    }
}


fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7];
}
