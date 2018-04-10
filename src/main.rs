// machine learning for unsupervised kmeans
use std::collections::BTreeMap;

struct kmeans {
    k: usize,
    sse: Vec<i32>,
    center: BTreeMap<Vec<i32>, >,

}

impl kmeans {
    fn compute_means(&self, &data: &Vec<i32>) -> i32 {
        let mut sum: i32 = 0;
        for i in data.iter() {
            sum += i.pow(2);
        }
        sum
    }

    fn compute_sse(&self, sse: i32, data: &Vec<i32>) -> i32 {
        sse
    }

    fn compute_centroid(data: &Vec<i32>) -> Vec<i32> {}

    fn converge(sse: i32) -> Result {}
}


fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7];
    print!("The means is {}\n", compute_means(&data));
}
