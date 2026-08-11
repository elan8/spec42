# META
~~~ini
description=Coverage: Standalone relationship declarations (disjoining, typing, subsetting, redefinition)
type=file
~~~
# SOURCE
~~~kerml
package RelationshipCoverage {
    type A;
    type B;
    type C;
    type D;
    feature f;
    feature g;
    feature parent;
    feature child;

    disjoining d1 disjoint A from B;
    disjoint C from D;

    typing t1 typing f typed by B;
    typing g : A;

    subset parent subsets f;

    redefinition child :>> parent;
    redefinition f redefines g;

    type UnionType unions A, B;
    type InterType intersects A, B;
    type DiffType differences A, B;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_relationships.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 1 4) (end 1 435))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwType,Ident,Semicolon,
KwType,Ident,Semicolon,
KwType,Ident,Semicolon,
KwType,Ident,Semicolon,
KwFeature,Ident,Semicolon,
KwFeature,Ident,Semicolon,
KwFeature,Ident,Semicolon,
KwFeature,Ident,Semicolon,
KwDisjoining,Ident,KwDisjoint,Ident,KwFrom,Ident,Semicolon,
KwDisjoint,Ident,KwFrom,Ident,Semicolon,
KwTyping,Ident,KwTyping,Ident,KwTyped,KwBy,Ident,Semicolon,
KwTyping,Ident,Colon,Ident,Semicolon,
KwSubset,Ident,KwSubsets,Ident,Semicolon,
KwRedefinition,Ident,ColonGtGt,Ident,Semicolon,
KwRedefinition,Ident,KwRedefines,Ident,Semicolon,
KwType,Ident,KwUnions,Ident,Comma,Ident,Semicolon,
KwType,Ident,KwIntersects,Ident,Comma,Ident,Semicolon,
KwType,Ident,KwDifferences,Ident,Comma,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'RelationshipCoverage'
    (malformed)
    (malformed)
    (malformed)
    (malformed)
    (feature_def 'f')
    (feature_def 'g')
    (feature_def 'parent')
    (feature_def 'child')
    (disjoining_decl 'd1' specific 'A' general 'B')
    (disjoining_decl specific 'C' general 'D')
    (malformed)
    (feature_typing_decl specific 'f' general 'B')
    (feature_typing_decl specific 'g' general 'A')
    (subsetting_decl specific 'parent' general 'f')
    (redefinition_decl specific 'child' general 'parent')
    (redefinition_decl specific 'f' general 'g')
    (malformed)
    (malformed)
    (malformed)))
~~~
# EXPECTED
~~~
parse.expected_specialization_or_body
parse.expected_specialization_or_body
parse.expected_specialization_or_body
parse.expected_specialization_or_body
parse.expected_general_type
parse.expected_specialization_or_body
parse.expected_specialization_or_body
parse.expected_specialization_or_body
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
~~~
# PROBLEMS
~~~
parse.expected_specialization_or_body
parse.expected_specialization_or_body
parse.expected_specialization_or_body
parse.expected_specialization_or_body
parse.expected_general_type
parse.expected_specialization_or_body
parse.expected_specialization_or_body
parse.expected_specialization_or_body
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
~~~
# FORMAT
~~~sysml
package RelationshipCoverage {
    type A;
    type B;
    type C;
    type D;
    feature f;
    feature g;
    feature parent;
    feature child;

    disjoining d1 disjoint A from B;
    disjoint C from D;

    typing t1 typing f typed by B;
    typing g : A;

    subset parent subsets f;

    redefinition child :>> parent;
    redefinition f redefines g;

    type UnionType unions A, B;
    type InterType intersects A, B;
    type DiffType differences A, B;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "cec622b2a991cf3bca978916176416ec28b19282ad1a6ef3d30c5c93bbb65b10") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RelationshipCoverage"))) (kind "package") (name "RelationshipCoverage") (declared-name "RelationshipCoverage") (range (start (line 0) (character 0)) (end (line 0) (character 467))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
