extern crate swc_core;

use std::fmt::Debug;
use swc_core::{
    common::{DUMMY_SP},
    ecma::ast::*,
};
use swc_core::ecma::visit::{Fold, noop_fold_type};

#[derive(Clone, Debug)]
struct PropTransformer;

impl Fold for PropTransformer {
    noop_fold_type!();

    fn fold_class(&mut self, n: Class) -> Class {

        let mut n = n;
        for member in &mut n.body {
            if let ClassMember::ClassProp(ref mut prop) = member {
                println!("The prop of member class is {:?}", prop.decorators.get_mut(0));

                if let Some(Decorator { expr, .. }) = prop.decorators.get_mut(0) {
                    if let Expr::Call(CallExpr { callee, args, .. }) = &mut **expr {
                        if let Callee::Expr(callee_expr) = callee {
                            if let Expr::Ident(Ident { sym, .. }) = &**callee_expr {
                                if sym == "Prop" {
                                    let type_annotation = prop.type_ann.as_ref().map(|t| &*t.type_ann);

                                    if let Some(TsType::TsKeywordType(TsKeywordType { kind, .. })) = type_annotation {
                                        let type_str = match kind {
                                            TsKeywordTypeKind::TsNumberKeyword => "Number",
                                            TsKeywordTypeKind::TsStringKeyword => "String",
                                            TsKeywordTypeKind::TsBooleanKeyword => "Boolean",
                                            _ => continue,
                                        };

                                        let type_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                            key: PropName::Ident(Ident::new("type".into(), DUMMY_SP)),
                                            value: Box::new(Expr::Ident(Ident::new(type_str.into(), DUMMY_SP))),
                                        })));

                                        if args.is_empty() {
                                            args.push(ExprOrSpread {
                                                spread: None,
                                                expr: Box::new(Expr::Object(ObjectLit {
                                                    span: DUMMY_SP,
                                                    props: vec![type_prop],
                                                })),
                                            });
                                        } else if let Expr::Object(ObjectLit { ref mut props, .. }) = *args[0].expr {
                                            props.push(type_prop);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        n
    }
}

pub fn transform_prop() -> impl Fold {
    PropTransformer
}

#[cfg(test)]
mod tests {
    use super::*;
    use swc_core::{
        ecma::transforms::testing::test_transform,
        ecma::parser::{Syntax, TsConfig},
    };

    #[test]
    fn add_type_to_empty_prop() {
        test_transform(
            Syntax::Typescript(TsConfig {
                decorators: true,
                ..Default::default()
            }),
            |_| transform_prop(),
            r#"
                class MyClass {
                    @Prop()
                    public test: boolean;
                }
            "#,
            r#"
                class MyClass {
                    @Prop({ type: Boolean })
                    public test: boolean;
                }
            "#,
            false
        );
    }

    #[test]
    fn add_type_to_prop_with_default() {
        test_transform(
            Syntax::Typescript(TsConfig {
                decorators: true,
                ..Default::default()
            }),
            |_| transform_prop(),
            r#"
                class MyClass {
                    @Prop({ default: false })
                    public test: boolean;
                }
            "#,
            r#"
                class MyClass {
                    @Prop({ default: false, type: Boolean })
                    public test: boolean;
                }
            "#,
            false
        );
    }

    #[test]
    fn add_type_to_string_prop() {
        test_transform(
            Syntax::Typescript(TsConfig {
                decorators: true,
                ..Default::default()
            }),
            |_| transform_prop(),
            r#"
                class MyClass {
                    @Prop()
                    public test: string;
                }
            "#,
            r#"
                class MyClass {
                    @Prop({ type: String })
                    public test: string;
                }
            "#,
            false
        );
    }

    #[test]
    fn add_type_to_number_prop() {
        test_transform(
            Syntax::Typescript(TsConfig {
                decorators: true,
                ..Default::default()
            }),
            |_| transform_prop(),
            r#"
                class MyClass {
                    @Prop()
                    public test: number;
                }
            "#,
            r#"
                class MyClass {
                    @Prop({ type: Number })
                    public test: number;
                }
            "#,
            false
        );
    }

}