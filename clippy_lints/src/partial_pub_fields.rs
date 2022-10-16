use clippy_utils::diagnostics::span_lint_and_help;
use rustc_ast::ast::*;
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};

declare_clippy_lint! {
    /// ### What it does
    /// Checks whether partial fields of a struct are public.
    ///
    /// Either make all fields of a type public, or make none of them public
    ///
    /// ### Why is this bad?
    /// Most types should either be:
    /// * Abstract data types: complex objects with opaque implementation which guard
    /// interior invariants and expose intentionally limited API to the outside world.
    /// * Data: relatively simple objects which group a bunch of related attributes together.
    ///
    /// ### Example
    /// ```rust
    /// pub struct Color {
    ///     pub r,
    ///     pub g,
    ///     b,
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// pub struct Color {
    ///     pub r,
    ///     pub g,
    ///     pub b,
    /// }
    /// ```
    #[clippy::version = "1.66.0"]
    pub PARTIAL_PUB_FIELDS,
    restriction,
    "partial fields of a struct are public"
}
declare_lint_pass!(PartialPubFields => [PARTIAL_PUB_FIELDS]);

impl EarlyLintPass for PartialPubFields {
    fn check_item(&mut self, cx: &EarlyContext<'_>, item: &Item) {
        let ItemKind::Struct(ref st, _) = item.kind else {
            return;
        };

        let mut fields = st.fields().iter();
        let Some(first_field) = fields.next() else {
            // Empty struct.
            return;
        };
        let all_pub = first_field.vis.kind.is_pub();
        let all_priv = !all_pub;

        let msg = "mixed usage of pub and non-pub fields";

        for field in fields {
            if all_priv && field.vis.kind.is_pub() {
                span_lint_and_help(
                    cx,
                    &PARTIAL_PUB_FIELDS,
                    field.vis.span,
                    msg,
                    None,
                    "consider using private field here",
                );
                return;
            } else if all_pub && !field.vis.kind.is_pub() {
                span_lint_and_help(
                    cx,
                    &PARTIAL_PUB_FIELDS,
                    field.vis.span,
                    msg,
                    None,
                    "consider using public field here",
                );
                return;
            }
        }
    }
}
