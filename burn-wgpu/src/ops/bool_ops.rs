use std::ops::Range;

use burn_tensor::{ops::BoolTensorOps, ops::IntTensorOps, Data, Shape};

use crate::{
    element::{FloatElement, IntElement},
    tensor::WgpuTensor,
    GraphicsApi, WgpuBackend,
};

use super::{BaseOps, BoolTensor, Device, IntTensor};

impl<G, F, I> BoolTensorOps<WgpuBackend<G, F, I>> for WgpuBackend<G, F, I>
where
    G: GraphicsApi + 'static,
    F: FloatElement,
    I: IntElement,
{
    fn bool_empty<const D: usize>(shape: Shape<D>, device: &Device<Self>) -> BoolTensor<Self, D> {
        BaseOps::<G>::empty(shape, device)
    }

    fn bool_shape<const D: usize>(tensor: &BoolTensor<Self, D>) -> Shape<D> {
        tensor.shape.clone()
    }

    fn bool_into_data<const D: usize>(tensor: BoolTensor<Self, D>) -> Data<bool, D> {
        let data = BaseOps::<G>::into_data(tensor);

        Data::new(data.value.into_iter().map(|i| i != 0).collect(), data.shape)
    }

    fn bool_from_data<const D: usize>(
        data: Data<bool, D>,
        device: &Device<Self>,
    ) -> BoolTensor<Self, D> {
        let data: Data<u32, D> = Data::new(
            data.value
                .into_iter()
                .map(|c| match c {
                    true => 1,
                    false => 0,
                })
                .collect(),
            data.shape,
        );
        BaseOps::<G>::from_data(data, device)
    }

    fn bool_into_int<const D: usize>(tensor: BoolTensor<Self, D>) -> IntTensor<Self, D> {
        if std::mem::size_of::<I>() == std::mem::size_of::<u32>() {
            return WgpuTensor::new(tensor.context, tensor.shape, tensor.buffer);
        }

        let device = Self::bool_device(&tensor);
        let data = Self::bool_into_data(tensor).convert::<I>();

        Self::int_from_data(data, &device)
    }

    fn bool_device<const D: usize>(tensor: &BoolTensor<Self, D>) -> Device<Self> {
        tensor.context.device.clone()
    }

    fn bool_to_device<const D: usize>(
        tensor: BoolTensor<Self, D>,
        device: &Device<Self>,
    ) -> BoolTensor<Self, D> {
        BaseOps::<G>::to_device(tensor, device)
    }

    fn bool_reshape<const D1: usize, const D2: usize>(
        tensor: BoolTensor<Self, D1>,
        shape: Shape<D2>,
    ) -> BoolTensor<Self, D2> {
        BaseOps::<G>::reshape(tensor, shape)
    }

    fn bool_index<const D1: usize, const D2: usize>(
        tensor: BoolTensor<Self, D1>,
        indexes: [Range<usize>; D2],
    ) -> BoolTensor<Self, D1> {
        BaseOps::<G>::index(tensor, indexes)
    }

    fn bool_index_assign<const D1: usize, const D2: usize>(
        tensor: BoolTensor<Self, D1>,
        indexes: [Range<usize>; D2],
        value: BoolTensor<Self, D1>,
    ) -> BoolTensor<Self, D1> {
        BaseOps::<G>::index_assign(tensor, indexes, value)
    }

    fn bool_cat<const D: usize>(
        tensors: Vec<BoolTensor<Self, D>>,
        dim: usize,
    ) -> BoolTensor<Self, D> {
        BaseOps::<G>::cat(tensors, dim)
    }

    fn bool_equal<const D: usize>(
        lhs: BoolTensor<Self, D>,
        rhs: BoolTensor<Self, D>,
    ) -> BoolTensor<Self, D> {
        BaseOps::<G>::equal(lhs, rhs)
    }

    fn bool_equal_elem<const D: usize>(lhs: BoolTensor<Self, D>, rhs: bool) -> BoolTensor<Self, D> {
        BaseOps::<G>::equal_elem(
            lhs,
            match rhs {
                true => 1,
                false => 0,
            },
        )
    }
}
