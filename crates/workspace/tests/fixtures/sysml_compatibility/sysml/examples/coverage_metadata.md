# META
~~~ini
description=Coverage: Metadata features with about clause and named metadata
type=file
~~~
# SOURCE
~~~sysml
metadata def Classified;
metadata def Approval;

package Annotated {
    @ Classified about Annotated;

    part def Vehicle;
    part def Engine;

    metadata m : Classified about Vehicle, Engine;

    #Classified part def AnnotatedPart;

    #Approval #Classified part def MultiAnnotated;
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# TOKENS
~~~zig
KwMetadata,KwDef,Ident,Semicolon,
KwMetadata,KwDef,Ident,Semicolon,
KwPackage,Ident,OpenCurly,
At,Ident,KwAbout,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwMetadata,Ident,Colon,Ident,KwAbout,Ident,Comma,Ident,Semicolon,
Hash,Ident,KwPart,KwDef,Ident,Semicolon,
Hash,Ident,Hash,Ident,KwPart,KwDef,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (metadata_def 'Classified')
  (metadata_def 'Approval')
  (package_def 'Annotated'
    (metadata_feature typed 'Classified' about 'Annotated')
    (part_def 'Vehicle')
    (part_def 'Engine')
    (metadata_feature 'm' typed 'Classified' about 'Vehicle', 'Engine')
    (part_def #'Classified' 'AnnotatedPart')
    (part_def #'Approval', 'Classified' 'MultiAnnotated')))
~~~
# FORMAT
~~~sysml
metadata def Classified;
metadata def Approval;

package Annotated {
    @ Classified about Annotated;

    part def Vehicle;
    part def Engine;

    metadata m : Classified about Vehicle, Engine;

    #Classified part def AnnotatedPart;

    #Approval #Classified part def MultiAnnotated;
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Annotated"))) (name "Annotated") (declared-name "Annotated")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Annotated::AnnotatedPart"))) (name "AnnotatedPart") (declared-name "AnnotatedPart") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Annotated::Engine"))) (name "Engine") (declared-name "Engine") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Annotated::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "Annotated::_Classified"))) (name "Classified") (declared-name "Classified"))
        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "Annotated::m"))) (name "m") (declared-name "m"))
      )
    )
    (element (kind "metadata def") (id (node (document "d0") (qualified-name "Approval"))) (name "Approval") (declared-name "Approval"))
    (element (kind "metadata def") (id (node (document "d0") (qualified-name "Classified"))) (name "Classified") (declared-name "Classified"))
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Annotated::_Classified"))) (to (node (document "d0") (qualified-name "Annotated"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Annotated::m"))) (to (node (document "d0") (qualified-name "Annotated::Engine"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Annotated::m"))) (to (node (document "d0") (qualified-name "Annotated::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Annotated::m"))) (to (node (document "d0") (qualified-name "Classified"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Annotated::AnnotatedPart"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Annotated::Engine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Annotated::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Annotated::_Classified"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Annotated::m"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Approval"))) (status missing-prerequisite) (target "Metadata::MetadataItem"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Classified"))) (status missing-prerequisite) (target "Metadata::MetadataItem"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/coverage_metadata.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "sysml")
        (range (start 4 4) (end 4 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "sysml")
        (range (start 13 4) (end 13 51))
      )
    )
  )
)
~~~
