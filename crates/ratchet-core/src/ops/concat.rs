use derive_new::new;
use glam::UVec4;

use crate::{
    gpu::{BindGroupLayoutDescriptor, CpuUniform, WorkgroupCount},
    wgc, KernelElement, MetaOperation, OpGuards, Operation, OperationError, RVec, Shape,
    StorageView, Strides, Tensor,
};

#[derive(new, Debug, Clone)]
pub struct Concat {
    inputs: RVec<Tensor>,
    dim: usize,
}

impl Operation for Concat {
    fn compute_view(&self) -> Result<StorageView, OperationError> {
        todo!()
    }
}

impl OpGuards for Concat {
    fn check_shapes(&self) {
        assert!(self.inputs.len() > 1);
        let first = &self.inputs[0];
        assert!(self.inputs.iter().all(|x| x.rank() == first.rank()));
        assert!(self.inputs.iter().all(|x| self.dim < x.rank()));
        //All tensors must have same shape, sans the concatenation dimension
        for axis in 0..self.dim {
            assert!(self
                .inputs
                .iter()
                .all(|x| x.shape()[axis] == first.shape()[axis]));
        }
        for axis in (self.dim + 1)..first.rank() {
            assert!(self
                .inputs
                .iter()
                .all(|x| x.shape()[axis] == first.shape()[axis]));
        }
    }

    fn check_dtypes(&self) {
        assert!(self.inputs.iter().all(|x| x.dt() == self.inputs[0].dt()));
    }
}

impl MetaOperation for Concat {
    fn srcs(&self) -> RVec<&Tensor> {
        self.inputs.iter().collect()
    }

    fn kernel_key(&self, dst: &Tensor) -> String {
        let ke = self.kernel_element(dst).as_str();
        format!("concat_{}", ke)
    }

    fn kernel_element(&self, _: &Tensor) -> KernelElement {
        KernelElement::Scalar
    }

    fn calculate_dispatch(&self, dst: &Tensor) -> Result<WorkgroupCount, OperationError> {
        let numel = dst.shape().numel();
        let x_groups = WorkgroupCount::div_ceil(numel as _, 64);
        let (x_groups, y_groups) = if x_groups > WorkgroupCount::MAX_WGS_PER_DIM {
            let y_groups = WorkgroupCount::div_ceil(x_groups, WorkgroupCount::MAX_WGS_PER_DIM);
            (WorkgroupCount::MAX_WGS_PER_DIM, y_groups)
        } else {
            (x_groups, 1)
        };
        Ok(wgc![x_groups as _, y_groups as _, 1])
    }

    fn storage_bind_group_layout(
        &self,
        _: bool,
    ) -> Result<BindGroupLayoutDescriptor, OperationError> {
        Ok(BindGroupLayoutDescriptor::nthary(self.inputs.len()))
    }

    fn write_metadata(
        &self,
        uniform: &mut CpuUniform,
        dst: &Tensor,
        _: &KernelElement,
    ) -> Result<u64, OperationError> {
        let original_rank = self.inputs[0].rank();
        let promotion = 4 - original_rank;
        let input_shapes: Vec<Shape> = self
            .inputs
            .iter()
            .map(|x| Shape::promote(x.shape().clone(), 4))
            .collect();
        let input_strides: Vec<Strides> = input_shapes.iter().map(|x| Strides::from(x)).collect();
        let promoted_dim = self.dim + promotion;
        let dst_shape = Shape::promote(dst.shape().clone(), 4);
        let dst_strides = Strides::from(&dst_shape);

        uniform.write_struct_member(&(promoted_dim as u32));
        uniform.write_struct_member(&UVec4::from(&dst_shape));
        uniform.write_struct_member(&UVec4::from(&dst_strides));
        uniform.write_struct_member(&(dst_shape.numel() as u32));

        for (shape, strides) in input_shapes.iter().zip(input_strides.iter()) {
            uniform.write_struct_member(&UVec4::from(shape));
            uniform.write_struct_member(&UVec4::from(strides));
            uniform.write_struct_member(&(shape.numel() as u32));
        }
        let cumsum = input_shapes
            .iter()
            .map(|s| s[promoted_dim])
            .scan(0_u32, |acc, x| {
                *acc += x as u32;
                Some(*acc)
            })
            .collect::<Vec<u32>>();
        uniform.write_struct_member(&cumsum);
        Ok(uniform.write_struct_end()?)
    }
}

#[cfg(test)]
mod tests {}
