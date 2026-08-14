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
  (document "memory://snapshot/scalar_values.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 9 46) (end 9 55))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2da3f0f882de2304c481cc3bc44d33be9421f90ab98191373b36d093aa90d5bf") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package contains a basic set of primitive scalar (non-collection) data types. \n\t * These include Boolean and String types and a hierarchy of concrete Number types, from \n\t * the most general type of Complex numbers to the most specific type of Positive integers.</p>\n\t "))))
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (path (named (kind library-package) (name "ScalarValues")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::DataValue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Boolean"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarValue")))))
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Complex"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Number")))))
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Integer"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Rational")))))
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Natural"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Number"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalValue")))))
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarValue")))))
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Positive"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Natural")))))
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Rational"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Real"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Complex")))))
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataValue")))))
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::String"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarValue")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (path (named (kind library-package) (name "ScalarValues")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::DataValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Boolean"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")))))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Complex"))) (kind specialization) (ordinal 0))
      (authored-target "Number")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Number")))))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Integer"))) (kind specialization) (ordinal 0))
      (authored-target "Rational")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Rational")))))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Natural"))) (kind specialization) (ordinal 0))
      (authored-target "Integer")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Integer")))))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Number"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")))))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")))))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Positive"))) (kind specialization) (ordinal 0))
      (authored-target "Natural")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Natural")))))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Rational"))) (kind specialization) (ordinal 0))
      (authored-target "Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Real"))) (kind specialization) (ordinal 0))
      (authored-target "Complex")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Complex")))))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue"))) (kind specialization) (ordinal 0))
      (authored-target "DataValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::String"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Boolean"))) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Boolean"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Complex"))) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Number"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Complex"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Integer"))) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Rational"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Integer"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Natural"))) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Integer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Natural"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Number"))) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Number"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue"))) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Positive"))) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Natural"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Positive"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Rational"))) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Real"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Rational"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Real"))) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Complex"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Real"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::String"))) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::String"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Boolean")))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Complex")))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Integer")))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Natural")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Natural")))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Positive")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Number")))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Positive")))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Natural")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Rational")))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Real")))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")))
      (subtype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::String")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::String")))
      (supertype (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/scalar_values.md") (range (start 7 19) (end 7 34)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (path (named (kind library-package) (name "ScalarValues")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Base::DataValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/scalar_values.md") (range (start 10 33) (end 10 44)) (probe (position 10 33))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Boolean"))) (kind specialization) (ordinal 0) (authored-target "ScalarValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")))))
    )
  )
  (query (document "memory://snapshot/scalar_values.md") (range (start 15 33) (end 15 39)) (probe (position 15 33))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Complex"))) (kind specialization) (ordinal 0) (authored-target "Number")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Number")))))
    )
  )
  (query (document "memory://snapshot/scalar_values.md") (range (start 18 33) (end 18 41)) (probe (position 18 33))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Integer"))) (kind specialization) (ordinal 0) (authored-target "Rational")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Rational")))))
    )
  )
  (query (document "memory://snapshot/scalar_values.md") (range (start 19 33) (end 19 40)) (probe (position 19 33))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Natural"))) (kind specialization) (ordinal 0) (authored-target "Integer")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Integer")))))
    )
  )
  (query (document "memory://snapshot/scalar_values.md") (range (start 14 41) (end 14 55)) (probe (position 14 41))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Number"))) (kind specialization) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")))))
    )
  )
  (query (document "memory://snapshot/scalar_values.md") (range (start 12 49) (end 12 60)) (probe (position 12 49))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::NumericalValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")))))
    )
  )
  (query (document "memory://snapshot/scalar_values.md") (range (start 20 34) (end 20 41)) (probe (position 20 34))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Positive"))) (kind specialization) (ordinal 0) (authored-target "Natural")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Natural")))))
    )
  )
  (query (document "memory://snapshot/scalar_values.md") (range (start 17 34) (end 17 38)) (probe (position 17 34))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Rational"))) (kind specialization) (ordinal 0) (authored-target "Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    )
  )
  (query (document "memory://snapshot/scalar_values.md") (range (start 16 30) (end 16 37)) (probe (position 16 30))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Real"))) (kind specialization) (ordinal 0) (authored-target "Complex")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::Complex")))))
    )
  )
  (query (document "memory://snapshot/scalar_values.md") (range (start 9 46) (end 9 55)) (probe (position 9 46))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue"))) (kind specialization) (ordinal 0) (authored-target "DataValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/scalar_values.md") (range (start 11 32) (end 11 43)) (probe (position 11 32))
    (reference (id (source (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::String"))) (kind specialization) (ordinal 0) (authored-target "ScalarValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")))))
    )
  )
)
~~~
