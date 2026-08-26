# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Data Type Library/VectorValues
type=file
~~~
# SOURCE
~~~kerml
standard library package VectorValues {
    doc /*
	 * This package provides a basic model of abstract vectors as well as concrete vectors
	 * whose components are numerical values. The package VectorFunctions defines the 
	 * corresponding vector-space functions.
	 */

    private import ScalarValues::NumericalValue;
    private import ScalarValues::Real;
    private import Collections::Array;

    abstract datatype VectorValue {
        doc /*
		 * A VectorValue is an abstract data type whose values may be operated on using
		 * VectorFunctions.
		 */
    }

    datatype NumericalVectorValue :> VectorValue, Array intersects VectorValue, Array {
        doc /*
		 * A NumericalVectorValue is a kind of VectorValue that is specifically represented
		 * as a one-dimensional array of NumericalValues. The dimension is allowed to be empty,
		 * permitting a NumericalVectorValue of rank 0, which is essentially isomorphic to a
		 * scalar NumericalValue.
		 */

        feature dimension[0..1] :>> dimensions;
        feature :>> elements : NumericalValue;
    }

    datatype CartesianVectorValue :> NumericalVectorValue {
        doc /*
		 * CartesianVectorValue is a specialization Numerical VectorValue for which there are 
		 * specific implementations in VectorFunctions of the abstract vector-space functions.
		 * 
		 * Note: The restriction of the element type to Real is to facilitate
		 * the complete definition of these functions.
		 */

        feature :>> elements : Real;
    }

    datatype ThreeVectorValue :> NumericalVectorValue {
        doc /*
		 * A ThreeVectorValue is a NumericalVectorValue that has dimension 3.
		 */

        feature :>> dimension = 3;
    }

    datatype CartesianThreeVectorValue :> CartesianVectorValue, ThreeVectorValue intersects CartesianVectorValue, ThreeVectorValue {
        doc /*
		 * A CartesianThreeVectorValue is a NumericalVectorValue that is both Cartesian and
		 * has dimension 3.
		 */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/vector_values.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 7 19) (end 7 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 19) (end 8 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 19) (end 9 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 18 50) (end 18 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 80) (end 18 85))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 36) (end 26 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 20) (end 27 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 31) (end 27 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 20) (end 39 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 31) (end 39 35))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:5530a7186e5bb056a73150c5f4a5b6c31ee462e1423092defdf988a90e351861") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package provides a basic model of abstract vectors as well as concrete vectors\n\t * whose components are numerical values. The package VectorFunctions defines the \n\t * corresponding vector-space functions.\n\t "))))
    (declaration (id (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::NumericalValue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Collections::Array") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A CartesianThreeVectorValue is a NumericalVectorValue that is both Cartesian and\n\t\t * has dimension 3.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CartesianVectorValue")) (specialization (reference "ThreeVectorValue")) (intersecting (reference "CartesianVectorValue")) (intersecting (reference "ThreeVectorValue")))))
    (declaration (id (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * CartesianVectorValue is a specialization Numerical VectorValue for which there are \n\t\t * specific implementations in VectorFunctions of the abstract vector-space functions.\n\t\t * \n\t\t * Note: The restriction of the element type to Real is to facilitate\n\t\t * the complete definition of these functions.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalVectorValue")))))
    (declaration (id (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "CartesianVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "elements")))))
    (declaration (id (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A NumericalVectorValue is a kind of VectorValue that is specifically represented\n\t\t * as a one-dimensional array of NumericalValues. The dimension is allowed to be empty,\n\t\t * permitting a NumericalVectorValue of rank 0, which is essentially isomorphic to a\n\t\t * scalar NumericalValue.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorValue")) (specialization (reference "Array")) (intersecting (reference "VectorValue")) (intersecting (reference "Array")))))
    (declaration (id (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "NumericalVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue")) (redefinition (reference "elements")))))
    (declaration (id (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue::dimension"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "dimensions")))))
    (declaration (id (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A ThreeVectorValue is a NumericalVectorValue that has dimension 3.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "NumericalVectorValue")))))
    (declaration (id (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "dimension")))))
    (declaration (id (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::VectorValue"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * A VectorValue is an abstract data type whose values may be operated on using\n\t\t * VectorFunctions.\n\t\t "))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Collections::Array")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (kind specialization) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue")))))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (kind specialization) (ordinal 1))
      (authored-target "ThreeVectorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue")))))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (kind intersecting) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue")))))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (kind intersecting) (ordinal 1))
      (authored-target "ThreeVectorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue")))))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalVectorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")))))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "CartesianVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "CartesianVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (kind specialization) (ordinal 0))
      (authored-target "VectorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::VectorValue")))))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (kind specialization) (ordinal 1))
      (authored-target "Array")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (kind intersecting) (ordinal 0))
      (authored-target "VectorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::VectorValue")))))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (kind intersecting) (ordinal 1))
      (authored-target "Array")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "NumericalVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "NumericalVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue::dimension"))) (kind redefinition) (ordinal 0))
      (authored-target "dimensions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue"))) (kind specialization) (ordinal 0))
      (authored-target "NumericalVectorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")))))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "dimension")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue::dimension")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (kind specialization) (ordinal 1)))
    (relationship (kind intersecting) (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (kind intersecting) (ordinal 0)))
    (relationship (kind intersecting) (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (kind intersecting) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue"))) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::VectorValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (kind specialization) (ordinal 0)))
    (relationship (kind intersecting) (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::VectorValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (kind intersecting) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue"))) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue::dimension"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "CartesianVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "NumericalVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue::dimension"))) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 3)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue")))
      (set-operand (operator intersection) (ordinal 0) (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue")))
      (set-operand (operator intersection) (ordinal 1) (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue")))
      (supertype (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue")))
      (supertype (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "CartesianVectorValue")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue")))
    )
    (declaration (id (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")))
      (set-operand (operator intersection) (ordinal 0) (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::VectorValue")))
      (supertype (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "NumericalVectorValue")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")))
    )
    (declaration (id (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue::dimension")))
      (featured-by (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")))
      (subtype (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue")))
      (supertype (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue")))
      (supertype (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue::dimension")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::VectorValue")))
      (subtype (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/vector_values.md") (range (start 7 19) (end 7 47)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::NumericalValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 8 19) (end 8 37)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 9 19) (end 9 37)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Collections::Array")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 50 42) (end 50 62)) (probe (position 50 42))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (kind specialization) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue")))))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 50 64) (end 50 80)) (probe (position 50 64))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (kind specialization) (ordinal 1) (authored-target "ThreeVectorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue")))))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 50 92) (end 50 112)) (probe (position 50 92))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (kind intersecting) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue")))))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 50 114) (end 50 130)) (probe (position 50 114))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianThreeVectorValue"))) (kind intersecting) (ordinal 1) (authored-target "ThreeVectorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue")))))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 30 37) (end 30 57)) (probe (position 30 37))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::CartesianVectorValue"))) (kind specialization) (ordinal 0) (authored-target "NumericalVectorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")))))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 39 31) (end 39 35)) (probe (position 39 31))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "CartesianVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 39 20) (end 39 28)) (probe (position 39 20))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "CartesianVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 18 37) (end 18 48)) (probe (position 18 37))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (kind specialization) (ordinal 0) (authored-target "VectorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::VectorValue")))))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 18 50) (end 18 55)) (probe (position 18 50))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (kind specialization) (ordinal 1) (authored-target "Array")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 18 67) (end 18 78)) (probe (position 18 67))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (kind intersecting) (ordinal 0) (authored-target "VectorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::VectorValue")))))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 18 80) (end 18 85)) (probe (position 18 80))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue"))) (kind intersecting) (ordinal 1) (authored-target "Array")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 27 31) (end 27 45)) (probe (position 27 31))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "NumericalVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 27 20) (end 27 28)) (probe (position 27 20))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "NumericalVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 26 36) (end 26 46)) (probe (position 26 36))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue::dimension"))) (kind redefinition) (ordinal 0) (authored-target "dimensions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 42 33) (end 42 53)) (probe (position 42 33))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::ThreeVectorValue"))) (kind specialization) (ordinal 0) (authored-target "NumericalVectorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")))))
    )
  )
  (query (document "memory://snapshot/vector_values.md") (range (start 47 20) (end 47 29)) (probe (position 47 20))
    (reference (id (source (node (document "memory://snapshot/vector_values.md") (path (named (kind library-package) (name "VectorValues")) (named (kind kerml-datatype) (name "ThreeVectorValue")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "dimension")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue::dimension")))))
    )
  )
)
~~~
