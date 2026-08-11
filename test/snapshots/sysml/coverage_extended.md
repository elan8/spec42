# META
~~~ini
description=Group 12: Extended Definitions and Usages (SysML §8.2.2.27)
type=file
~~~
# SOURCE
~~~sysml
package ExtendedExamples {
    #situation def Failure;
    #situation def Failure :> Base;
    abstract #situation def AbstractFailure;
    #SecurityRelated #situation def Vulnerability;
    #situation def Failure { part p; }
    #situation batteryLow;
    #situation x : T;
    #situation x : T { }
    variation #situation def V;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_extended.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 1 15) (end 1 32))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 2 15) (end 2 40))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 3 4) (end 3 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "sysml")
        (range (start 4 4) (end 4 55))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 5 15) (end 5 43))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 6 15) (end 6 31))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 7 15) (end 7 26))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 8 15) (end 8 29))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 9 4) (end 9 32))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
Hash,Ident,KwDef,Ident,Semicolon,
Hash,Ident,KwDef,Ident,ColonGt,Ident,Semicolon,
KwAbstract,Hash,Ident,KwDef,Ident,Semicolon,
Hash,Ident,Hash,Ident,KwDef,Ident,Semicolon,
Hash,Ident,KwDef,Ident,OpenCurly,KwPart,Ident,Semicolon,CloseCurly,
Hash,Ident,Ident,Semicolon,
Hash,Ident,Ident,Colon,Ident,Semicolon,
Hash,Ident,Ident,Colon,Ident,OpenCurly,CloseCurly,
KwVariation,Hash,Ident,KwDef,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ExtendedExamples'
    (extended_def #'situation' 'Failure')
    (extended_def #'situation' 'Failure' :> 'Base')
    (extended_def abstract #'situation' 'AbstractFailure')
    (extended_def #'SecurityRelated', 'situation' 'Vulnerability')
    (extended_def #'situation' 'Failure'
      (part_usage 'p'))
    (extended_usage #'situation' 'batteryLow')
    (extended_usage #'situation' 'x' : 'T')
    (extended_usage #'situation' 'x' : 'T')
    (extended_def variation #'situation' 'V')))
~~~
# EXPECTED
~~~
semantic.duplicate_name 'Failure'
semantic.duplicate_name 'Failure'
semantic.duplicate_name 'x'
semantic.ambiguous_member 'Failure'
semantic.ambiguous_member 'Failure'
semantic.ambiguous_member 'x'
semantic.unresolved_name 'Base'
semantic.unresolved_name 'T'
semantic.unresolved_name 'T'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'Failure'
semantic.duplicate_name 'Failure'
semantic.duplicate_name 'x'
semantic.ambiguous_member 'Failure'
semantic.ambiguous_member 'Failure'
semantic.ambiguous_member 'x'
semantic.unresolved_name 'Base'
semantic.unresolved_name 'T'
semantic.unresolved_name 'T'
~~~
# FORMAT
~~~sysml
package ExtendedExamples {
    #situation def Failure;
    #situation def Failure :> Base;
    abstract #situation def AbstractFailure;
    #SecurityRelated #situation def Vulnerability;
    #situation def Failure { part p; }
    #situation batteryLow;
    #situation x : T;
    #situation x : T { }
    variation #situation def V;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "195b70ac4690dcf66b928bac48890da04bb5ae444ebcad237ac9d0c949b0ee67") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ExtendedExamples"))) (kind "package") (name "ExtendedExamples") (declared-name "ExtendedExamples") (range (start (line 0) (character 0)) (end (line 0) (character 333))))
    (element (id (node (document "d0") (qualified-name "ExtendedExamples::_situation"))) (kind "metadata keyword") (name "situation") (declared-name "situation") (range (start (line 1) (character 4)) (end (line 1) (character 15))) (parent (node (document "d0") (qualified-name "ExtendedExamples"))))
    (element (id (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword"))) (kind "metadata keyword") (name "situation") (declared-name "situation") (range (start (line 2) (character 4)) (end (line 2) (character 15))) (parent (node (document "d0") (qualified-name "ExtendedExamples"))))
    (element (id (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword2"))) (kind "metadata keyword") (name "situation") (declared-name "situation") (range (start (line 5) (character 4)) (end (line 5) (character 15))) (parent (node (document "d0") (qualified-name "ExtendedExamples"))))
    (element (id (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword3"))) (kind "metadata keyword") (name "situation") (declared-name "situation") (range (start (line 6) (character 4)) (end (line 6) (character 15))) (parent (node (document "d0") (qualified-name "ExtendedExamples"))))
    (element (id (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword4"))) (kind "metadata keyword") (name "situation") (declared-name "situation") (range (start (line 7) (character 4)) (end (line 7) (character 15))) (parent (node (document "d0") (qualified-name "ExtendedExamples"))))
    (element (id (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword5"))) (kind "metadata keyword") (name "situation") (declared-name "situation") (range (start (line 8) (character 4)) (end (line 8) (character 15))) (parent (node (document "d0") (qualified-name "ExtendedExamples"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
