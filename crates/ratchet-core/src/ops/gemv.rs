use encase::ShaderType;
use half::f16;
use ratchet_macros::WgslMetadata;

use crate::{
    gpu::dtype::WgslDType, rvec, Array, BindingMode, BuiltIn, CpuUniform, DType, GEMMSpec,
    InvariantError, KernelElement, KernelSource, Matmul, MatmulMeta, OperationError, Scalar,
    Tensor, Vec4, WgslFragment, WgslKernelBuilder, WgslPrimitive, WorkgroupSize,
};
use glam::IVec3;
use inline_wgsl::wgsl;
use num_traits::Zero;

#[derive(Debug, Clone)]
pub struct GEMV {
    lhs: Tensor,
    rhs: Tensor,
    bias: Option<Tensor>,
    trans_lhs: bool,
    trans_rhs: bool,
    trans_out: bool,
}

impl From<Matmul> for GEMV {
    fn from(matmul: Matmul) -> Self {
        let Matmul {
            lhs,
            rhs,
            bias,
            trans_lhs,
            trans_rhs,
            trans_out,
        } = matmul;
        GEMV {
            lhs,
            rhs,
            bias,
            trans_lhs,
            trans_rhs,
            trans_out,
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[derive(Debug, Clone, ShaderType, WgslMetadata)]
pub struct WorkgroupGEMVMeta {
    aShape: IVec3,
    aStrides: IVec3,
    bShape: IVec3,
    bStrides: IVec3,
    outShape: IVec3,
    outStrides: IVec3,
    dimAOuter: i32,
    dimBOuter: i32,
    dimInner: i32,
}

impl WorkgroupGEMVMeta {
    pub(crate) fn write_metadata(
        uniform: &mut CpuUniform,
        spec: &GEMMSpec,
    ) -> Result<u64, OperationError> {
        MatmulMeta::write_metadata(uniform, spec)
    }
}

#[allow(clippy::too_many_arguments)]
#[derive(Debug, Clone, ShaderType, WgslMetadata)]
pub struct SubgroupGEMVMeta {
    OVS: i32, //out_vec_size
    IVS: i32, //in_vec_size
}

impl SubgroupGEMVMeta {
    pub(crate) fn write_metadata(
        uniform: &mut CpuUniform,
        spec: &GEMMSpec,
    ) -> Result<u64, OperationError> {
        Ok(uniform.write(&SubgroupGEMVMeta {
            OVS: spec.dim_lhs_outer() as _,
            IVS: spec.dim_inner() as _,
        })?)
    }
}

impl GEMV {
    fn register_bindings_workgroup<P: WgslPrimitive>(
        &self,
        builder: &mut WgslKernelBuilder,
        _: bool,
    ) -> Result<(), OperationError> {
        let (A, _, bias) = (&self.lhs, &self.rhs, &self.bias);

        if A.dt().is_float() {
            let float_arr = Array::<P>::default();
            builder.register_storage("A", BindingMode::ReadOnly, float_arr);
            builder.register_storage("X", BindingMode::ReadOnly, float_arr);
            if bias.is_some() {
                builder.register_storage("bias", BindingMode::ReadOnly, float_arr);
            }
            builder.register_storage("result", BindingMode::ReadWrite, float_arr);
        } else if A.dt().is_quantized() {
            let scalar = Array::<Scalar<P::T>>::default();
            builder.register_storage("A", BindingMode::ReadOnly, Array::<Scalar<u32>>::default());
            builder.register_storage("scale", BindingMode::ReadOnly, scalar);
            builder.register_storage("X", BindingMode::ReadOnly, Array::<Vec4<P::T>>::default());
            if bias.is_some() {
                builder.register_storage("bias", BindingMode::ReadOnly, scalar);
            }
            builder.register_storage("result", BindingMode::ReadWrite, scalar);
        } else {
            return Err(InvariantError::UnsupportedDType(A.dt()).into());
        }

        builder.register_uniform();
        Ok(())
    }

    fn register_bindings_subgroup<P: WgslPrimitive>(
        &self,
        builder: &mut WgslKernelBuilder,
        _: bool,
    ) -> Result<(), OperationError> {
        let float_arr = Array::<P>::default();
        builder.register_storage("mat", BindingMode::ReadOnly, float_arr);
        builder.register_storage("inVec", BindingMode::ReadOnly, float_arr);
        builder.register_storage("outVec", BindingMode::ReadWrite, float_arr);
        builder.register_uniform();
        Ok(())
    }

    pub fn build_kernel(
        &self,
        inplace: bool,
        dst: &Tensor,
        workgroup_size: &WorkgroupSize,
        spec: GEMMSpec,
    ) -> Result<KernelSource, OperationError> {
        let kernel_element = KernelElement::Scalar;
        match (self.lhs.dt(), kernel_element) {
            (DType::F32, KernelElement::Scalar) => {
                self.render_gemv::<Scalar<f32>>(inplace, dst, workgroup_size, spec)
            }
            (DType::F16, KernelElement::Scalar) => {
                self.render_gemv::<Scalar<f16>>(inplace, dst, workgroup_size, spec)
            }
            (DType::Q8_0F(_), _) => {
                self.render_gemv::<Vec4<f32>>(inplace, dst, workgroup_size, spec)
            }
            (DType::Q8_0H(_), _) => {
                self.render_gemv::<Vec4<f16>>(inplace, dst, workgroup_size, spec)
            }
            _ => panic!("Unsupported dtype"),
        }
    }

    fn workgroup_gemv<P: WgslPrimitive>(
        &self,
        inplace: bool,
        _: &Tensor,
        workgroup_size: &WorkgroupSize,
        spec: GEMMSpec,
    ) -> Result<KernelSource, OperationError> {
        let device = self.lhs.device().try_gpu().unwrap();
        let mut kernel_builder = WgslKernelBuilder::new(
            workgroup_size.clone(),
            rvec![
                BuiltIn::GlobalInvocationId,
                BuiltIn::LocalInvocationId,
                BuiltIn::WorkgroupId,
            ],
            device.compute_features().clone(),
        );

        self.register_bindings_workgroup::<P>(&mut kernel_builder, inplace)
            .unwrap();
        let n = P::W;
        let accessor = P::render_type();
        let scalar = P::T::DT;
        let zero = P::T::zero().render();

        kernel_builder.write_metadata::<WorkgroupGEMVMeta>();
        kernel_builder.write_unpack(self.lhs.dt());

        let work_size = (workgroup_size.x * workgroup_size.y / (n as u32)).render();
        kernel_builder.write_global(wgsl! {
            var<workgroup> work: array<'accessor, 'work_size>;
        });

        let (TILE_X, _) = spec.heuristic.as_workgroup_size();
        let A_FIT = spec.lhs_shape()[1] % TILE_X == 0;

        let readA = match (A_FIT, self.lhs.dt()) {
            (true, DType::F32) | (true, DType::F16) => {
                wgsl! {
                    fn readA(batch: i32, row: i32, col: i32) -> 'scalar {
                        return A[dot(metadata.aStrides, vec3<i32>(batch, row, col))];
                    }
                }
            }
            (false, DType::F32) | (false, DType::F16) => {
                wgsl! {
                    fn readA(batch: i32, row: i32, col: i32) -> 'scalar {
                        var val = 'zero;
                        if (row <= metadata.aShape.y) {
                            val = A[dot(metadata.aStrides, vec3<i32>(batch, row, col))];
                        }
                        return val;
                    }
                }
            }
            (true, DType::Q8_0F(_)) | (true, DType::Q8_0H(_)) => {
                wgsl! {
                    fn readA(batch: i32, row: i32, col: i32) -> vec4<'scalar> {
                        return unpack(A[dot(metadata.aStrides, vec3<i32>(batch, row, col))]);
                    }
                }
            }
            _ => unimplemented!(),
        };
        kernel_builder.write_global(readA);

        let workgroup_size_y = workgroup_size.y;
        let main_loop = match self.lhs.dt() {
            DType::Q8_0F(_) | DType::Q8_0H(_) => {
                wgsl! {
                    let sIndex = (aOffset / 4) + row * metadata.aStrides.y / 32;
                    for (var k = i32(global_invocation_id.y); k < metadata.dimInner / 4; k+='workgroup_size_y / 4) {
                        sum = fma(unpack(A[aIndex + k]) * scale[sIndex + (k/8)], X[k], sum);
                    }
                }
            }
            _ => {
                wgsl! {
                    for (var k = i32(global_invocation_id.y); k < metadata.dimInner; k+='workgroup_size_y) {
                        sum = fma(readA(batchA, row, k), X[bOffset + k], sum);
                    }
                }
            }
        };

        kernel_builder.write_main(wgsl! { let row = i32(global_invocation_id.x); });

        kernel_builder.write_main(wgsl! {
            let batch = i32(global_invocation_id.z);
            let batchA = batch % metadata.aShape.x;
            let batchB = batch % metadata.bShape.x;
        });

        kernel_builder.write_main(wgsl! {
            let aOffset = metadata.aStrides.x * batchA / 'n;
            let bOffset = metadata.bStrides.x * batchB / 'n;
            let outOffset = metadata.outStrides.x * batch / 'n;
        });

        kernel_builder.write_main(wgsl! { var sum = 'accessor(0.0); });
        kernel_builder.write_main(wgsl! { let aIndex = aOffset + row * metadata.aStrides.y / 'n; });

        kernel_builder.write_main(main_loop);

        let workgroup_size_x = workgroup_size.x.render();
        let workgroup_size_y = workgroup_size.y.render();
        kernel_builder.write_main(wgsl! {
            let rows = 'workgroup_size_x;
            let cols = 'workgroup_size_y / 'n;

            let ii = u32(local_invocation_id.x);
            let jj = u32(local_invocation_id.y);
            work[ii + rows * jj] = sum;
            workgroupBarrier();

            // Reduce sums in log2(cols) steps
            for (var s = u32(cols) / 2u; s > 0u; s >>= 1u) {
                if (jj < s) {
                    work[ii + rows * jj] += work[ii + rows * (jj + s)];
                }
                workgroupBarrier();
            }
        });

        let bias = if self.bias.is_some() {
            wgsl! { bias[row] }
        } else {
            wgsl! { 0. }
        };

        let finalizer = match P::W {
            4 | 2 => wgsl! { result[outOffset + row] = dot(work[ii], 'accessor(1.0)) + 'bias; },
            1 => wgsl! { result[outOffset + row] = work[ii] + 'bias; },
            _ => unimplemented!(),
        };

        kernel_builder.write_main(wgsl! {
            if (jj == 0) {
                'finalizer
            }
        });

        Ok(kernel_builder.build()?)
    }

    fn subgroup_gemv<P: WgslPrimitive>(
        &self,
        inplace: bool,
        _: &Tensor,
        workgroup_size: &WorkgroupSize,
        spec: GEMMSpec,
    ) -> Result<KernelSource, OperationError> {
        const TM: usize = 4;
        const TN: usize = 4;
        const BM: usize = 8;
        const BN: usize = 32;

        let device = self.lhs.device().try_gpu().unwrap();
        let mut kernel_builder = WgslKernelBuilder::new(
            workgroup_size.clone(),
            rvec![
                BuiltIn::LocalInvocationIndex,
                BuiltIn::WorkgroupId,
                BuiltIn::SubgroupSize,
                BuiltIn::SubgroupInvocationId,
            ],
            device.compute_features().clone(),
        );
        kernel_builder.write_metadata::<SubgroupGEMVMeta>();

        self.register_bindings_subgroup::<P>(&mut kernel_builder, inplace)
            .unwrap();

        let work_size = BN * TN * 2;
        kernel_builder.write_global(wgsl! {
            var<workgroup> tgpMemory: array<f32, 'work_size>;
        });

        kernel_builder.write_main(wgsl! {
            let simd_gid = local_invocation_index / subgroup_size;
            let simd_lid = subgroup_invocation_id;


            let matBatchOffset = i32(workgroup_id.z) * metadata.OVS * metadata.IVS;
            let inVecBatchOffset = i32(workgroup_id.z) * metadata.IVS;
            let outVecBatchOffset = i32(workgroup_id.z) * metadata.OVS;


            // Threadgroup in_vec cache
            let inVecBlockOffset = i32(simd_lid * 'TN * 2);

            // Thread local accumulation results
            var result: array<f32, 'TM>;
            var inter: array<f32, 'TN>;
            var vCoeff: array<f32, 'TN>;

            // Block position
            var outRow = i32((workgroup_id.x * 'BM + simd_gid) * 'TM);

            // Exit simdgroup if rows out of bound
            if (outRow >= metadata.OVS) {
                return;
            }

            // Adjust tail simdgroup to ensure in bound reads
            outRow = select(metadata.OVS - 'TM, outRow, outRow + 'TM <= metadata.OVS);

            // Advance matrix
            let matOffset = matBatchOffset + outRow * metadata.IVS;
        });

        let main_tgp_load = (0..TN)
            .map(|tn| {
                wgsl! {
                    tgpMemory[inVecBlockOffset + 'tn] = inVec[inVecBatchOffset + bn + 'tn];
                }
                .into()
            })
            .collect::<WgslFragment>();

        let edge_tgp_load = (0..TN)
            .map(|tn| {
                wgsl! {
                    tgpMemory[inVecBlockOffset + 'tn] = select(inVec[inVecBatchOffset + bn + 'tn], 0.0, bn + 'tn < metadata.IVS);
                }
                .into()
            })
            .collect::<WgslFragment>();

        let load_rows = (0..TN)
            .map(|tn| {
                wgsl! {
                    vCoeff['tn] = tgpMemory[inVecBlockOffset + 'tn];
                }
                .into()
            })
            .collect::<WgslFragment>();

        let main_inter_load = (0..TN)
            .map(|tn| {
                wgsl! {
                    inter['tn] = mat[matOffset + tm * metadata.IVS + bn + 'tn];
                }
                .into()
            })
            .collect::<WgslFragment>();

        let edge_inter_load = (0..TN)
            .map(|tn| {
                wgsl! {
                    inter['tn] = mat[matOffset + tm * metadata.IVS + select(metadata.IVS - 1, bn + 'tn, bn + 'tn < metadata.IVS)];
                }
                .into()
            })
            .collect::<WgslFragment>();

        let accumulate = (0..TN)
            .map(|tn| {
                wgsl! {
                    result[tm] = fma(inter['tn], vCoeff['tn], result[tm]);
                }
                .into()
            })
            .collect::<WgslFragment>();

        let finalizer = (0..TM)
            .map(|tm| {
                wgsl! {
                    outVec[outVecBatchOffset + outRow + 'tm] = result['tm];
                }
                .into()
            })
            .collect::<WgslFragment>();

        let BNTN = BN * TN;
        kernel_builder.write_main(wgsl! {
            // Loop over in_vec in blocks of SIMD_SIZE * {{TN}}
            for (var bn = i32(simd_lid * 'TN); bn < i32(metadata.IVS); bn += 'BNTN) {
                workgroupBarrier();

                // Prefetch in_vector for threadgroup use
                if (simd_gid == 0u) {
                    // Main load loop
                    if (bn + 'TN <= i32(metadata.IVS)) {
                        'main_tgp_load
                    } else { // Edgecase
                        'edge_tgp_load
                    }
                }

                workgroupBarrier();

                // Load for all rows
                'load_rows

                // Per thread work loop
                for (var tm = 0; tm < 'TM; tm++) {
                    // Load for the row
                    if (bn + 'TN <= metadata.IVS) {
                        'main_inter_load
                    } else { // Edgecase
                        'edge_inter_load
                    }

                    // Accumulate results
                    'accumulate
                }
            }

            for (var tm = 0; tm < 'TM; tm++) {
                result[tm] = subgroupAdd(result[tm]);
            }

            // Write outputs
            if (simd_lid == 0u) {
                'finalizer
            }
        });

        let x = kernel_builder.build()?;
        println!("{}", x);
        Ok(x)
    }

    fn render_gemv<P: WgslPrimitive>(
        &self,
        inplace: bool,
        _: &Tensor,
        workgroup_size: &WorkgroupSize,
        spec: GEMMSpec,
    ) -> Result<KernelSource, OperationError> {
        let device = self.lhs.device().try_gpu().unwrap();
        if device.compute_features().SUBGROUP {
            println!("Using subgroup gemv");
            self.subgroup_gemv::<P>(inplace, &self.lhs, workgroup_size, spec)
        } else {
            println!("Using workgroup gemv");
            self.workgroup_gemv::<P>(inplace, &self.lhs, workgroup_size, spec)
        }
    }
}
