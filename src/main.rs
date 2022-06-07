use ndarray::prelude::*;
use ndarray_rand::rand;
use ndarray_rand::rand_distr::{StandardNormal, Uniform};
use ndarray_rand::{RandomExt, SamplingStrategy};
use ndarray_stats::histogram::{strategies::Sqrt, GridBuilder};
use ndarray_stats::HistogramExt;
use noisy_float::types::{n64, N64};
use rand::seq::IteratorRandom;

fn main() {
    //Using ndarray crate
    let arr1 = array![1., 2., 3., 4., 5., 6.];
    println!("1D array: \t{}", arr1);

    //using array std::
    let ls1 = [1., 2., 3., 4., 5., 6.];
    println!("1D list: \t{:?}", ls1);

    //using vec!
    let vec1 = vec![1., 2., 3., 4., 5., 6.];
    println!("1D vector \t{:?}", vec1);

    // adding with ndarray crate
    let arr2 = array![1., 2.2, 3.3, 4., 5., 6.];
    let arr3 = arr1 + arr2;
    println!("1D array: \t{}", arr3);

    // adding with std lib array
    let ls2 = [1., 2.2, 3.3, 4., 5., 6.];
    let mut ls3 = ls1.clone();
    for i in 1..ls2.len() {
        ls3[i] = ls1[i] + ls2[i];
    }
    println!("1D list: \t{:?}", ls3);

    //adding using vec --- iter() -- is important
    let vec2 = vec![1., 2.2, 3.3, 4., 5., 6.];
    let vec3: Vec<f64> = vec1
        .iter()
        .zip(vec2.iter())
        .map(|(&e1, &e2)| e1 + e2)
        .collect();
    println!("1D vec: \t{:?}", vec3);

    // 2D arrays using ndarray crate
    let arr4 = array![[1., 2., 3.], [4., 5., 6.]];
    let arr5 = Array::from_elem((2, 1), 2.);
    let arr6 = arr4 + arr5;
    println!("2D array: \n{}", arr6);

    // Fill arr6 with 0s
    let arr7 = Array::<f64, _>::zeros(arr6.raw_dim());
    let arr8 = arr6 * arr7;
    println!("\n {}", arr8);

    // identity matrix
    let identity: &Array2<f64> = &Array::eye(3);
    println!("\n identity matrix start: \n\n {}", identity);
    let arr9 = array![[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]];
    let arr10 = &arr9 * identity;
    println!("\n{}", arr10);
    let arr11 = arr9.dot(identity);
    println!("\n{}", arr11);

    //0-D array, just an element
    println!(" \n0-D array \n {}", array![2.]);
    println!("Dimensions: {}", array![2.].ndim());

    //3D or more array
    let arr12 = Array::<i8, _>::ones((2, 3, 2, 2));
    println!("\nMULTIDIMENSIONAL\n{}", arr12);

    //RANDOM START
    println!("!!! \n HERE STARTS RANDOM \n !!!");

    // Array of shape (5,2) filled with a uniform dist between 1 and 10
    let arr13 = Array::random((2, 5), Uniform::new(0., 10.));
    println!("{:5.2}", arr13);

    //"pick" data from an array, sampling
    let arr14 = array![1., 2., 3., 4., 5., 6.];
    let arr15 = arr14.sample_axis(Axis(0), 2, SamplingStrategy::WithoutReplacement);
    println!("\n Sampling from: \t{} \nTwo elements: \t{}", arr14, arr15);

    // same as above, basically
    let mut rng = rand::thread_rng();
    let faces = "😀😎😐😕😠😢";
    let arr16 = Array::from_shape_vec((2, 2), faces.chars().choose_multiple(&mut rng, 4)).unwrap();
    println!("\n Sampling from:\t{}", faces);
    println!("Elements: \n {}", arr16);

    // //standard normaldistrubisjon
    // let arr17 = Array::<f64, _>::random_using((10000, 2), StandardNormal, &mut rand::thread_rng());
    // let data = arr17.mapv(|e| n64(e));
    // //create grid for histogram
    // let grid = GridBuilder::<Sqrt<N64>>::from_array(&data).unwrap().build();
    // let histogram = data.histogram(grid);
    // let histogram_matrix = histogram.counts();

    // let data = histogram_matrix.sum_axis(Axis(0));
    // let his_data: Vec<(f32, f32)> = data
    //     .iter()
    //     .enumerate()
    //     .map(|(e, i)| (e as f32, *i as f32))
    //     .collect();

    // // plot histogram!!
    // let file = std::fs::File::create("standard_normal_hist.svg").unwrap();
    // let mut graph = simple_fmt!(his_data, "Histogram", "x", "y");
    // graph
    //     .histogram("Stand.Norm.Dist.", his_data)
    //     .xmarker(0)
    //     .ymarker(0);

    // graph.simple_theme(poloto::upgrade_write(file));
}
