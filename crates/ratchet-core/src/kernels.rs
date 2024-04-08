// This file is generated by build.rs. Do not edit it manually.
use lazy_static::lazy_static;
use std::collections::HashMap;
lazy_static! {
    pub static ref KERNELS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(
            "sgemm_true_false_false_false_true_scalar",
            include_str!(r"../kernels/generated/sgemm_true_false_false_false_true_scalar.wgsl"),
        );
        m.insert(
            "log_inplace_vec4",
            include_str!(r"../kernels/generated/log_inplace_vec4.wgsl"),
        );
        m.insert(
            "floor_inplace_vec4",
            include_str!(r"../kernels/generated/floor_inplace_vec4.wgsl"),
        );
        m.insert(
            "sgemm_true_false_false_false_false_scalar",
            include_str!(r"../kernels/generated/sgemm_true_false_false_false_false_scalar.wgsl"),
        );
        m.insert(
            "abs_scalar",
            include_str!(r"../kernels/generated/abs_scalar.wgsl"),
        );
        m.insert(
            "tanh_inplace_vec2",
            include_str!(r"../kernels/generated/tanh_inplace_vec2.wgsl"),
        );
        m.insert(
            "ceil_inplace_vec4",
            include_str!(r"../kernels/generated/ceil_inplace_vec4.wgsl"),
        );
        m.insert(
            "concat6_scalar",
            include_str!(r"../kernels/generated/concat6_scalar.wgsl"),
        );
        m.insert(
            "relu_vec2",
            include_str!(r"../kernels/generated/relu_vec2.wgsl"),
        );
        m.insert(
            "sub_vec4",
            include_str!(r"../kernels/generated/sub_vec4.wgsl"),
        );
        m.insert(
            "gelu_inplace_vec2",
            include_str!(r"../kernels/generated/gelu_inplace_vec2.wgsl"),
        );
        m.insert(
            "-_inplace_scalar",
            include_str!(r"../kernels/generated/-_inplace_scalar.wgsl"),
        );
        m.insert(
            "gelu_inplace_scalar",
            include_str!(r"../kernels/generated/gelu_inplace_scalar.wgsl"),
        );
        m.insert(
            "sgemm_false_false_true_vec4",
            include_str!(r"../kernels/generated/sgemm_false_false_true_vec4.wgsl"),
        );
        m.insert(
            "sgemm_true_false_false_true_false_scalar",
            include_str!(r"../kernels/generated/sgemm_true_false_false_true_false_scalar.wgsl"),
        );
        m.insert(
            "tanh_vec2",
            include_str!(r"../kernels/generated/tanh_vec2.wgsl"),
        );
        m.insert(
            "sin_vec4",
            include_str!(r"../kernels/generated/sin_vec4.wgsl"),
        );
        m.insert(
            "qgemm_true_true_true_vec4",
            include_str!(r"../kernels/generated/qgemm_true_true_true_vec4.wgsl"),
        );
        m.insert(
            "-_scalar",
            include_str!(r"../kernels/generated/-_scalar.wgsl"),
        );
        m.insert(
            "sgemm_true_false_true_false_false_scalar",
            include_str!(r"../kernels/generated/sgemm_true_false_true_false_false_scalar.wgsl"),
        );
        m.insert(
            "sgemm_false_false_false_true_true_scalar",
            include_str!(r"../kernels/generated/sgemm_false_false_false_true_true_scalar.wgsl"),
        );
        m.insert(
            "mul_scalar",
            include_str!(r"../kernels/generated/mul_scalar.wgsl"),
        );
        m.insert(
            "sgemm_false_true_true_vec4",
            include_str!(r"../kernels/generated/sgemm_false_true_true_vec4.wgsl"),
        );
        m.insert(
            "cos_inplace_scalar",
            include_str!(r"../kernels/generated/cos_inplace_scalar.wgsl"),
        );
        m.insert(
            "log_vec2",
            include_str!(r"../kernels/generated/log_vec2.wgsl"),
        );
        m.insert(
            "mul_vec2",
            include_str!(r"../kernels/generated/mul_vec2.wgsl"),
        );
        m.insert(
            "gelu_vec4",
            include_str!(r"../kernels/generated/gelu_vec4.wgsl"),
        );
        m.insert(
            "tanh_inplace_scalar",
            include_str!(r"../kernels/generated/tanh_inplace_scalar.wgsl"),
        );
        m.insert(
            "-_inplace_vec4",
            include_str!(r"../kernels/generated/-_inplace_vec4.wgsl"),
        );
        m.insert(
            "sgemm_true_false_false_vec4",
            include_str!(r"../kernels/generated/sgemm_true_false_false_vec4.wgsl"),
        );
        m.insert(
            "sqrt_scalar",
            include_str!(r"../kernels/generated/sqrt_scalar.wgsl"),
        );
        m.insert(
            "exp_vec2",
            include_str!(r"../kernels/generated/exp_vec2.wgsl"),
        );
        m.insert(
            "concat3_scalar",
            include_str!(r"../kernels/generated/concat3_scalar.wgsl"),
        );
        m.insert(
            "relu_inplace_vec4",
            include_str!(r"../kernels/generated/relu_inplace_vec4.wgsl"),
        );
        m.insert(
            "cos_vec2",
            include_str!(r"../kernels/generated/cos_vec2.wgsl"),
        );
        m.insert(
            "sin_inplace_scalar",
            include_str!(r"../kernels/generated/sin_inplace_scalar.wgsl"),
        );
        m.insert(
            "qgemm_true_false_true_vec4",
            include_str!(r"../kernels/generated/qgemm_true_false_true_vec4.wgsl"),
        );
        m.insert(
            "layernorm_vec4",
            include_str!(r"../kernels/generated/layernorm_vec4.wgsl"),
        );
        m.insert(
            "sgemm_true_true_false_false_true_scalar",
            include_str!(r"../kernels/generated/sgemm_true_true_false_false_true_scalar.wgsl"),
        );
        m.insert(
            "gelu_inplace_vec4",
            include_str!(r"../kernels/generated/gelu_inplace_vec4.wgsl"),
        );
        m.insert(
            "sgemm_true_false_true_true_true_scalar",
            include_str!(r"../kernels/generated/sgemm_true_false_true_true_true_scalar.wgsl"),
        );
        m.insert(
            "sgemm_true_false_true_true_false_scalar",
            include_str!(r"../kernels/generated/sgemm_true_false_true_true_false_scalar.wgsl"),
        );
        m.insert(
            "sub_scalar",
            include_str!(r"../kernels/generated/sub_scalar.wgsl"),
        );
        m.insert(
            "neg_scalar",
            include_str!(r"../kernels/generated/neg_scalar.wgsl"),
        );
        m.insert(
            "sgemm_true_true_false_true_true_scalar",
            include_str!(r"../kernels/generated/sgemm_true_true_false_true_true_scalar.wgsl"),
        );
        m.insert(
            "sgemm_false_true_false_vec4",
            include_str!(r"../kernels/generated/sgemm_false_true_false_vec4.wgsl"),
        );
        m.insert(
            "tanh_vec4",
            include_str!(r"../kernels/generated/tanh_vec4.wgsl"),
        );
        m.insert(
            "sin_vec2",
            include_str!(r"../kernels/generated/sin_vec2.wgsl"),
        );
        m.insert(
            "ceil_scalar",
            include_str!(r"../kernels/generated/ceil_scalar.wgsl"),
        );
        m.insert(
            "sgemm_false_false_false_false_false_scalar",
            include_str!(r"../kernels/generated/sgemm_false_false_false_false_false_scalar.wgsl"),
        );
        m.insert(
            "log_inplace_vec2",
            include_str!(r"../kernels/generated/log_inplace_vec2.wgsl"),
        );
        m.insert(
            "floor_inplace_vec2",
            include_str!(r"../kernels/generated/floor_inplace_vec2.wgsl"),
        );
        m.insert(
            "sgemm_true_true_true_true_true_scalar",
            include_str!(r"../kernels/generated/sgemm_true_true_true_true_true_scalar.wgsl"),
        );
        m.insert(
            "tanh_inplace_vec4",
            include_str!(r"../kernels/generated/tanh_inplace_vec4.wgsl"),
        );
        m.insert(
            "sqrt_inplace_scalar",
            include_str!(r"../kernels/generated/sqrt_inplace_scalar.wgsl"),
        );
        m.insert(
            "sgemm_true_false_true_false_true_scalar",
            include_str!(r"../kernels/generated/sgemm_true_false_true_false_true_scalar.wgsl"),
        );
        m.insert(
            "ceil_inplace_vec2",
            include_str!(r"../kernels/generated/ceil_inplace_vec2.wgsl"),
        );
        m.insert(
            "sgemm_true_true_true_false_false_scalar",
            include_str!(r"../kernels/generated/sgemm_true_true_true_false_false_scalar.wgsl"),
        );
        m.insert(
            "sgemm_true_true_false_true_false_scalar",
            include_str!(r"../kernels/generated/sgemm_true_true_false_true_false_scalar.wgsl"),
        );
        m.insert(
            "relu_vec4",
            include_str!(r"../kernels/generated/relu_vec4.wgsl"),
        );
        m.insert(
            "sub_vec2",
            include_str!(r"../kernels/generated/sub_vec2.wgsl"),
        );
        m.insert(
            "mul_vec4",
            include_str!(r"../kernels/generated/mul_vec4.wgsl"),
        );
        m.insert(
            "gelu_vec2",
            include_str!(r"../kernels/generated/gelu_vec2.wgsl"),
        );
        m.insert(
            "ceil_inplace_scalar",
            include_str!(r"../kernels/generated/ceil_inplace_scalar.wgsl"),
        );
        m.insert(
            "-_inplace_vec2",
            include_str!(r"../kernels/generated/-_inplace_vec2.wgsl"),
        );
        m.insert(
            "exp_vec4",
            include_str!(r"../kernels/generated/exp_vec4.wgsl"),
        );
        m.insert(
            "sgemm_false_false_false_true_false_scalar",
            include_str!(r"../kernels/generated/sgemm_false_false_false_true_false_scalar.wgsl"),
        );
        m.insert(
            "relu_inplace_vec2",
            include_str!(r"../kernels/generated/relu_inplace_vec2.wgsl"),
        );
        m.insert(
            "qgemm_true_true_false_vec4",
            include_str!(r"../kernels/generated/qgemm_true_true_false_vec4.wgsl"),
        );
        m.insert(
            "cos_vec4",
            include_str!(r"../kernels/generated/cos_vec4.wgsl"),
        );
        m.insert(
            "sgemm_false_false_true_false_false_scalar",
            include_str!(r"../kernels/generated/sgemm_false_false_true_false_false_scalar.wgsl"),
        );
        m.insert(
            "sgemm_false_false_false_vec4",
            include_str!(r"../kernels/generated/sgemm_false_false_false_vec4.wgsl"),
        );
        m.insert(
            "layernorm_vec2",
            include_str!(r"../kernels/generated/layernorm_vec2.wgsl"),
        );
        m.insert(
            "broadcast_scalar",
            include_str!(r"../kernels/generated/broadcast_scalar.wgsl"),
        );
        m.insert(
            "concat5_scalar",
            include_str!(r"../kernels/generated/concat5_scalar.wgsl"),
        );
        m.insert(
            "sgemm_false_false_true_true_true_scalar",
            include_str!(r"../kernels/generated/sgemm_false_false_true_true_true_scalar.wgsl"),
        );
        m.insert(
            "tanh_scalar",
            include_str!(r"../kernels/generated/tanh_scalar.wgsl"),
        );
        m.insert(
            "sgemm_false_false_false_false_true_scalar",
            include_str!(r"../kernels/generated/sgemm_false_false_false_false_true_scalar.wgsl"),
        );
        m.insert(
            "log_vec4",
            include_str!(r"../kernels/generated/log_vec4.wgsl"),
        );
        m.insert(
            "relu_scalar",
            include_str!(r"../kernels/generated/relu_scalar.wgsl"),
        );
        m.insert(
            "sgemm_false_true_false_true_true_scalar",
            include_str!(r"../kernels/generated/sgemm_false_true_false_true_true_scalar.wgsl"),
        );
        m.insert(
            "add_scalar",
            include_str!(r"../kernels/generated/add_scalar.wgsl"),
        );
        m.insert(
            "neg_vec2",
            include_str!(r"../kernels/generated/neg_vec2.wgsl"),
        );
        m.insert(
            "floor_vec2",
            include_str!(r"../kernels/generated/floor_vec2.wgsl"),
        );
        m.insert(
            "sgemm_true_true_true_true_false_scalar",
            include_str!(r"../kernels/generated/sgemm_true_true_true_true_false_scalar.wgsl"),
        );
        m.insert(
            "concat2_scalar",
            include_str!(r"../kernels/generated/concat2_scalar.wgsl"),
        );
        m.insert(
            "layernorm_scalar",
            include_str!(r"../kernels/generated/layernorm_scalar.wgsl"),
        );
        m.insert(
            "sgemm_true_false_true_vec4",
            include_str!(r"../kernels/generated/sgemm_true_false_true_vec4.wgsl"),
        );
        m.insert(
            "sgemm_false_true_true_true_true_scalar",
            include_str!(r"../kernels/generated/sgemm_false_true_true_true_true_scalar.wgsl"),
        );
        m.insert(
            "div_scalar",
            include_str!(r"../kernels/generated/div_scalar.wgsl"),
        );
        m.insert(
            "qgemm_false_false_true_vec4",
            include_str!(r"../kernels/generated/qgemm_false_false_true_vec4.wgsl"),
        );
        m.insert(
            "exp_scalar",
            include_str!(r"../kernels/generated/exp_scalar.wgsl"),
        );
        m.insert(
            "floor_scalar",
            include_str!(r"../kernels/generated/floor_scalar.wgsl"),
        );
        m.insert(
            "neg_inplace_vec4",
            include_str!(r"../kernels/generated/neg_inplace_vec4.wgsl"),
        );
        m.insert(
            "sgemm_true_false_false_true_true_scalar",
            include_str!(r"../kernels/generated/sgemm_true_false_false_true_true_scalar.wgsl"),
        );
        m.insert(
            "ceil_vec4",
            include_str!(r"../kernels/generated/ceil_vec4.wgsl"),
        );
        m.insert(
            "sqrt_inplace_vec2",
            include_str!(r"../kernels/generated/sqrt_inplace_vec2.wgsl"),
        );
        m.insert(
            "exp_inplace_scalar",
            include_str!(r"../kernels/generated/exp_inplace_scalar.wgsl"),
        );
        m.insert(
            "sgemm_true_true_true_false_true_scalar",
            include_str!(r"../kernels/generated/sgemm_true_true_true_false_true_scalar.wgsl"),
        );
        m.insert(
            "qgemm_false_true_true_vec4",
            include_str!(r"../kernels/generated/qgemm_false_true_true_vec4.wgsl"),
        );
        m.insert(
            "qgemm_true_false_false_vec4",
            include_str!(r"../kernels/generated/qgemm_true_false_false_vec4.wgsl"),
        );
        m.insert(
            "div_vec4",
            include_str!(r"../kernels/generated/div_vec4.wgsl"),
        );
        m.insert(
            "cos_inplace_vec2",
            include_str!(r"../kernels/generated/cos_inplace_vec2.wgsl"),
        );
        m.insert(
            "slice_scalar",
            include_str!(r"../kernels/generated/slice_scalar.wgsl"),
        );
        m.insert(
            "floor_inplace_scalar",
            include_str!(r"../kernels/generated/floor_inplace_scalar.wgsl"),
        );
        m.insert(
            "exp_inplace_vec4",
            include_str!(r"../kernels/generated/exp_inplace_vec4.wgsl"),
        );
        m.insert(
            "sgemm_false_true_true_false_true_scalar",
            include_str!(r"../kernels/generated/sgemm_false_true_true_false_true_scalar.wgsl"),
        );
        m.insert(
            "cos_scalar",
            include_str!(r"../kernels/generated/cos_scalar.wgsl"),
        );
        m.insert("-_vec2", include_str!(r"../kernels/generated/-_vec2.wgsl"));
        m.insert(
            "abs_inplace_vec2",
            include_str!(r"../kernels/generated/abs_inplace_vec2.wgsl"),
        );
        m.insert(
            "sqrt_vec2",
            include_str!(r"../kernels/generated/sqrt_vec2.wgsl"),
        );
        m.insert(
            "add_vec2",
            include_str!(r"../kernels/generated/add_vec2.wgsl"),
        );
        m.insert(
            "abs_inplace_scalar",
            include_str!(r"../kernels/generated/abs_inplace_scalar.wgsl"),
        );
        m.insert(
            "sgemm_false_true_true_true_false_scalar",
            include_str!(r"../kernels/generated/sgemm_false_true_true_true_false_scalar.wgsl"),
        );
        m.insert(
            "concat7_scalar",
            include_str!(r"../kernels/generated/concat7_scalar.wgsl"),
        );
        m.insert(
            "sin_scalar",
            include_str!(r"../kernels/generated/sin_scalar.wgsl"),
        );
        m.insert(
            "sgemm_false_true_false_false_false_scalar",
            include_str!(r"../kernels/generated/sgemm_false_true_false_false_false_scalar.wgsl"),
        );
        m.insert(
            "gelu_scalar",
            include_str!(r"../kernels/generated/gelu_scalar.wgsl"),
        );
        m.insert(
            "sin_inplace_vec2",
            include_str!(r"../kernels/generated/sin_inplace_vec2.wgsl"),
        );
        m.insert(
            "abs_vec4",
            include_str!(r"../kernels/generated/abs_vec4.wgsl"),
        );
        m.insert(
            "sgemm_true_true_false_false_false_scalar",
            include_str!(r"../kernels/generated/sgemm_true_true_false_false_false_scalar.wgsl"),
        );
        m.insert(
            "sgemm_true_true_true_vec4",
            include_str!(r"../kernels/generated/sgemm_true_true_true_vec4.wgsl"),
        );
        m.insert(
            "ceil_vec2",
            include_str!(r"../kernels/generated/ceil_vec2.wgsl"),
        );
        m.insert(
            "sqrt_inplace_vec4",
            include_str!(r"../kernels/generated/sqrt_inplace_vec4.wgsl"),
        );
        m.insert(
            "neg_inplace_scalar",
            include_str!(r"../kernels/generated/neg_inplace_scalar.wgsl"),
        );
        m.insert(
            "concat4_scalar",
            include_str!(r"../kernels/generated/concat4_scalar.wgsl"),
        );
        m.insert(
            "neg_vec4",
            include_str!(r"../kernels/generated/neg_vec4.wgsl"),
        );
        m.insert(
            "sgemm_true_true_false_vec4",
            include_str!(r"../kernels/generated/sgemm_true_true_false_vec4.wgsl"),
        );
        m.insert(
            "floor_vec4",
            include_str!(r"../kernels/generated/floor_vec4.wgsl"),
        );
        m.insert(
            "qgemm_false_false_false_vec4",
            include_str!(r"../kernels/generated/qgemm_false_false_false_vec4.wgsl"),
        );
        m.insert(
            "relu_inplace_scalar",
            include_str!(r"../kernels/generated/relu_inplace_scalar.wgsl"),
        );
        m.insert(
            "permute_scalar",
            include_str!(r"../kernels/generated/permute_scalar.wgsl"),
        );
        m.insert(
            "qgemm_false_true_false_vec4",
            include_str!(r"../kernels/generated/qgemm_false_true_false_vec4.wgsl"),
        );
        m.insert(
            "neg_inplace_vec2",
            include_str!(r"../kernels/generated/neg_inplace_vec2.wgsl"),
        );
        m.insert(
            "abs_inplace_vec4",
            include_str!(r"../kernels/generated/abs_inplace_vec4.wgsl"),
        );
        m.insert(
            "sqrt_vec4",
            include_str!(r"../kernels/generated/sqrt_vec4.wgsl"),
        );
        m.insert(
            "add_vec4",
            include_str!(r"../kernels/generated/add_vec4.wgsl"),
        );
        m.insert(
            "sin_inplace_vec4",
            include_str!(r"../kernels/generated/sin_inplace_vec4.wgsl"),
        );
        m.insert(
            "abs_vec2",
            include_str!(r"../kernels/generated/abs_vec2.wgsl"),
        );
        m.insert(
            "sgemm_false_false_true_true_false_scalar",
            include_str!(r"../kernels/generated/sgemm_false_false_true_true_false_scalar.wgsl"),
        );
        m.insert(
            "concat8_scalar",
            include_str!(r"../kernels/generated/concat8_scalar.wgsl"),
        );
        m.insert(
            "log_scalar",
            include_str!(r"../kernels/generated/log_scalar.wgsl"),
        );
        m.insert(
            "sgemm_false_true_false_false_true_scalar",
            include_str!(r"../kernels/generated/sgemm_false_true_false_false_true_scalar.wgsl"),
        );
        m.insert(
            "sgemm_false_true_false_true_false_scalar",
            include_str!(r"../kernels/generated/sgemm_false_true_false_true_false_scalar.wgsl"),
        );
        m.insert(
            "sgemm_false_true_true_false_false_scalar",
            include_str!(r"../kernels/generated/sgemm_false_true_true_false_false_scalar.wgsl"),
        );
        m.insert(
            "cos_inplace_vec4",
            include_str!(r"../kernels/generated/cos_inplace_vec4.wgsl"),
        );
        m.insert(
            "div_vec2",
            include_str!(r"../kernels/generated/div_vec2.wgsl"),
        );
        m.insert(
            "sgemm_false_false_true_false_true_scalar",
            include_str!(r"../kernels/generated/sgemm_false_false_true_false_true_scalar.wgsl"),
        );
        m.insert(
            "exp_inplace_vec2",
            include_str!(r"../kernels/generated/exp_inplace_vec2.wgsl"),
        );
        m.insert(
            "log_inplace_scalar",
            include_str!(r"../kernels/generated/log_inplace_scalar.wgsl"),
        );
        m.insert("-_vec4", include_str!(r"../kernels/generated/-_vec4.wgsl"));
        m.insert("conv_scalar", include_str!(r"../kernels/conv_scalar.wgsl"));
        m.insert(
            "cache_scalar",
            include_str!(r"../kernels/cache_scalar.wgsl"),
        );
        m.insert(
            "index_write_scalar",
            include_str!(r"../kernels/index_write_scalar.wgsl"),
        );
        m.insert(
            "wq8_index_select_scalar",
            include_str!(r"../kernels/wq8_index_select_scalar.wgsl"),
        );
        m.insert(
            "wq8_index_select_coarse_scalar",
            include_str!(r"../kernels/wq8_index_select_coarse_scalar.wgsl"),
        );
        m.insert("rope_scalar", include_str!(r"../kernels/rope_scalar.wgsl"));
        m.insert(
            "f32_index_select_scalar",
            include_str!(r"../kernels/f32_index_select_scalar.wgsl"),
        );
        m.insert(
            "softmax_vec2",
            include_str!(r"../kernels/softmax_vec2.wgsl"),
        );
        m.insert(
            "softmax_scalar",
            include_str!(r"../kernels/softmax_scalar.wgsl"),
        );
        m.insert(
            "softmax_vec4",
            include_str!(r"../kernels/softmax_vec4.wgsl"),
        );
        m
    };
}
