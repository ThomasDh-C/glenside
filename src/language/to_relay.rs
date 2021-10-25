use super::super::codegen::generate_worklist_for_codegen;
use super::Language;
use super::MyAnalysis;
use egg::EGraph;
use egg::Id;
use std::collections::HashMap;
use std::convert::TryFrom;
use tvm::ir::relay::*;
use tvm::ir::span::Span;
use tvm::runtime::IsObjectRef;
use tvm::Context;
use tvm::NDArray;

pub fn to_relay(egraph: &EGraph<Language, MyAnalysis>, id: Id, ctx: Context) -> Expr {
    let worklist = generate_worklist_for_codegen(egraph, id);
    let mut hashmap = HashMap::new();
    for id in worklist {
        to_relay_impl(egraph, id, ctx, &mut hashmap);
    }
    return hashmap[&id].clone();
}

fn to_relay_impl(
    egraph: &EGraph<Language, MyAnalysis>,
    id: Id,
    ctx: Context,
    hashmap: &mut HashMap<Id, Expr>,
) {
    assert!(!hashmap.contains_key(&id), "Id is already in hashmap!");
    match {
        assert_eq!(
            egraph[id].len(),
            1,
            "egraph should have a single enode per eclass"
        );
        &egraph[id].nodes[0]
    } {
        Language::Usize(v) => {
            hashmap.insert(
                id,
                Constant::new(
                    NDArray::from_rust_ndarray(
                        &ndarray12::arr0(u32::try_from(*v).unwrap()).into_dyn(),
                        ctx,
                        // TODO(@gussmith23) hardcoded code
                        DataType::new(1, 32, 1),
                    )
                    .unwrap(),
                    Span::null(),
                )
                .upcast(),
            );
        }
        Language::MoveAxis(_) => todo!(),
        Language::CartesianProduct(_) => todo!(),
        Language::MapDotProduct(_) => todo!(),
        Language::Slice(_) => todo!(),
        Language::Concatenate(_) => todo!(),
        Language::ElementwiseAdd(_) => todo!(),
        Language::BsgSystolicArray(_) => todo!(),
        Language::SystolicArray(_) => todo!(),
        Language::SystolicArrayWithBlocking(_) => todo!(),
        Language::SystolicArrayConv2dNchwOihwWithBlocking(_) => todo!(),
        Language::SystolicArrayConv2dNhwcHwioWithBlocking(_) => todo!(),
        Language::SystolicArrayConv2dIm2colNchwOihwWithBlocking(_) => todo!(),
        Language::SystolicArrayConv2dIm2colNhwcHwioWithBlocking(_) => todo!(),
        Language::AccessWindows(_) => todo!(),
        Language::ShapeOf(_) => todo!(),
        Language::SliceShape(_) => todo!(),
        Language::ShapeInsertAxis(_) => todo!(),
        Language::ShapeRemoveAxis(_) => todo!(),
        Language::Access(_) => todo!(),
        Language::AccessTranspose(_) => todo!(),
        Language::AccessCartesianProduct(_) => todo!(),
        Language::Compute(_) => todo!(),
        Language::AccessReshape(_) => todo!(),
        Language::AccessFlatten(_) => todo!(),
        Language::Shape(_) => todo!(),
        Language::List(_) => todo!(),
        Language::ConstructTuple(_) => todo!(),
        Language::TupleGetItem(_) => todo!(),
        Language::AccessShape(_) => todo!(),
        Language::AccessSlice(_) => todo!(),
        Language::AccessConcatenate(_) => todo!(),
        Language::AccessPair(_) => todo!(),
        Language::AccessShiftRight(_) => todo!(),
        Language::AccessTensor(_) => todo!(),
        Language::AccessPad(_) => todo!(),
        Language::AccessSqueeze(_) => todo!(),
        Language::AccessInsertAxis(_) => todo!(),
        Language::AccessBroadcast(_) => todo!(),
        Language::AccessLiteral(_) => todo!(),
        Language::Literal(_) => todo!(),
        Language::RelayOperatorCall(_) => todo!(),
        Language::NotNanFloat64(_) => todo!(),
        Language::RelayOperator(_) => todo!(),
        Language::RelayActivationLayout(_) => todo!(),
        Language::RelayKernelLayout(_) => todo!(),
        Language::PadType(_) => todo!(),
        Language::ComputeType(_) => todo!(),
        Language::Symbol(_) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use egg::RecExpr;
    use ndarray12::arr0;
    use ndarray12::ArrayD;
    use tvm::{
        compiler::graph_rt::{compile_module, CompilerConfig},
        ir::IRModule,
        runtime::graph_rt::GraphRt,
    };

    #[test]
    fn usize() {
        let hashmap = HashMap::new();
        let glenside_expr = RecExpr::<Language>::from_str("23").unwrap();
        let mut egraph = EGraph::new(MyAnalysis {
            name_to_shape: hashmap,
        });

        let id = egraph.add_expr(&glenside_expr);

        let out = to_relay(
            &egraph,
            id,
            Context {
                device_type: tvm::DeviceType::CPU,
                device_id: 0,
            },
        );

        let module =
            compile_module(CompilerConfig::default(), IRModule::from_expr(out).unwrap()).unwrap();

        let mut rt = GraphRt::from_module(module, Context::cpu(0)).unwrap();
        assert_eq!(
            ArrayD::<u32>::try_from(&rt.get_output(0).unwrap()).unwrap(),
            arr0(23).into_dyn()
        );
    }
}