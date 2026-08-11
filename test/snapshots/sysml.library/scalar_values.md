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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "scalar_values.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 34))
      )
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1581c1e6e3a4f3bd274af51155e25043882415cf9b55c72c7671fa3e493acc42") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ScalarValues"))) (kind "package") (name "ScalarValues") (declared-name "ScalarValues"))
    (element (id (node (document "d0") (qualified-name "ScalarValues::Boolean"))) (kind "kermlDecl") (name "Boolean") (declared-name "Boolean") (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::Complex"))) (kind "kermlDecl") (name "Complex") (declared-name "Complex") (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::DataValue"))) (kind "import") (name "DataValue") (declared-name "DataValue") (parent (node (document "d0") (qualified-name "ScalarValues"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::DataValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::Integer"))) (kind "kermlDecl") (name "Integer") (declared-name "Integer") (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::Natural"))) (kind "kermlDecl") (name "Natural") (declared-name "Natural") (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::Number"))) (kind "kermlDecl") (name "Number") (declared-name "Number") (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::NumericalValue"))) (kind "kermlDecl") (name "NumericalValue") (declared-name "NumericalValue") (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::Positive"))) (kind "kermlDecl") (name "Positive") (declared-name "Positive") (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::Rational"))) (kind "kermlDecl") (name "Rational") (declared-name "Rational") (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::Real"))) (kind "kermlDecl") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::ScalarValue"))) (kind "kermlDecl") (name "ScalarValue") (declared-name "ScalarValue") (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::String"))) (kind "kermlDecl") (name "String") (declared-name "String") (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ScalarValues"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ScalarValues::DataValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::DataValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
  (document "d0"
    (query (range (start 7 19) (end 7 34)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "ScalarValues::DataValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::DataValue")
        (range (start 7 19) (end 7 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
