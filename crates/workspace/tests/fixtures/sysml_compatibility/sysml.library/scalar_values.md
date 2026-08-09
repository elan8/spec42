# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Data Type Library/ScalarValues
type=file
~~~
# SOURCE
~~~kerml
standard library package ScalarValues {
    doc /*
	 * This package contains a basic set of primitive scalar (non-collection) data types. 
	 * These include Boolean and String types and a hierarchy of concrete Number types, from 
	 * the most general type of Complex numbers to the most specific type of Positive integers.</p>
	 */

    private import Base::DataValue;

    abstract datatype ScalarValue specializes DataValue;
    datatype Boolean specializes ScalarValue;
    datatype String specializes ScalarValue;
    abstract datatype NumericalValue specializes ScalarValue;

    abstract datatype Number specializes NumericalValue;
    datatype Complex specializes Number;
    datatype Real specializes Complex;
    datatype Rational specializes Real;
    datatype Integer specializes Rational;
    datatype Natural specializes Integer;
    datatype Positive specializes Natural;
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'DataValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'DataValue'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwAbstract,KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwAbstract,KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ScalarValues'
    (documentation)
    (import_decl private 'Base::DataValue')
    (datatype_def abstract 'ScalarValue' :> 'DataValue')
    (datatype_def 'Boolean' :> 'ScalarValue')
    (datatype_def 'String' :> 'ScalarValue')
    (datatype_def abstract 'NumericalValue' :> 'ScalarValue')
    (datatype_def abstract 'Number' :> 'NumericalValue')
    (datatype_def 'Complex' :> 'Number')
    (datatype_def 'Real' :> 'Complex')
    (datatype_def 'Rational' :> 'Real')
    (datatype_def 'Integer' :> 'Rational')
    (datatype_def 'Natural' :> 'Integer')
    (datatype_def 'Positive' :> 'Natural')))
~~~
# FORMAT
~~~sysml
standard library package ScalarValues {
    doc /*
	 * This package contains a basic set of primitive scalar (non-collection) data types. 
	 * These include Boolean and String types and a hierarchy of concrete Number types, from 
	 * the most general type of Complex numbers to the most specific type of Positive integers.</p>
	 */

    private import Base::DataValue;

    abstract datatype ScalarValue specializes DataValue;
    datatype Boolean specializes ScalarValue;
    datatype String specializes ScalarValue;
    abstract datatype NumericalValue specializes ScalarValue;

    abstract datatype Number specializes NumericalValue;
    datatype Complex specializes Number;
    datatype Real specializes Complex;
    datatype Rational specializes Real;
    datatype Integer specializes Rational;
    datatype Natural specializes Integer;
    datatype Positive specializes Natural;
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ScalarValues"))) (name "ScalarValues") (declared-name "ScalarValues")
      (contains
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ScalarValues::Boolean"))) (name "Boolean") (declared-name "Boolean"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ScalarValues::Complex"))) (name "Complex") (declared-name "Complex"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ScalarValues::DataValue"))) (name "DataValue") (declared-name "DataValue"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ScalarValues::Integer"))) (name "Integer") (declared-name "Integer"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ScalarValues::Natural"))) (name "Natural") (declared-name "Natural"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ScalarValues::Number"))) (name "Number") (declared-name "Number"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ScalarValues::NumericalValue"))) (name "NumericalValue") (declared-name "NumericalValue"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ScalarValues::Positive"))) (name "Positive") (declared-name "Positive"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ScalarValues::Rational"))) (name "Rational") (declared-name "Rational"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ScalarValues::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ScalarValues::ScalarValue"))) (name "ScalarValue") (declared-name "ScalarValue"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ScalarValues::String"))) (name "String") (declared-name "String"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ScalarValues::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ScalarValues::_documentation"))) (to (node (document "d0") (qualified-name "ScalarValues"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
