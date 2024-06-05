#![allow(clippy::not_unsafe_ptr_arg_deref)]
use swc_core::{
    ecma::{ast::Program, visit::FoldWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
fn transform_vue_auto_prop_plugin(program: Program, _data: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut vue_auto_prop::transform_prop())
}