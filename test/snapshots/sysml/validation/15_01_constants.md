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
  (document "15_01_constants.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 19) (end 3 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 8) (end 21 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 8) (end 24 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 8) (end 42 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 8) (end 43 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 44 8) (end 44 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 48 8) (end 48 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 8) (end 52 65))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "33849a5fbc0f6a1a5d5104de14d6a75881886c80e1f8e89df63e1a00be7bcaa1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_01-Constants"))) (kind "package") (name "15_01-Constants") (declared-name "15_01-Constants"))
    (element (id (node (document "d0") (qualified-name "15_01-Constants::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_01-Constants"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_01-Constants::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_01-Constants"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_01-Constants::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_01-Constants"))) (authored (membership (kind Import) (visibility "private") (import (reference "RealFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants"))) (kind "package") (name "Fundamental Physical Constants") (declared-name "Fundamental Physical Constants") (parent (node (document "d0") (qualified-name "15_01-Constants"))))
    (element (id (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants"))))
    (element (id (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::electron to proton mass ratio"))) (kind "attribute def") (name "electron to proton mass ratio") (declared-name "electron to proton mass ratio") (parent (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::fine structure constant"))) (kind "attribute def") (name "fine structure constant") (declared-name "fine structure constant") (parent (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::speed of light in vacuum"))) (kind "attribute def") (name "speed of light in vacuum") (declared-name "speed of light in vacuum") (parent (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "15_01-Constants::Global Context"))) (kind "package") (name "Global Context") (declared-name "Global Context") (parent (node (document "d0") (qualified-name "15_01-Constants"))))
    (element (id (node (document "d0") (qualified-name "15_01-Constants::Global Context::nominal earth gravitational acceleration"))) (kind "attribute def") (name "nominal earth gravitational acceleration") (declared-name "nominal earth gravitational acceleration") (parent (node (document "d0") (qualified-name "15_01-Constants::Global Context"))) (authored (membership (kind Owning)) (relationships (typing (reference "AccelerationValue")))))
    (element (id (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants"))) (kind "package") (name "Mathematical Constants") (declared-name "Mathematical Constants") (parent (node (document "d0") (qualified-name "15_01-Constants"))))
    (element (id (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants"))))
    (element (id (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants::e"))) (kind "attribute def") (name "e") (declared-name "e") (parent (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants::pi"))) (kind "attribute def") (name "pi") (declared-name "pi") (parent (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "15_01-Constants::Model X Context"))) (kind "package") (name "Model X Context") (declared-name "Model X Context") (parent (node (document "d0") (qualified-name "15_01-Constants"))))
    (element (id (node (document "d0") (qualified-name "15_01-Constants::Model X Context::amplifier gain"))) (kind "attribute def") (name "amplifier gain") (declared-name "amplifier gain") (parent (node (document "d0") (qualified-name "15_01-Constants::Model X Context"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_01-Constants::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_01-Constants::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_01-Constants::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "RealFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::electron to proton mass ratio"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::fine structure constant"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::speed of light in vacuum"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_01-Constants::Global Context::nominal earth gravitational acceleration"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants::e"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants::pi"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_01-Constants::Model X Context::amplifier gain"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (node (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::electron to proton mass ratio")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::fine structure constant")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::speed of light in vacuum")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_01-Constants::Global Context::nominal earth gravitational acceleration")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_01-Constants::Model X Context::amplifier gain")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 19) (end 2 21)) (probe (position 2 19))
      (reference
        (source (document "d0") (qualified-name "15_01-Constants::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 2 19) (end 2 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 19) (end 3 32)) (probe (position 3 19))
      (reference
        (source (document "d0") (qualified-name "15_01-Constants::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "RealFunctions::*")
        (range (start 3 19) (end 3 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 19) (end 1 40)) (probe (position 1 19))
      (reference
        (source (document "d0") (qualified-name "15_01-Constants::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences::*")
        (range (start 1 19) (end 1 40))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
