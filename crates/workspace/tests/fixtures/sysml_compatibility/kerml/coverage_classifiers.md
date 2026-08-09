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
(model
  (namespace
    (package 'ClassifierCoverage'
      (datatype_def 'Scalar')
      (class_def 'Entity')
      (structure_def 'Vector')
      (association_def 'Relationship')
      (assoc_struct_def 'AssocStruct')
      (metaclass_def 'MetaEntity')
      (datatype_def 'D' :> 'ClassifierCoverage::Scalar'[datatype_def])
      (class_def 'C' :> 'ClassifierCoverage::Entity'[class_def])
      (structure_def 'S' :> 'ClassifierCoverage::Vector'[structure_def])
      (association_def 'A' :> 'ClassifierCoverage::Relationship'[association_def])
      (metaclass_def 'M' :> 'ClassifierCoverage::MetaEntity'[metaclass_def])
      (datatype_def abstract 'AbstractScalar')
      (structure_def abstract 'AbstractVector')
      (association_def abstract 'AbstractRel')
      (metaclass_def abstract 'AbstractMeta'))))
~~~
