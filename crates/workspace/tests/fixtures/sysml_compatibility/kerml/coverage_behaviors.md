# META
~~~ini
description=Coverage: KerML behavior, step, function, expression, predicate, bool, inv, interaction, flow, succession flow
type=file
~~~
# SOURCE
~~~kerml
package BehaviorCoverage {
    behavior Action1 {
        in x;
        out y;
    }

    step s1 : Action1;

    function F {
        in a;
        return feature result : Integer;
    }

    expr E { in x; 1 + x }

    predicate P { in x : Boolean; x }

    bool b { true }

    inv I { true }
    inv false NegI { false }

    interaction Inter {
        in x;
        out y;
    }

    class Container {
        step a1 : Action1;
        step a2 : Action1;
        succession a1 then a2;
        flow a1.y to a2.x;
        succession flow sf from a1.y to a2.x;
    }
}
~~~
# EXPECTED
~~~
semantic.duplicate_name 'a1'
semantic.ambiguous_member 'a1'
semantic.invalid_connection_end_count
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Boolean'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'a1'
semantic.ambiguous_member 'a1'
semantic.invalid_connection_end_count
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Boolean'
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwBehavior,Ident,OpenCurly,
KwIn,Ident,Semicolon,
KwOut,Ident,Semicolon,
CloseCurly,
KwStep,Ident,Colon,Ident,Semicolon,
KwFunction,Ident,OpenCurly,
KwIn,Ident,Semicolon,
KwReturn,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwExpr,Ident,OpenCurly,KwIn,Ident,Semicolon,DecimalValue,Plus,Ident,CloseCurly,
KwPredicate,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,Ident,CloseCurly,
KwBool,Ident,OpenCurly,KwTrue,CloseCurly,
KwInv,Ident,OpenCurly,KwTrue,CloseCurly,
KwInv,KwFalse,Ident,OpenCurly,KwFalse,CloseCurly,
KwInteraction,Ident,OpenCurly,
KwIn,Ident,Semicolon,
KwOut,Ident,Semicolon,
CloseCurly,
KwClass,Ident,OpenCurly,
KwStep,Ident,Colon,Ident,Semicolon,
KwStep,Ident,Colon,Ident,Semicolon,
KwSuccession,Ident,KwThen,Ident,Semicolon,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwSuccession,KwFlow,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'BehaviorCoverage'
    (behavior_def
      (feature_def in 'x')
      (feature_def out 'y'))
    (step_def)
    (function_def
      (feature_def in 'a')
      (return_member))
    (expression_def
      (feature_def in 'x')
      (result_expr_member))
    (predicate_def
      (feature_def in 'x' : 'Boolean')
      (result_expr_member))
    (boolean_expr_def
      (result_expr_member))
    (invariant_def
      (result_expr_member))
    (invariant_def
      (result_expr_member))
    (interaction_def
      (feature_def in 'x')
      (feature_def out 'y'))
    (class_def 'Container'
      (step_def)
      (step_def)
      (succession_def
        (connector_end)
        (connector_end))
      (flow_feature 'a1')
      (succession_flow_feature 'sf'
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package BehaviorCoverage {
    behavior Action1 {
        in x;
        out y;
    }

    step s1 : Action1;

    function F {
        in a;
        return feature result : Integer;
    }

    expr E { in x; 1 + x }

    predicate P { in x : Boolean; x }

    bool b { true }

    inv I { true }
    inv false NegI { false }

    interaction Inter {
        in x;
        out y;
    }

    class Container {
        step a1 : Action1;
        step a2 : Action1;
        succession a1 then a2;
        flow a1;
        succession flow sf from a1.y to a2.x;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'BehaviorCoverage'
      (behavior_def 'Action1'
        (feature_def in 'x')
        (feature_def out 'y'))
      (step_def 's1' : 'BehaviorCoverage::Action1'[behavior_def])
      (function_def 'F'
        (feature_def in 'a')
        (return_parameter_membership
          (feature_def out 'result' : 'Integer'[unresolved])))
      (expression_def 'E'
        (feature_def in 'x')
        (result_expr_membership))
      (predicate_def 'P'
        (feature_def in 'x' : 'Boolean'[unresolved])
        (result_expr_membership))
      (boolean_expr_def 'b'
        (result_expr_membership))
      (invariant_def 'I'
        (result_expr_membership))
      (invariant_def 'NegI'
        (result_expr_membership))
      (interaction_def 'Inter'
        (feature_def in 'x')
        (feature_def out 'y'))
      (class_def 'Container'
        (step_def 'a1' : 'BehaviorCoverage::Action1'[behavior_def])
        (step_def 'a2' : 'BehaviorCoverage::Action1'[behavior_def])
        (succession_def
          (connector_end 'a1')
          (connector_end 'a2'))
        (flow_usage composite 'a1')
        (flow_usage composite 'sf'
          (connector_end 'a1.y')
          (connector_end 'a2.x'))))))
~~~
