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
    (element (id (node (document "d0") (qualified-name "ScalarValues"))) (kind "package") (name "ScalarValues") (declared-name "ScalarValues") (range (start (line 0) (character 0)) (end (line 0) (character 887))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::Boolean"))) (kind "kermlDecl") (name "Boolean") (declared-name "Boolean") (range (start (line 10) (character 4)) (end (line 10) (character 45))) (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::Complex"))) (kind "kermlDecl") (name "Complex") (declared-name "Complex") (range (start (line 15) (character 4)) (end (line 15) (character 40))) (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::DataValue"))) (kind "import") (name "DataValue") (declared-name "DataValue") (range (start (line 7) (character 4)) (end (line 7) (character 35))) (parent (node (document "d0") (qualified-name "ScalarValues"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::DataValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 19)) (end (line 7) (character 34))))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::Integer"))) (kind "kermlDecl") (name "Integer") (declared-name "Integer") (range (start (line 18) (character 4)) (end (line 18) (character 42))) (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::Natural"))) (kind "kermlDecl") (name "Natural") (declared-name "Natural") (range (start (line 19) (character 4)) (end (line 19) (character 41))) (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::Number"))) (kind "kermlDecl") (name "Number") (declared-name "Number") (range (start (line 14) (character 4)) (end (line 14) (character 56))) (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::NumericalValue"))) (kind "kermlDecl") (name "NumericalValue") (declared-name "NumericalValue") (range (start (line 12) (character 4)) (end (line 12) (character 61))) (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::Positive"))) (kind "kermlDecl") (name "Positive") (declared-name "Positive") (range (start (line 20) (character 4)) (end (line 20) (character 42))) (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::Rational"))) (kind "kermlDecl") (name "Rational") (declared-name "Rational") (range (start (line 17) (character 4)) (end (line 17) (character 39))) (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::Real"))) (kind "kermlDecl") (name "Real") (declared-name "Real") (range (start (line 16) (character 4)) (end (line 16) (character 38))) (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::ScalarValue"))) (kind "kermlDecl") (name "ScalarValue") (declared-name "ScalarValue") (range (start (line 9) (character 4)) (end (line 9) (character 56))) (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::String"))) (kind "kermlDecl") (name "String") (declared-name "String") (range (start (line 11) (character 4)) (end (line 11) (character 44))) (parent (node (document "d0") (qualified-name "ScalarValues"))))
    (element (id (node (document "d0") (qualified-name "ScalarValues::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 887))) (parent (node (document "d0") (qualified-name "ScalarValues"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ScalarValues::DataValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::DataValue") (range (start (line 7) (character 19)) (end (line 7) (character 34))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
