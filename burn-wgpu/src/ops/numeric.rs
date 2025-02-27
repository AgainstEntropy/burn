use crate::kernel::{
    binary_elemwise_default, binary_elemwise_inplace_default, reduction_args_dim, reduction_dim,
    reduction_sum, unary_scalar_default, unary_scalar_inplace_default, ArgsMax, ArgsMin, MeanDim,
    SumDim,
};
use crate::pool::get_context;
use crate::{
    binary_elemwise, binary_elemwise_inplace, element::WgpuElement, tensor::WgpuTensor,
    unary_scalar, unary_scalar_inplace,
};
use crate::{GraphicsApi, WgpuDevice};
use burn_tensor::{Element, ElementConversion, Shape};
use std::marker::PhantomData;

pub struct NumericOps<G: GraphicsApi> {
    _g: PhantomData<G>,
}

impl<G: GraphicsApi> NumericOps<G> {
    pub fn zeros<E: WgpuElement, const D: usize>(
        shape: Shape<D>,
        device: &WgpuDevice,
    ) -> WgpuTensor<E, D> {
        let context = get_context::<G>(device);

        let buffer = context.create_buffer(shape.num_elements() * core::mem::size_of::<E>());

        WgpuTensor::new(context, shape, buffer)
    }

    pub fn ones<E: WgpuElement + Element, const D: usize>(
        shape: Shape<D>,
        device: &WgpuDevice,
    ) -> WgpuTensor<E, D> {
        let context = get_context::<G>(device);

        let buffer = context.create_buffer(shape.num_elements() * core::mem::size_of::<E>());

        Self::add_scalar(WgpuTensor::new(context, shape, buffer), 1i32.elem::<E>())
    }

    pub fn add<E: WgpuElement, const D: usize>(
        lhs: WgpuTensor<E, D>,
        rhs: WgpuTensor<E, D>,
    ) -> WgpuTensor<E, D> {
        binary_elemwise!(Add, "+");
        binary_elemwise_inplace!(AddInplace, "+");

        if lhs.can_mut_broadcast(&rhs) {
            return binary_elemwise_inplace_default::<AddInplace, E, D>(lhs, rhs);
        }

        if rhs.can_mut_broadcast(&lhs) {
            return binary_elemwise_inplace_default::<AddInplace, E, D>(rhs, lhs);
        }

        binary_elemwise_default::<Add, E, D>(lhs, rhs)
    }

    pub fn add_scalar<E: WgpuElement, const D: usize>(
        lhs: WgpuTensor<E, D>,
        rhs: E,
    ) -> WgpuTensor<E, D> {
        unary_scalar!(AddScalar, ops "+");
        unary_scalar_inplace!(AddScalarInplace, ops "+");

        if lhs.can_mut() {
            return unary_scalar_inplace_default::<AddScalarInplace, E, D>(lhs, rhs);
        }

        unary_scalar_default::<AddScalar, E, D>(lhs, rhs)
    }

    pub fn sub<E: WgpuElement, const D: usize>(
        lhs: WgpuTensor<E, D>,
        rhs: WgpuTensor<E, D>,
    ) -> WgpuTensor<E, D> {
        binary_elemwise!(Sub, "-");
        binary_elemwise_inplace!(SubInplace, "-");

        if lhs.can_mut_broadcast(&rhs) {
            return binary_elemwise_inplace_default::<SubInplace, E, D>(lhs, rhs);
        }

        binary_elemwise_default::<Sub, E, D>(lhs, rhs)
    }

    pub fn sub_scalar<E: WgpuElement, const D: usize>(
        lhs: WgpuTensor<E, D>,
        rhs: E,
    ) -> WgpuTensor<E, D> {
        unary_scalar!(SubScalar, ops "-");
        unary_scalar_inplace!(SubScalarInplace, ops "-");

        if lhs.can_mut() {
            return unary_scalar_inplace_default::<SubScalarInplace, E, D>(lhs, rhs);
        }

        unary_scalar_default::<SubScalar, E, D>(lhs, rhs)
    }

    pub fn mul<E: WgpuElement, const D: usize>(
        lhs: WgpuTensor<E, D>,
        rhs: WgpuTensor<E, D>,
    ) -> WgpuTensor<E, D> {
        binary_elemwise!(Mul, "*");
        binary_elemwise_inplace!(MulInplace, "*");

        if lhs.can_mut_broadcast(&rhs) {
            return binary_elemwise_inplace_default::<MulInplace, E, D>(lhs, rhs);
        }

        if rhs.can_mut_broadcast(&lhs) {
            return binary_elemwise_inplace_default::<MulInplace, E, D>(rhs, lhs);
        }

        binary_elemwise_default::<Mul, E, D>(lhs, rhs)
    }

    pub fn mul_scalar<E: WgpuElement, const D: usize>(
        lhs: WgpuTensor<E, D>,
        rhs: E,
    ) -> WgpuTensor<E, D> {
        unary_scalar!(MulScalar, ops "*");
        unary_scalar_inplace!(MulScalarInplace, ops "*");

        if lhs.can_mut() {
            return unary_scalar_inplace_default::<MulScalarInplace, E, D>(lhs, rhs);
        }

        unary_scalar_default::<MulScalar, E, D>(lhs, rhs)
    }

    pub fn div<E: WgpuElement, const D: usize>(
        lhs: WgpuTensor<E, D>,
        rhs: WgpuTensor<E, D>,
    ) -> WgpuTensor<E, D> {
        binary_elemwise!(Div, "/");
        binary_elemwise_inplace!(DivInplace, "/");

        if lhs.can_mut_broadcast(&rhs) {
            return binary_elemwise_inplace_default::<DivInplace, E, D>(lhs, rhs);
        }

        binary_elemwise_default::<Div, E, D>(lhs, rhs)
    }

    pub fn div_scalar<E: WgpuElement, const D: usize>(
        lhs: WgpuTensor<E, D>,
        rhs: E,
    ) -> WgpuTensor<E, D> {
        unary_scalar!(DivScalar, ops "/");
        unary_scalar_inplace!(DivScalarInplace, ops "/");

        if lhs.can_mut() {
            return unary_scalar_inplace_default::<DivScalarInplace, E, D>(lhs, rhs);
        }

        unary_scalar_default::<DivScalar, E, D>(lhs, rhs)
    }

    pub fn sum<E: WgpuElement + Element, const D: usize>(
        tensor: WgpuTensor<E, D>,
    ) -> WgpuTensor<E, 1> {
        reduction_sum(tensor)
    }

    pub fn sum_dim<E: WgpuElement + Element, const D: usize>(
        tensor: WgpuTensor<E, D>,
        dim: usize,
    ) -> WgpuTensor<E, D> {
        reduction_dim::<SumDim, E, D>(tensor, dim)
    }

    pub fn mean<E: WgpuElement + Element, const D: usize>(
        tensor: WgpuTensor<E, D>,
    ) -> WgpuTensor<E, 1> {
        let num_elems = tensor.shape.num_elements();
        Self::div_scalar(Self::sum(tensor), (num_elems as f32).elem())
    }

    pub fn mean_dim<E: WgpuElement + Element, const D: usize>(
        tensor: WgpuTensor<E, D>,
        dim: usize,
    ) -> WgpuTensor<E, D> {
        reduction_dim::<MeanDim, E, D>(tensor, dim)
    }

    pub fn argmax<E: WgpuElement + Element, I: WgpuElement, const D: usize>(
        tensor: WgpuTensor<E, D>,
        dim: usize,
    ) -> WgpuTensor<I, D> {
        reduction_args_dim::<ArgsMax, E, I, D>(tensor, dim)
    }

    pub fn argmin<E: WgpuElement + Element, I: WgpuElement, const D: usize>(
        tensor: WgpuTensor<E, D>,
        dim: usize,
    ) -> WgpuTensor<I, D> {
        reduction_args_dim::<ArgsMin, E, I, D>(tensor, dim)
    }
}
