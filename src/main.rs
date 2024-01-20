
use data_lib::data_range;

fn random() -> f64 {
    use std::arch::x86_64::_rdrand64_step;
    let mut num = 0u64;
    if(unsafe { _rdrand64_step(&mut num) } == 1) {
        num as f64 / u64::MAX as f64
    } else {
        panic!("failed to generate random number");
    }
}

trait Constructor {
    fn new() -> Self;
}

pub enum LayerType {
    Input ,
    Hidden,
    Output
}

mod builder_traits {
    use crate::LayerType;

    pub trait NnBuilder {
        #[must_use]
        fn new() -> Self;

        fn add_layer(self, num_neurons: usize, layers: LayerType) -> Self;
        
        fn build(self) -> crate::NeuralNetwork;
    }

}

mod builders {
    use crate::{NeuralNetwork, LayerType, Layer, Constructor, Neuron};
    use crate::builder_traits;
    use crate::random;

    pub struct NnBuilder { nn: NeuralNetwork }

    impl builder_traits::NnBuilder for NnBuilder {

        fn new() -> Self { NnBuilder { nn: NeuralNetwork::new() } }

        fn add_layer(mut self, num_neurons: usize, layer_type: LayerType) -> Self {
            fn new_layer(num_neurons: usize) -> Layer {
                let mut layer = Layer::new();
                for _ in 0..num_neurons {
                    let mut n = Neuron::new();
                    n.weight = random() * (2.0 - 1.0);
                    n.bias   = random() * (2.0 - 1.0);
                    layer.neurons.push(n);
                }
                layer
            }

            match layer_type {
                LayerType::Input  => {
                    self.nn.input_layer.push(new_layer(num_neurons));
                },
                LayerType::Hidden => {
                    self.nn.hidden_layer.push(new_layer(num_neurons));
                },
                LayerType::Output => {
                    self.nn.output_layer.push(new_layer(num_neurons));
                },
            }
            self
        }

        fn build(self) -> NeuralNetwork { self.nn }
    }

    pub fn new() -> NnBuilder {
        use builder_traits::NnBuilder as _;
        NnBuilder::new()
    }
}

#[derive(Debug, Clone)]
pub struct Neuron {
    weight: f64,
    bias: f64,
}

impl Constructor for Neuron {
    fn new() -> Self {
        Neuron {
            weight: 0.0,
            bias: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Layer {
    neurons: Vec<Neuron>,
}

impl Constructor for Layer {
    fn new() -> Self {
        Layer { neurons: Vec::new() }
    }
}

#[derive(Debug)]
pub struct NeuralNetwork {
    input_layer : Vec<Layer>,
    hidden_layer: Vec<Layer>,
    output_layer: Vec<Layer>
}

impl Constructor for NeuralNetwork {
    fn new() -> Self {
        NeuralNetwork {
            input_layer : Vec::new(),
            hidden_layer: Vec::new(),
            output_layer: Vec::new()
        }
    }
}

impl NeuralNetwork {
    // train the ai on given input data
    fn train(&mut self, input: Vec<f64>) {
        // feed forward
        let output = self.feed_forward(input.clone());

        // back propagation
        let mut error = Vec::new();
        input.iter().zip(output.iter()).for_each(|(i, o)| {
            error.push(i - o);
        });

        // adjust weights and biases
        self.input_layer.iter_mut().zip(error.iter()).for_each(|(layer, error)| {
            layer.neurons.iter_mut().for_each(|neuron| {
                neuron.weight += error;
                neuron.bias   += error;
            });
        });

        self.hidden_layer.iter_mut().zip(error.iter()).for_each(|(layer, error)| {
            layer.neurons.iter_mut().for_each(|neuron| {
                neuron.weight += error;
                neuron.bias   += error;
            });
        });

        self.output_layer.iter_mut().zip(error.iter()).for_each(|(layer, error)| {
            layer.neurons.iter_mut().for_each(|neuron| {
                neuron.weight += error;
                neuron.bias   += error;
            });
        });
    }

    // predict the output given input data
    fn predict(&self, input: Vec<f64>) -> Vec<f64> {
        self.feed_forward(input)
    }

    // feed forward
    fn feed_forward(&self, input: Vec<f64>) -> Vec<f64> {
        let mut output = Vec::new();

        // input layer
        self.input_layer.iter().zip(input.iter()).for_each(|(layer, input)| {
            layer.neurons.iter().for_each(|neuron| {
                output.push(neuron.weight * input + neuron.bias);
            });
        });

        // hidden layer
        self.hidden_layer.iter().zip(input.iter()).for_each(|(layer, input)| {
            layer.neurons.iter().for_each(|neuron| {
                output.push(neuron.weight * input + neuron.bias);
            });
        });

        // output layer
        self.output_layer.iter().zip(input.iter()).for_each(|(layer, input)| {
            layer.neurons.iter().for_each(|neuron| {
                output.push(neuron.weight * input + neuron.bias);
            });
        });

        output
    }
}

// hehe... this is the data the ai will be trained on
// its made using a proc macro...
// just because I can! XD
const DATA: &[[i32; 2]] = data_range!( 1..100 );

fn main() {
    let mut w = 0.0;
    // ai algorithm that learns to multiply by 2
    // algorithm: f(x) = x * x
    // learn
    (0..5).into_iter().for_each(|_| {
        for data in DATA.iter() {
            let input = vec![data[0] as f64, data[1] as f64];
            let prediction = (w * input[0]) + (w * input[1]);
            // add the derivative of the loss function to the weight
            w += 0.01 * (input[1] - prediction);
        }
    });
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
    println!("Actual  : 800 * 2 = {}", (w * 800.0).round());
    println!("See how it failed? The ai is only trained on certain data, so when it sees new data it cannot CORRECTLY predict the output.");
}
