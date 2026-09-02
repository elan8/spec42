# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_01-Constants
type=file
~~~
# SOURCE
~~~sysml
package '15_01-Constants' {
    private import MeasurementReferences::*;
    private import SI::*;
    private import RealFunctions::*;

    /* Note: Value properties that are bound to specific values are constants and have the specified
     * values in all contexts. It is not legal to redefine them.
     */    
    
    package 'Mathematical Constants' {
	    doc
	    /*
	     * Standard mathematical constants
	     * 
	     * Irrational constants cannot be represented exactly with finite precision.
	     * However, they can be required to be implemented with a attribute that is accurate
	     * to at least a certain precision.
	     * 
	     * (The decimal literals here should be interpreted as being fixed point and exact.)
	     */
    
        attribute e: Real {
        	assert constraint { round(e * 1E20) == 271828182845904523536.0 }
        }
        attribute pi: Real {
        	assert constraint { round(pi * 1E20) == 314159265358979323846.0 }
        }
    }

    package 'Fundamental Physical Constants' {
	    doc
	    /*
	     * Standard fundamental physical constants
	     * 
	     * Physical constants have a standard measured attribute to a finite precision.
	     *
	     * The reference source is:
	     * CODATA - Task Group on Fundamental Physical Constants (TGFC) - 2018 CODATA recommended values
	     * See https://codata.org/initiatives/strategic-programme/fundamental-physical-constants/
	     * For the actual values see https://pml.nist.gov/cuu/Constants/ 
	     */
    
        attribute 'fine structure constant'      : DimensionOneValue = 7.2973525693E-3[one];  // 2018 CODATA attribute 7.2973525693E-3;  uncertainty = 0.0000000011E-3
        attribute 'electron to proton mass ratio': DimensionOneValue = 5.44617021487E-4[one]; // 2018 CODATA attribute 5.44617021487E-4; uncertainty = 0.00000000033E-4 
        attribute 'speed of light in vacuum'     : SpeedValue = 299792458[m/s];               // 2018 CODATA attribute 299792458 m s^-1; (exact)
     }

    package 'Global Context' {
        attribute 'nominal earth gravitational acceleration': AccelerationValue = 9.80665['m/s²'];
    }

    package 'Model X Context' {
        attribute 'amplifier gain': DimensionOneValue = 3.5[one];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/15_01_constants.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 19) (end 1 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 19) (end 3 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 21) (end 21 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 29) (end 22 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 22) (end 24 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 29) (end 25 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 51) (end 42 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 51) (end 43 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 44 51) (end 44 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 48 62) (end 48 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 36) (end 52 53))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:d17b05dafeaf1d07cf2bfddb67af28eab670e80ae3346cb8d689651d39c65ba4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants"))) (kind package) (membership (kind owning) (visibility default)) (documentation (comment (text " Note: Value properties that are bound to specific values are constants and have the specified\n     * values in all contexts. It is not legal to redefine them.\n     "))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "MeasurementReferences") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RealFunctions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Fundamental Physical Constants"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t     * Standard fundamental physical constants\n\t     * \n\t     * Physical constants have a standard measured attribute to a finite precision.\n\t     *\n\t     * The reference source is:\n\t     * CODATA - Task Group on Fundamental Physical Constants (TGFC) - 2018 CODATA recommended values\n\t     * See https://codata.org/initiatives/strategic-programme/fundamental-physical-constants/\n\t     * For the actual values see https://pml.nist.gov/cuu/Constants/ \n\t     "))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Fundamental Physical Constants::electron to proton mass ratio"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "electron to proton mass ratio")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "electron to proton mass ratio")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DimensionOneValue")))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "electron to proton mass ratio")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "electron to proton mass ratio")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "electron to proton mass ratio")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Fundamental Physical Constants::fine structure constant"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "fine structure constant")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "fine structure constant")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DimensionOneValue")))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "fine structure constant")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "fine structure constant")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "fine structure constant")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Fundamental Physical Constants::speed of light in vacuum"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "speed of light in vacuum")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "speed of light in vacuum")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue")))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "speed of light in vacuum")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "speed of light in vacuum")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "speed of light in vacuum")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Global Context"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Global Context::nominal earth gravitational acceleration"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Global Context")) (named (kind attribute) (name "nominal earth gravitational acceleration")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Global Context")) (named (kind attribute) (name "nominal earth gravitational acceleration")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AccelerationValue")))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Global Context")) (named (kind attribute) (name "nominal earth gravitational acceleration")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Global Context")) (named (kind attribute) (name "nominal earth gravitational acceleration")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Global Context")) (named (kind attribute) (name "nominal earth gravitational acceleration")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t     * Standard mathematical constants\n\t     * \n\t     * Irrational constants cannot be represented exactly with finite precision.\n\t     * However, they can be required to be implemented with a attribute that is accurate\n\t     * to at least a certain precision.\n\t     * \n\t     * (The decimal literals here should be interpreted as being fixed point and exact.)\n\t     "))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants::e"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "e")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind assert-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "e")) (invocationCallee (reference "round")))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants::pi"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "pi")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind assert-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "pi")) (invocationCallee (reference "round")))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Model X Context"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Model X Context::amplifier gain"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Model X Context")) (named (kind attribute) (name "amplifier gain")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Model X Context")) (named (kind attribute) (name "amplifier gain")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DimensionOneValue")))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Model X Context")) (named (kind attribute) (name "amplifier gain")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Model X Context")) (named (kind attribute) (name "amplifier gain")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Model X Context")) (named (kind attribute) (name "amplifier gain")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RealFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Fundamental Physical Constants::electron to proton mass ratio"))) (kind featureTyping) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Fundamental Physical Constants::fine structure constant"))) (kind featureTyping) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Fundamental Physical Constants::speed of light in vacuum"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Global Context::nominal earth gravitational acceleration"))) (kind featureTyping) (ordinal 0))
      (authored-target "AccelerationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants::e"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "e")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "e")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants::e")))))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "e")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "round")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants::pi"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "pi")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "pi")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants::pi")))))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "pi")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "round")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Model X Context::amplifier gain"))) (kind featureTyping) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "e")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants::e"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "e")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "pi")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants::pi"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "pi")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "electron to proton mass ratio")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "electron to proton mass ratio")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "fine structure constant")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "fine structure constant")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "speed of light in vacuum")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "speed of light in vacuum")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Global Context")) (named (kind attribute) (name "nominal earth gravitational acceleration")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Global Context")) (named (kind attribute) (name "nominal earth gravitational acceleration")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "e")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants::e"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "pi")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants::pi"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Model X Context")) (named (kind attribute) (name "amplifier gain")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Model X Context")) (named (kind attribute) (name "amplifier gain")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "electron to proton mass ratio")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind real) (real 0.000544617021487))) (unit "one")))
    (evaluated (declaration (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "fine structure constant")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind real) (real 0.0072973525693))) (unit "one")))
    (evaluated (declaration (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "speed of light in vacuum")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Global Context")) (named (kind attribute) (name "nominal earth gravitational acceleration")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind real) (real 9.80665))) (unit "m/s²")))
    (evaluated (declaration (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "e")) (anonymous (kind assert-constraint) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "pi")) (anonymous (kind assert-constraint) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Model X Context")) (named (kind attribute) (name "amplifier gain")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind real) (real 3.5))) (unit "one")))
    (unit (declaration (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "electron to proton mass ratio")) (anonymous (kind kerml-expression) (ordinal 0))))) (ordinal 0) (authored "one") (start 43 88) (end 43 91) (outcome (status catalog-unavailable)))
    (unit (declaration (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "fine structure constant")) (anonymous (kind kerml-expression) (ordinal 0))))) (ordinal 0) (authored "one") (start 42 87) (end 42 90) (outcome (status catalog-unavailable)))
    (unit (declaration (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Global Context")) (named (kind attribute) (name "nominal earth gravitational acceleration")) (anonymous (kind kerml-expression) (ordinal 0))))) (ordinal 0) (authored "m/s²") (start 48 90) (end 48 97) (outcome (status catalog-unavailable)))
    (unit (declaration (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Model X Context")) (named (kind attribute) (name "amplifier gain")) (anonymous (kind kerml-expression) (ordinal 0))))) (ordinal 0) (authored "one") (start 52 60) (end 52 63) (outcome (status catalog-unavailable)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "electron to proton mass ratio")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "electron to proton mass ratio")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "fine structure constant")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "fine structure constant")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "speed of light in vacuum")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Fundamental Physical Constants")) (named (kind attribute) (name "speed of light in vacuum")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Global Context")) (named (kind attribute) (name "nominal earth gravitational acceleration")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Global Context")) (named (kind attribute) (name "nominal earth gravitational acceleration")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "e")) (anonymous (kind assert-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants::e")))
    )
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "pi")) (anonymous (kind assert-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants::pi")))
    )
    (declaration (id (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Model X Context")) (named (kind attribute) (name "amplifier gain")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Model X Context")) (named (kind attribute) (name "amplifier gain")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/15_01_constants.md") (range (start 1 19) (end 1 43)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_01_constants.md") (range (start 2 19) (end 2 24)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_01_constants.md") (range (start 3 19) (end 3 35)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "RealFunctions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_01_constants.md") (range (start 43 51) (end 43 68)) (probe (position 43 51))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Fundamental Physical Constants::electron to proton mass ratio"))) (kind featureTyping) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_01_constants.md") (range (start 42 51) (end 42 68)) (probe (position 42 51))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Fundamental Physical Constants::fine structure constant"))) (kind featureTyping) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_01_constants.md") (range (start 44 51) (end 44 61)) (probe (position 44 51))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Fundamental Physical Constants::speed of light in vacuum"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_01_constants.md") (range (start 48 62) (end 48 79)) (probe (position 48 62))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Global Context::nominal earth gravitational acceleration"))) (kind featureTyping) (ordinal 0) (authored-target "AccelerationValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_01_constants.md") (range (start 21 21) (end 21 25)) (probe (position 21 21))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants::e"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_01_constants.md") (range (start 22 35) (end 22 36)) (probe (position 22 35))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "e")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "e")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants::e")))))
    )
  )
  (query (document "memory://snapshot/15_01_constants.md") (range (start 22 29) (end 22 34)) (probe (position 22 29))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "e")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "round")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_01_constants.md") (range (start 24 22) (end 24 26)) (probe (position 24 22))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants::pi"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_01_constants.md") (range (start 25 35) (end 25 37)) (probe (position 25 35))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "pi")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "pi")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Mathematical Constants::pi")))))
    )
  )
  (query (document "memory://snapshot/15_01_constants.md") (range (start 25 29) (end 25 34)) (probe (position 25 29))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (path (named (kind package) (name "15_01-Constants")) (named (kind package) (name "Mathematical Constants")) (named (kind attribute) (name "pi")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "round")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_01_constants.md") (range (start 52 36) (end 52 53)) (probe (position 52 36))
    (reference (id (source (node (document "memory://snapshot/15_01_constants.md") (qualified-name "15_01-Constants::Model X Context::amplifier gain"))) (kind featureTyping) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    )
  )
)
~~~
