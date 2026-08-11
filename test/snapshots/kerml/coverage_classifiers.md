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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDatatype,Ident,Semicolon,
KwClass,Ident,Semicolon,
KwStruct,Ident,Semicolon,
KwAssoc,Ident,Semicolon,
KwAssoc,KwStruct,Ident,Semicolon,
KwMetaclass,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwClass,Ident,ColonGt,Ident,Semicolon,
KwStruct,Ident,KwSpecializes,Ident,Semicolon,
KwAssoc,Ident,ColonGt,Ident,Semicolon,
KwMetaclass,Ident,KwSpecializes,Ident,Semicolon,
KwAbstract,KwDatatype,Ident,Semicolon,
KwAbstract,KwStruct,Ident,Semicolon,
KwAbstract,KwAssoc,Ident,Semicolon,
KwAbstract,KwMetaclass,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ClassifierCoverage'
    (datatype_def 'Scalar')
    (class_def 'Entity')
    (structure_def 'Vector')
    (association_def 'Relationship')
    (assoc_struct_def 'AssocStruct')
    (metaclass_def 'MetaEntity')
    (datatype_def 'D' :> 'Scalar')
    (class_def 'C' :> 'Entity')
    (structure_def 'S' :> 'Vector')
    (association_def 'A' :> 'Relationship')
    (metaclass_def 'M' :> 'MetaEntity')
    (datatype_def abstract 'AbstractScalar')
    (structure_def abstract 'AbstractVector')
    (association_def abstract 'AbstractRel')
    (metaclass_def abstract 'AbstractMeta')))
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
~~~sysml
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "7d89267b505b3ad7ef2071c37d8228cfa52002a62ca214d654fcdbe9974e9352") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage"))) (kind "package") (name "ClassifierCoverage") (declared-name "ClassifierCoverage") (range (start (line 0) (character 0)) (end (line 0) (character 473))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::A"))) (kind "kermlDecl") (name "A") (declared-name "A") (range (start (line 11) (character 4)) (end (line 11) (character 28))) (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::AbstractMeta"))) (kind "kermlDecl") (name "AbstractMeta") (declared-name "AbstractMeta") (range (start (line 17) (character 4)) (end (line 17) (character 36))) (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::AbstractRel"))) (kind "kermlDecl") (name "AbstractRel") (declared-name "AbstractRel") (range (start (line 16) (character 4)) (end (line 16) (character 31))) (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::AbstractScalar"))) (kind "kermlDecl") (name "AbstractScalar") (declared-name "AbstractScalar") (range (start (line 14) (character 4)) (end (line 14) (character 37))) (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::AbstractVector"))) (kind "classifier decl") (name "AbstractVector") (declared-name "AbstractVector") (range (start (line 15) (character 4)) (end (line 15) (character 35))) (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::C"))) (kind "classifier decl") (name "C") (declared-name "C") (range (start (line 9) (character 4)) (end (line 9) (character 22))) (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::D"))) (kind "kermlDecl") (name "D") (declared-name "D") (range (start (line 8) (character 4)) (end (line 8) (character 34))) (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::Entity"))) (kind "classifier decl") (name "Entity") (declared-name "Entity") (range (start (line 2) (character 4)) (end (line 2) (character 17))) (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::M"))) (kind "kermlDecl") (name "M") (declared-name "M") (range (start (line 12) (character 4)) (end (line 12) (character 39))) (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::MetaEntity"))) (kind "kermlDecl") (name "MetaEntity") (declared-name "MetaEntity") (range (start (line 6) (character 4)) (end (line 6) (character 25))) (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::Relationship"))) (kind "kermlDecl") (name "Relationship") (declared-name "Relationship") (range (start (line 4) (character 4)) (end (line 4) (character 23))) (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::S"))) (kind "classifier decl") (name "S") (declared-name "S") (range (start (line 10) (character 4)) (end (line 10) (character 32))) (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::Scalar"))) (kind "kermlDecl") (name "Scalar") (declared-name "Scalar") (range (start (line 1) (character 4)) (end (line 1) (character 20))) (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::Vector"))) (kind "classifier decl") (name "Vector") (declared-name "Vector") (range (start (line 3) (character 4)) (end (line 3) (character 18))) (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
    (element (id (node (document "d0") (qualified-name "ClassifierCoverage::struct"))) (kind "kermlDecl") (name "struct") (declared-name "struct") (range (start (line 5) (character 4)) (end (line 5) (character 29))) (parent (node (document "d0") (qualified-name "ClassifierCoverage"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
