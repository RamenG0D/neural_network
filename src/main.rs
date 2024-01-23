use data_lib::data_range;

// hehe... this is the data the ai will be trained on
// its made using a proc macro...
// just because I can! XD
const DATA: &[[i32; 2]] = data_range!( 1..10 );

const TRAINING_TIMES: std::ops::Range<i32> = 0..100;

fn cost(prediction: f64, actual: f64) -> f64 {
    (prediction - actual).powi(2)
}

fn main() {
    let mut w = 0.0;
    // ai algorithm that learns to multiply by 2
    // learn
    for _ in TRAINING_TIMES {
        for data in DATA.iter() {
            let prediction = (w * (data[0] as f64)) + (w * (data[1] as f64));
            let error = cost(prediction, data[1] as f64);
            let delta = error * 0.1;
            w += delta;
        }
    }

    // predict
    for data in DATA.iter() {
        let input = vec![data[0] as f64, data[1] as f64];
        let prediction = (w * input[0]) + (w * input[1]);
        println!("AI: {} * 2 = {}", input[0], prediction.round());
    }

    println!("Final weight: {}", w);

    // extra challenge for the ai
    println!("Extra challenge for the ai");
    println!("--------------------------");
    println!("Expected: 800 * 2 = 1600"  );
    println!("Actual  : 800 * 2 = {}", ((w * 800.0) + (w * 1600.0)).round());
}
