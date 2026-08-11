# META
~~~ini
description=Coverage: KerML classifier definition keywords (datatype, class, struct, assoc, metaclass)
type=file
~~~
# SOURCE
~~~kerml
package ClassifierCoverage {
    datatype Scalar;
    class Entity;
    struct Vector;
    assoc Relationship;
    assoc struct AssocStruct;
    metaclass MetaEntity;

    datatype D specializes Scalar;
    class C :> Entity;
    struct S specializes Vector;
    assoc A :> Relationship;
    metaclass M specializes MetaEntity;

    abstract datatype AbstractScalar;
    abstract struct AbstractVector;
    abstract assoc AbstractRel;
    abstract metaclass AbstractMeta;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_classifiers.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "7d89267b505b3ad7ef2071c37d8228cfa52002a62ca214d654fcdbe9974e9352") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage"))) (kind "package") (name "ClassifierCoverage") (declared-name "ClassifierCoverage"))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::A"))) (kind "kermlDecl") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::AbstractMeta"))) (kind "kermlDecl") (name "AbstractMeta") (declared-name "AbstractMeta") (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::AbstractRel"))) (kind "kermlDecl") (name "AbstractRel") (declared-name "AbstractRel") (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::AbstractScalar"))) (kind "kermlDecl") (name "AbstractScalar") (declared-name "AbstractScalar") (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::AbstractVector"))) (kind "classifier decl") (name "AbstractVector") (declared-name "AbstractVector") (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::C"))) (kind "classifier decl") (name "C") (declared-name "C") (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::D"))) (kind "kermlDecl") (name "D") (declared-name "D") (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::Entity"))) (kind "classifier decl") (name "Entity") (declared-name "Entity") (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::M"))) (kind "kermlDecl") (name "M") (declared-name "M") (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::MetaEntity"))) (kind "kermlDecl") (name "MetaEntity") (declared-name "MetaEntity") (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::Relationship"))) (kind "kermlDecl") (name "Relationship") (declared-name "Relationship") (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::S"))) (kind "classifier decl") (name "S") (declared-name "S") (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::Scalar"))) (kind "kermlDecl") (name "Scalar") (declared-name "Scalar") (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::Vector"))) (kind "classifier decl") (name "Vector") (declared-name "Vector") (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::struct"))) (kind "kermlDecl") (name "struct") (declared-name "struct") (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
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
