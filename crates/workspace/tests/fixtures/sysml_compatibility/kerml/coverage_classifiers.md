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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ClassifierCoverage"))) (name "ClassifierCoverage") (declared-name "ClassifierCoverage")
      (contains
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ClassifierCoverage::A"))) (name "A") (declared-name "A"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ClassifierCoverage::AbstractMeta"))) (name "AbstractMeta") (declared-name "AbstractMeta"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ClassifierCoverage::AbstractRel"))) (name "AbstractRel") (declared-name "AbstractRel"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ClassifierCoverage::AbstractScalar"))) (name "AbstractScalar") (declared-name "AbstractScalar"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ClassifierCoverage::AbstractVector"))) (name "AbstractVector") (declared-name "AbstractVector"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ClassifierCoverage::C"))) (name "C") (declared-name "C"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ClassifierCoverage::D"))) (name "D") (declared-name "D"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ClassifierCoverage::Entity"))) (name "Entity") (declared-name "Entity"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ClassifierCoverage::M"))) (name "M") (declared-name "M"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ClassifierCoverage::MetaEntity"))) (name "MetaEntity") (declared-name "MetaEntity"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ClassifierCoverage::Relationship"))) (name "Relationship") (declared-name "Relationship"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ClassifierCoverage::S"))) (name "S") (declared-name "S"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ClassifierCoverage::Scalar"))) (name "Scalar") (declared-name "Scalar"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ClassifierCoverage::Vector"))) (name "Vector") (declared-name "Vector"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ClassifierCoverage::struct"))) (name "struct") (declared-name "struct"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
