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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "22ff8990e8de6d1522d19d30f2eb6ee8803010376d14ac6d65f3c78e48a2324e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ExtendedExamples"))) (kind "package") (name "ExtendedExamples") (declared-name "ExtendedExamples"))
    (element (id (node (document "d0") (qualified-name "ExtendedExamples::_situation"))) (kind "metadata keyword") (name "situation") (declared-name "situation") (parent (node (document "d0") (qualified-name "ExtendedExamples"))))
    (element (id (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword"))) (kind "metadata keyword") (name "situation") (declared-name "situation") (parent (node (document "d0") (qualified-name "ExtendedExamples"))))
    (element (id (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword2"))) (kind "metadata keyword") (name "situation") (declared-name "situation") (parent (node (document "d0") (qualified-name "ExtendedExamples"))))
    (element (id (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword3"))) (kind "metadata keyword") (name "situation") (declared-name "situation") (parent (node (document "d0") (qualified-name "ExtendedExamples"))))
    (element (id (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword4"))) (kind "metadata keyword") (name "situation") (declared-name "situation") (parent (node (document "d0") (qualified-name "ExtendedExamples"))))
    (element (id (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword5"))) (kind "metadata keyword") (name "situation") (declared-name "situation") (parent (node (document "d0") (qualified-name "ExtendedExamples"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
