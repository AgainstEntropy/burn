// Orginally copied from the burn/examples/mnist package

use burn::{
    config::Config,
    module::Module,
    nn,
    tensor::{backend::Backend, Tensor},
};

#[derive(Module, Debug)]
pub struct ConvBlock<B: Backend> {
    conv: nn::conv::Conv2d<B>,
    pool: nn::pool::MaxPool2d,
    activation: nn::GELU,
}

#[derive(Config)]
pub struct ConvBlockConfig {
    channels: [usize; 2],
    #[config(default = "[3, 3]")]
    kernel_size: [usize; 2],
}

impl<B: Backend> ConvBlock<B> {
    pub fn new(config: &ConvBlockConfig) -> Self {
        let conv = nn::conv::Conv2dConfig::new(config.channels, config.kernel_size)
            .with_padding(nn::conv::Conv2dPaddingConfig::Same)
            .init();
        let pool = nn::pool::MaxPool2dConfig::new(config.channels[1], config.kernel_size)
            .with_padding(nn::conv::Conv2dPaddingConfig::Same)
            .init();
        let activation = nn::GELU::new();

        Self {
            conv,
            pool,
            activation,
        }
    }

    pub fn forward(&self, input: Tensor<B, 4>) -> Tensor<B, 4> {
        let x = self.conv.forward(input.clone());
        let x = self.pool.forward(x);
        let x = self.activation.forward(x);

        (x + input) / 2.0
    }
}
