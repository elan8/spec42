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
# FORMAT
~~~sysml
package ExtendedExamples {
    #situation def Failure;
    #situation def Failure :> Base;
    abstract #situation def AbstractFailure;
    #SecurityRelated #situation def Vulnerability;
    #situation def Failure {
        part p;
    }
    #situation batteryLow;
    #situation x : T;
    #situation x : T { }
    variation #situation def V;
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ExtendedExamples"))) (name "ExtendedExamples") (declared-name "ExtendedExamples")
      (contains
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "ExtendedExamples::_situation"))) (name "situation") (declared-name "situation"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword"))) (name "situation") (declared-name "situation"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword2"))) (name "situation") (declared-name "situation"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword3"))) (name "situation") (declared-name "situation"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword4"))) (name "situation") (declared-name "situation"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword5"))) (name "situation") (declared-name "situation"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ExtendedExamples::_situation"))) (to (node (document "d0") (qualified-name "ExtendedExamples"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword"))) (to (node (document "d0") (qualified-name "ExtendedExamples"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword2"))) (to (node (document "d0") (qualified-name "ExtendedExamples"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword3"))) (to (node (document "d0") (qualified-name "ExtendedExamples"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword4"))) (to (node (document "d0") (qualified-name "ExtendedExamples"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ExtendedExamples::_situation#metadata_keyword5"))) (to (node (document "d0") (qualified-name "ExtendedExamples"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
