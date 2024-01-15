use derive_new::new;
use encase::ShaderType;

use crate::{
    gpu::{
        BindGroupLayoutDescriptor, BindGroupLayoutHandle, ComputePipelineDescriptor,
        ComputePipelineHandle, CpuUniform, KernelElement, PipelineLayoutDescriptor, WgpuDevice,
        WorkgroupCount,
    },
    rvec, wgc, CompiledOp, OpMetadata, Operation, OperationError, RVec, Tensor,
};

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, new)]
pub struct Binary {
    lhs: Tensor,
    rhs: Tensor,
    op: BinaryOp,
}

impl Binary {
    pub fn op(&self) -> &BinaryOp {
        &self.op
    }
}

#[derive(Debug, ShaderType)]
pub struct BinaryMeta {
    M: u32,
    N: u32,
}

impl OpMetadata for BinaryMeta {}

impl Operation for Binary {
    type Meta = BinaryMeta;

    fn name(&self) -> &'static str {
        "Binary"
    }

    fn srcs(&self) -> RVec<&Tensor> {
        rvec![&self.lhs, &self.rhs]
    }

    fn storage_layout(&self, device: &WgpuDevice) -> BindGroupLayoutHandle {
        device
            .get_or_create_bind_group_layout(&BindGroupLayoutDescriptor::binary())
            .unwrap()
    }

    fn compile(
        &self,
        device: &WgpuDevice,
        uniform: &CpuUniform,
    ) -> Result<(ComputePipelineHandle, WorkgroupCount), OperationError> {
        let (lhs, rhs) = (&self.lhs, &self.rhs);
        let wgcx = WorkgroupCount::div_ceil(lhs.shape()[0], 64);

        let storage_layout = self.storage_layout(device);
        let pipeline_layout = device
            .get_or_create_pipeline_layout(&PipelineLayoutDescriptor {
                entries: rvec![storage_layout],
            })
            .unwrap();
        let pipeline = device
            .get_or_create_compute_pipeline(&ComputePipelineDescriptor {
                pipeline_layout,
                kernel_key: "add",
                elem: KernelElement::Scalar,
            })
            .unwrap();
        Ok((pipeline, wgc![wgcx as _, 1, 1]))
    }
}