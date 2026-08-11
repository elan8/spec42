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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_metadata.md"
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
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "09f4fd17531e2f9f52420cda01132a27aa44df681136c27ef6ee3cb557edf1da") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Annotated"))) (kind "package") (name "Annotated") (declared-name "Annotated") (range (start (line 3) (character 0)) (end (line 3) (character 244))))
    (element (id (node (document "d0") (qualified-name "Annotated::AnnotatedPart"))) (kind "part def") (name "AnnotatedPart") (declared-name "AnnotatedPart") (range (start (line 11) (character 16)) (end (line 11) (character 39))) (parent (node (document "d0") (qualified-name "Annotated"))))
    (element (id (node (document "d0") (qualified-name "Annotated::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 7) (character 4)) (end (line 7) (character 20))) (parent (node (document "d0") (qualified-name "Annotated"))))
    (element (id (node (document "d0") (qualified-name "Annotated::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 6) (character 4)) (end (line 6) (character 21))) (parent (node (document "d0") (qualified-name "Annotated"))))
    (element (id (node (document "d0") (qualified-name "Annotated::_Classified"))) (kind "metadata keyword") (name "Classified") (declared-name "Classified") (range (start (line 11) (character 4)) (end (line 11) (character 16))) (parent (node (document "d0") (qualified-name "Annotated"))))
    (element (id (node (document "d0") (qualified-name "Annotated::m"))) (kind "metadata usage") (name "m") (declared-name "m") (range (start (line 9) (character 4)) (end (line 9) (character 50))) (parent (node (document "d0") (qualified-name "Annotated"))) (authored (membership (kind Feature)) (relationships (typing (reference "Classified") (range none)))))
    (element (id (node (document "d0") (qualified-name "Approval"))) (kind "metadata def") (name "Approval") (declared-name "Approval") (range (start (line 1) (character 0)) (end (line 1) (character 22))))
    (element (id (node (document "d0") (qualified-name "Classified"))) (kind "metadata def") (name "Classified") (declared-name "Classified") (range (start (line 0) (character 0)) (end (line 0) (character 24))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Annotated::m"))) (kind featureTyping) (ordinal 0)) (authored-target "Classified") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Annotated::_Classified")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Annotated::m"))) (target (node (document "d0") (qualified-name "Annotated::_Classified"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Annotated::m"))) (kind featureTyping) (ordinal 0)))
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
