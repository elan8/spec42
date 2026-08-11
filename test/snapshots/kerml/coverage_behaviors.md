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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_behaviors.md"
    (diagnostics
    )
  )
)
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
        flow a1.y to a2.x;
        succession flow sf from a1.y to a2.x;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "9a8f000aebecf6f1ad6de43c82e06dfae58285d4bad506e9699f4b58ac95f586") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage"))) (kind "package") (name "BehaviorCoverage") (declared-name "BehaviorCoverage") (range (start (line 0) (character 0)) (end (line 0) (character 573))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::Action1"))) (kind "kermlDecl") (name "Action1") (declared-name "Action1") (range (start (line 1) (character 4)) (end (line 1) (character 57))) (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::Container"))) (kind "classifier decl") (name "Container") (declared-name "Container") (range (start (line 27) (character 4)) (end (line 27) (character 185))) (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::E"))) (kind "kermlDecl") (name "E") (declared-name "E") (range (start (line 13) (character 4)) (end (line 13) (character 26))) (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::F"))) (kind "kermlDecl") (name "F") (declared-name "F") (range (start (line 8) (character 4)) (end (line 8) (character 77))) (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::I"))) (kind "kermlDecl") (name "I") (declared-name "I") (range (start (line 19) (character 4)) (end (line 19) (character 18))) (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::Inter"))) (kind "kermlDecl") (name "Inter") (declared-name "Inter") (range (start (line 22) (character 4)) (end (line 22) (character 58))) (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::P"))) (kind "kermlDecl") (name "P") (declared-name "P") (range (start (line 15) (character 4)) (end (line 15) (character 37))) (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::b"))) (kind "kermlDecl") (name "b") (declared-name "b") (range (start (line 17) (character 4)) (end (line 17) (character 19))) (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::false"))) (kind "kermlDecl") (name "false") (declared-name "false") (range (start (line 20) (character 4)) (end (line 20) (character 28))) (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
    (element (id (node (document "d0") (qualified-name "BehaviorCoverage::s1"))) (kind "kermlDecl") (name "s1") (declared-name "s1") (range (start (line 6) (character 4)) (end (line 6) (character 22))) (parent (node (document "d0") (qualified-name "BehaviorCoverage"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
