extern crate km;

use std::collections::BTreeMap;
use km::Kmeans;

fn main() {
    // 先定义几个向量
    let k = 2;
    let datanum = 4;
    let datadim = 2;
    let mut center = BTreeMap::new();
    let mut data = BTreeMap::new();
    let mut class = Vec::with_capacity(datanum as usize);
    let mut k_sk = Kmeans {
        k: k,
        sse: 0,
        center: &mut center,
        data: &mut data,
        datanum: datanum,
        datadim: datadim,
        class: &mut class,
    };
    k_sk.new(k, datadim);
    // 输入数据
    k_sk.input_data(&vec![1.0, 1.0]);
    k_sk.input_data(&vec![1.0, 2.0]);
    k_sk.input_data(&vec![2.0, 1.0]);
    k_sk.input_data(&vec![2.0, 2.0]);
    //输入数据
    k_sk.init();
    let mut precise: f32 = 1.0;
    let mut loops = 0;
    let mut lastsse = 1.0;
    while (precise > 1e-4) || (loops < 10000) {
        // 循环结束条件
        k_sk.determine_class(); // 计算每个点属于类别
        let sse = k_sk.compute_sse(); //计算sse
        precise = (1.0 - sse as f32 / lastsse as f32).abs();
        loops += 1;
        if !(sse == 0.0) {
            lastsse = sse; //替换sse
        } else {
            lastsse = 1.0;
        }
        k_sk.compute_centroid(); //计算中心轴
        println!("precise: {:}", precise);
        println!("sse {:}", sse);
    }
    println!("{:?}", k_sk.data);
    println!("{:?} \n {:?}", k_sk.class, k_sk.center);
}
