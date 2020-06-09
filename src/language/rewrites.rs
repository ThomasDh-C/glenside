use super::{Language, MyAnalysis};
use egg::{rewrite, Applier, ConditionalApplier, EGraph, Id, Pattern, Rewrite, Subst, Var};
use ndarray::Dimension;

// TODO(@gussmith23) I think I should make this a conditional applier, and fold in
// checks to make sure it has a shape and that it's an input
pub fn has_shape(
    var: &'static str,
) -> impl Fn(&mut egg::EGraph<Language, MyAnalysis>, egg::Id, &egg::Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| !egraph[subst[&var]].data.shape.is_none()
}
/// short_circuit lets us return early if we don't actually care about the
/// result of this check. This is the easiest way I could find to do this using
/// egg's conditional appliers.
/// TODO(@gussmith23) make this cleaner
pub fn is_symbol(
    short_circuit: bool,
    var: &'static str,
) -> impl Fn(&mut egg::EGraph<Language, MyAnalysis>, egg::Id, &egg::Subst) -> bool {
    let var = var.parse().unwrap();
    // TODO(@gussmith23) should this be `all` or `any` or something else entirely?
    move |egraph, _, subst| {
        if short_circuit {
            true
        } else {
            egraph[subst[&var]]
                .nodes
                .iter()
                .map(|enode| match enode {
                    Language::Symbol(_) => true,
                    _ => false,
                })
                .all(|x| x)
        }
    }
}
fn has_axis(
    var: &'static str,
    axis: usize,
) -> impl Fn(&mut egg::EGraph<Language, MyAnalysis>, egg::Id, &egg::Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| {
        axis < egraph[subst[&var]]
            .data
            .shape
            .as_ref()
            .unwrap()
            .as_array_view()
            .len()
    }
}
fn dimension_greater_than(
    var: &'static str,
    axis: usize,
    greater_than: usize,
) -> impl Fn(&mut egg::EGraph<Language, MyAnalysis>, egg::Id, &egg::Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| egraph[subst[&var]].data.shape.as_ref().unwrap()[axis] > greater_than
}
fn dimension_is_even(
    var: &'static str,
    axis: usize,
) -> impl Fn(&mut egg::EGraph<Language, MyAnalysis>, egg::Id, &egg::Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| egraph[subst[&var]].data.shape.as_ref().unwrap()[axis] % 2 == 0
}

// TODO(@gussmith23) not sure all this should be public.
pub struct RewriteNonMatchingCartConcatenateApplier {
    pub a1: egg::Var,
    pub a2: egg::Var,
    pub a_axis: usize,
    pub b1: egg::Var,
    pub b2: egg::Var,
    pub b_axis: usize,
}
impl egg::Applier<Language, MyAnalysis> for RewriteNonMatchingCartConcatenateApplier {
    fn apply_one(
        &self,
        _egraph: &mut egg::EGraph<Language, MyAnalysis>,
        _id: egg::Id,
        _subst: &egg::Subst,
    ) -> std::vec::Vec<egg::Id> {
        // For now, just want to handle these cases.
        assert!(self.a_axis == 0 || self.a_axis == 1);
        assert!(self.b_axis == 0 || self.b_axis == 1);
        assert_ne!(self.a_axis, self.b_axis);

        // We will break up the as into smaller chunks and the bs into
        // smaller chunks, so that they all match in size.
        // The goal is to have the innermost concatenates be along axis 0, and
        // the outermost concatenates to be along axis 1. Additionally, the goal
        // is that the result should only involve cartesian products of
        // concatenates, where the left and right concatenate use the same axis.
        // Then, existing rewrites can be used to bubble the concatenates up
        // through the cartesian products.

        // Each a needs to be split into 4; each b needs to be split into 4.

        // First we want to construct all of the concatenates along the 1 axis.
        // These will become our innermost concatenates.
        // One of these is already concatenateted along the 1 axis!

        // TODO(@gussmith23) left off here, I think I should actually do something
        // simpler here and just rewrite the two concatenates that are the
        // children of this cartesian product.
        // It needs some information from elsewhere in the graph, though,
        // that's the tough thing.

        // So we're going to slice-and-concatenate all 4 tensors. We'll slice the
        // as based on the bs size, and slice the bs based on the as size.
        // TODO(@gussmith23) I could write an even simpler rewrite rule that slices
        // more indiscriminately, everywhere. Right now I'm using some
        // context clue (the overarching cartesian product) to only apply
        // this where needed.

        // All I actually want to do is to rewrite that second concatenate.
        //  (cartesian-product
        //   (concatenate ?a1 ?a2 0)
        //   (concatenate ?b1 ?b2 1)
        //  )
        //  (cartesian-product
        //   (concatenate ?a1 ?a2 0)
        //   (concatenate (concatenate (slice ?b1) (slice ?b1)  0)
        //  )
        //

        vec![]
    }
}

struct SplitApplier {
    axis: usize,
}
impl egg::Applier<Language, MyAnalysis> for SplitApplier {
    fn apply_one(
        &self,
        egraph: &mut egg::EGraph<Language, MyAnalysis>,
        id: egg::Id,
        _subst: &egg::Subst,
    ) -> std::vec::Vec<egg::Id> {
        let shape: ndarray::IxDyn = MyAnalysis::get_shape(id, egraph).clone();
        assert_eq!(shape[self.axis] % 2, 0);
        let low_bound = 0;
        let low_bound_id = egraph.add(Language::Usize(low_bound));
        let high_bound = shape[self.axis];
        let high_bound_id = egraph.add(Language::Usize(high_bound));
        let middle_bound = high_bound / 2;
        let middle_bound_id = egraph.add(Language::Usize(middle_bound));

        let axis_id = egraph.add(Language::Usize(self.axis));

        let slice_0_id = egraph.add(Language::Slice([
            id,
            axis_id,
            low_bound_id,
            middle_bound_id,
        ]));
        let slice_1_id = egraph.add(Language::Slice([
            id,
            axis_id,
            middle_bound_id,
            high_bound_id,
        ]));

        let id: egg::Id = egraph.add(Language::Concatenate([slice_0_id, slice_1_id, axis_id]));

        vec![id]
    }
}

pub fn split(
    axis: usize,
    dimension_greater_than: usize,
    split_all_nodes: bool,
) -> Rewrite<Language, MyAnalysis> {
    rewrite!(format!("split-axis-{}", axis); "?a" =>
                  {SplitApplier{axis: axis}}
                  if self::dimension_greater_than("?a", axis, dimension_greater_than)
                  if dimension_is_even("?a", axis)
                  if has_axis("?a", axis)
                  if has_shape("?a")
                  if is_symbol(split_all_nodes, "?a"))
}

pub fn collapse_nested_slices() -> Rewrite<Language, MyAnalysis> {
    struct CollapseNestedSlicesApplier {
        low0: Var,
        high0: Var,
        low1: Var,
        high1: Var,
    }
    impl Applier<Language, MyAnalysis> for CollapseNestedSlicesApplier {
        fn apply_one(
            &self,
            egraph: &mut EGraph<Language, MyAnalysis>,
            eclass: Id,
            subst: &Subst,
        ) -> Vec<Id> {
            let low0: usize = MyAnalysis::get_usize(subst[&self.low0], egraph);
            let high0: usize = MyAnalysis::get_usize(subst[&self.high0], egraph);
            let low1: usize = MyAnalysis::get_usize(subst[&self.low1], egraph);
            let high1: usize = MyAnalysis::get_usize(subst[&self.high1], egraph);

            let new_low: usize = low0 + low1;
            assert!(high1 - low1 <= high0 - low0);
            let new_high: usize = new_low + (high1 - low1);

            format!("(slice ?t ?axis {} {})", new_low, new_high)
                .parse::<Pattern<Language>>()
                .unwrap()
                .apply_one(egraph, eclass, subst)
        }
    }
    rewrite!("collapse-nested-slices";
    "(slice (slice ?t ?axis ?low0 ?high0) ?axis ?low1 ?high1)" =>
    { CollapseNestedSlicesApplier {
        low0: "?low0".parse().unwrap(),
        low1: "?low1".parse().unwrap(),
        high0: "?high0".parse().unwrap(),
        high1: "?high1".parse().unwrap(),
    }})
}

pub fn bubble_concatenate_through_move_axis() -> Rewrite<Language, MyAnalysis> {
    struct MoveAxisApplier {
        concatenate_axis: Var,
        src_axis: Var,
        dst_axis: Var,
    }
    impl Applier<Language, MyAnalysis> for MoveAxisApplier {
        fn apply_one(
            &self,
            egraph: &mut EGraph<Language, MyAnalysis>,
            eclass: Id,
            subst: &Subst,
        ) -> Vec<Id> {
            let original_concatenate_axis: usize =
                MyAnalysis::get_usize(subst[&self.concatenate_axis], egraph);
            let src_axis: usize = MyAnalysis::get_usize(subst[&self.src_axis], egraph);
            let dst_axis: usize = MyAnalysis::get_usize(subst[&self.dst_axis], egraph);

            // If the move now happens /before/ the concatenate, we have to
            // figure out what the new axis for the concatenate is.
            // TODO(@gussmith23) Would be nice to have a more principled system of
            // keeping track of axes. This is where Remy's relational algebra
            // stuff could be really useful!
            let new_concatenate_axis: usize = if (original_concatenate_axis < src_axis
                && original_concatenate_axis < dst_axis)
                || (original_concatenate_axis > src_axis && original_concatenate_axis > dst_axis)
            {
                // Axis is unaffected if it's not between src and dst.
                original_concatenate_axis
            } else if original_concatenate_axis == src_axis {
                dst_axis
            } else if original_concatenate_axis < src_axis && original_concatenate_axis >= dst_axis
            {
                original_concatenate_axis + 1
            } else if original_concatenate_axis > src_axis && original_concatenate_axis <= dst_axis
            {
                original_concatenate_axis - 1
            } else {
                unreachable!()
            };

            format!(
                "(concatenate
                      (move-axis ?a ?src-axis ?dst-axis)
                      (move-axis ?b ?src-axis ?dst-axis) {})",
                new_concatenate_axis
            )
            .parse::<Pattern<_>>()
            .unwrap()
            .apply_one(egraph, eclass, subst)
        }
    }
    rewrite!("bubble-concatenate-through-move-axis";
        "(move-axis (concatenate ?a ?b ?concatenate-axis) ?src-axis ?dst-axis)" =>
    {
        MoveAxisApplier {
            concatenate_axis: "?concatenate-axis".parse().unwrap(),
            src_axis:"?src-axis".parse().unwrap(),
            dst_axis:"?dst-axis".parse().unwrap()
        }
    })
}

/// Whether an axis is the last axis of a given tensor
fn last_axis(
    var: &'static str,
    axis: &'static str,
) -> impl Fn(&mut egg::EGraph<Language, MyAnalysis>, egg::Id, &egg::Subst) -> bool {
    let var = var.parse().unwrap();
    let axis_id = axis.parse().unwrap();
    move |egraph, _, subst| {
        egraph[subst[&var]]
            .data
            .shape
            .as_ref()
            .unwrap()
            .as_array_view()
            .len()
            - 1
            == egraph[subst[&axis_id]].data.usize_value.unwrap()
    }
}
fn not_last_axis(
    var: &'static str,
    axis: &'static str,
) -> impl Fn(&mut egg::EGraph<Language, MyAnalysis>, egg::Id, &egg::Subst) -> bool {
    move |egraph, id, subst| !(last_axis(var, axis)(egraph, id, subst))
}
fn same_number_of_dimensions(
    a: &'static str,
    b: &'static str,
) -> impl Fn(&mut egg::EGraph<Language, MyAnalysis>, egg::Id, &egg::Subst) -> bool {
    let a = a.parse().unwrap();
    let b = b.parse().unwrap();
    move |egraph, _, subst| {
        egraph[subst[&a]]
            .data
            .shape
            .as_ref()
            .unwrap()
            .as_array_view()
            .len()
            == egraph[subst[&b]]
                .data
                .shape
                .as_ref()
                .unwrap()
                .as_array_view()
                .len()
    }
}

// TODO(@gussmith23) naming
pub fn bubble_concatenate_through_cartesian_product_not_last_axis_left(
) -> Rewrite<Language, MyAnalysis> {
    rewrite!("bubble-concatenate-through-cartesian-product-not-last-axis-left";
                  "(cartesian-product (concatenate ?t1 ?t2 ?axis) ?right)" =>
                  "(concatenate
                    (cartesian-product ?t1 ?right)
                    (cartesian-product ?t2 ?right)
                    ?axis)"
                  if not_last_axis("?t1", "?axis")
                  // This should always be true, for now. Just making extra sure
                  if same_number_of_dimensions("?t1", "?t2"))
}

struct BubbleConcatenateThroughCartesianProductNotLastAxisRightApplier {
    left: Var,
    axis: Var,
}
impl Applier<Language, MyAnalysis>
    for BubbleConcatenateThroughCartesianProductNotLastAxisRightApplier
{
    fn apply_one(
        &self,
        egraph: &mut EGraph<Language, MyAnalysis>,
        matched_id: Id,
        subst: &Subst,
    ) -> Vec<Id> {
        // cart-prod [a1, ..., an, c] [b1, ..., bm, c]
        // = [a1, ..., an, b1, ..., bm, 2, c]
        // So the axis gets shifted over by the a1, ..., an added in.
        let left_shape = MyAnalysis::get_shape(subst[&self.left], egraph);
        let left_shape_length: usize = left_shape.as_array_view().len();
        let old_axis: usize = MyAnalysis::get_usize(subst[&self.axis], egraph);
        let new_axis = old_axis + left_shape_length - 1;

        let applier: Pattern<Language> = format!(
            "(concatenate
                    (cartesian-product ?left ?t1)
                    (cartesian-product ?left ?t2)
                    {})",
            new_axis
        )
        .parse()
        .unwrap();

        applier.apply_one(egraph, matched_id, subst)
    }
}

// TODO(@gussmith23) naming
pub fn bubble_concatenate_through_cartesian_product_not_last_axis_right(
) -> Rewrite<Language, MyAnalysis> {
    rewrite!("bubble-concatenate-through-cartesian-product-not-last-axis-right";
    "(cartesian-product ?left (concatenate ?t1 ?t2 ?axis))" =>
    {
        ConditionalApplier {
            applier: ConditionalApplier {
                applier:
                BubbleConcatenateThroughCartesianProductNotLastAxisRightApplier {
                    left: "?left".parse().unwrap(),
                    axis: "?axis".parse().unwrap(),
                },
                condition: not_last_axis("?t1", "?axis")
            },
            condition: same_number_of_dimensions("?t1", "?t2")
        }
    })
}

struct BubbleConcatenateThroughCartesianProductLastAxisApplier {
    // Note that we're assuming a1's shape is the same as a2; same with b1 and
    // b2.
    a1: Var,
    b1: Var,
}
impl Applier<Language, MyAnalysis> for BubbleConcatenateThroughCartesianProductLastAxisApplier {
    fn apply_one(
        &self,
        egraph: &mut EGraph<Language, MyAnalysis>,
        matched_id: Id,
        subst: &Subst,
    ) -> Vec<Id> {
        // cart-prod [a1, ..., an, c] [b1, ..., bm, c]
        // = [a1, ..., an, b1, ..., bm, 2, c]
        // axis1 and axis2 both point to their c dimension.
        let a_shape = MyAnalysis::get_shape(subst[&self.a1], egraph);
        let a_shape_length: usize = a_shape.as_array_view().len();
        let b_shape = MyAnalysis::get_shape(subst[&self.b1], egraph);
        let b_shape_length: usize = b_shape.as_array_view().len();
        let new_axis = a_shape_length - 1 // skip [a1, ..., an]
            + b_shape_length - 1          // skip [b1, ..., bm]
            + 1; // skip [2]

        // TODO
        let applier: Pattern<Language> = format!(
            // "(concatenate
            //   (concatenate
            //    (cartesian-product ?a1 ?b1)
            //    (cartesian-product ?a1 ?b2)
            //    {0})
            //   (concatenate
            //    (cartesian-product ?a2 ?b1)
            //    (cartesian-product ?a2 ?b2)
            //    {0})
            //  {0})",
            "(concatenate
              (cartesian-product ?a1 ?b1)
              (cartesian-product ?a2 ?b2)
             {0})",
            new_axis
        )
        .parse()
        .unwrap();

        applier.apply_one(egraph, matched_id, subst)
    }
}

// TODO(@gussmith23) naming
pub fn bubble_concatenate_through_cartesian_product_last_axis() -> Rewrite<Language, MyAnalysis> {
    // TODO(@gussmith23) I think we need more checks here, to make sure that the sizes
    // actually line up correctly.
    rewrite!("bubble-concatenate-through-cartesian-product-last-axis";
    "(cartesian-product (concatenate ?a1 ?a2 ?axis1) (concatenate ?b1 ?b2 ?axis2))" =>

    {
        ConditionalApplier {
            condition: same_number_of_dimensions("?a1", "?a2"),
            applier: ConditionalApplier {
                condition: last_axis("?a1", "?axis1"),
                applier:                       ConditionalApplier {
                    condition: same_number_of_dimensions("?b1", "?b2"),
                    applier: ConditionalApplier {
                        condition: last_axis("?b1", "?axis2"),
                        applier: BubbleConcatenateThroughCartesianProductLastAxisApplier {
                            a1: "?a1".parse().unwrap(),
                            b1: "?b1".parse().unwrap(),
                        }
                    }
                }

            }
        }
    })
}

pub fn bubble_concatenate_through_cartesian_product_axis_0_0() -> Rewrite<Language, MyAnalysis> {
    // TODO(@gussmith23) this isn't the only way this could be done.
    // Also there's gotta be a name for this in terms of algebraic rules
    // TODO(@gussmith23) would it make our pattern-matching life easier if (1) we
    // put the axes at the start of concatenate and (2) we used cons cells?
    rewrite!("bubble-concatenate-through-cartesian-product-axes-0-0";
                  "(cartesian-product (concatenate ?a1 ?a2 0) (concatenate ?b1 ?b2 0))"
                  // TODO(@gussmith23) check this
                  => "(concatenate
                           (concatenate (cartesian-product ?a1 ?b1)
                                   (cartesian-product ?a1 ?b2) 1)
                           (concatenate (cartesian-product ?a2 ?b1)
                                   (cartesian-product ?a2 ?b2) 1)
                           0)"
    )
}
pub fn rewrite_nonmatching_cartesian_product_concatenate() -> Rewrite<Language, MyAnalysis> {
    rewrite!(
    "rewrite-nonmatching-cartesian-product-concatenate";
    "(cartesian-product
              (concatenate ?a1 ?a2 0)
              (concatenate ?b1 ?b2 1)
             )" =>
    {RewriteNonMatchingCartConcatenateApplier{
        a1:"?a1".parse().unwrap(),
        a2:"?a2".parse().unwrap(),
        a_axis:0,
        b1:"?b1".parse().unwrap(),
        b2:"?b2".parse().unwrap(),
        b_axis:1,
    }})
}

pub fn bubble_concatenate_through_map_dot_product_not_last_axis() -> Rewrite<Language, MyAnalysis> {
    rewrite!(

        "bubble-concatenate-through-map-dot-product-not-last-axis";
        "(map-dot-product
          (concatenate ?left ?right ?axis)
         )" =>
        "(concatenate
          (map-dot-product ?left)
          (map-dot-product ?right)
         ?axis)"
            if not_last_axis("?left", "?axis")
            // This should always be true, for now. Just making extra sure
            if same_number_of_dimensions("?left", "?right")
    )
}

pub fn bubble_concatenate_through_map_dot_product_last_axis() -> Rewrite<Language, MyAnalysis> {
    rewrite!(

        "bubble-concatenate-through-map-dot-product-last-axis";
        "(map-dot-product
          (concatenate ?left ?right ?axis)
         )" =>
            "(elementwise-add
              (map-dot-product ?left)
              (map-dot-product ?right)
             )"
            if last_axis("?left", "?axis")
            // This should always be true, for now. Just making extra sure
            if same_number_of_dimensions("?left", "?right")
    )
}

pub fn systolic_array_vector_matrix() -> Rewrite<Language, MyAnalysis> {
    struct SystolicArrayApplier {
        a: Var,
        b: Var,
    }
    impl Applier<Language, MyAnalysis> for SystolicArrayApplier {
        fn apply_one(
            &self,
            egraph: &mut EGraph<Language, MyAnalysis>,
            eclass: Id,
            subst: &Subst,
        ) -> Vec<Id> {
            let a_shape = MyAnalysis::get_shape(subst[&self.a], egraph);
            let b_shape = MyAnalysis::get_shape(subst[&self.b], egraph);
            assert_eq!(a_shape.as_array_view().len(), 1);
            assert_eq!(b_shape.as_array_view().len(), 2);
            let rows: usize = b_shape.as_array_view()[0];
            let cols: usize = b_shape.as_array_view()[1];

            let pattern: Pattern<Language> =
                format!("(bsg-systolic-array {} {} ?a ?b)", rows, cols)
                    .parse()
                    .unwrap();

            pattern.apply_one(egraph, eclass, subst)
        }
    }

    rewrite!("systolic-array";
             // TODO(@gussmith23) should check that ?a is a vector.
             "(map-dot-product (cartesian-product ?a (move-axis ?b 1 0)))" =>
             {SystolicArrayApplier{a: "?a".parse().unwrap(), b: "?b".parse().unwrap(),}})
}

#[cfg(test)]
mod tests {

    use super::*;
    use egg::{Pattern, Runner, Searcher};

    #[test]
    fn split() {
        let program = "t-32-32".parse().unwrap();

        let rws = vec![
            super::split(0, 16, true),
            super::split(1, 16, true),
            super::collapse_nested_slices(),
        ];

        let mut egraph = egg::EGraph::<Language, MyAnalysis>::new(MyAnalysis);
        egraph.add_expr(&program);
        let runner = Runner::<_, _, ()>::new(MyAnalysis)
            .with_egraph(egraph)
            .run(&rws);

        assert_eq!(
            "(slice (slice t-32-32 1 0 16) 0 0 16)"
                .parse::<Pattern<_>>()
                .unwrap()
                .search(&runner.egraph)
                .len(),
            1
        );

        assert_eq!(
            "(slice (slice t-32-32 1 16 32) 0 0 16)"
                .parse::<Pattern<_>>()
                .unwrap()
                .search(&runner.egraph)
                .len(),
            1
        );

        assert_eq!(
            "(slice (slice t-32-32 1 0 16) 0 16 32)"
                .parse::<Pattern<_>>()
                .unwrap()
                .search(&runner.egraph)
                .len(),
            1
        );

        assert_eq!(
            "(slice (slice t-32-32 1 16 32) 0 16 32)"
                .parse::<Pattern<_>>()
                .unwrap()
                .search(&runner.egraph)
                .len(),
            1
        );
    }
}
