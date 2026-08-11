# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_03-Value Expression
type=file
~~~
# SOURCE
~~~sysml
package '15_03-Value Expression' {
    private import SI::*;
    private import USCustomaryUnits::*;

    part def Vehicle_1 {
        attribute mass: MassValue = 1200 [kg];
        attribute length: LengthValue = 4.82 [m];
        part leftFrontWheel : Wheel;
        part rightFrontWheel : Wheel;
    }

    part def Wheel {
    	attribute hubDiameter: LengthValue = 18 ['in'];
        attribute width: LengthValue = 245 [mm];
        attribute outerDiameter: LengthValue = (hubDiameter + 2 * tire.height) [mm] {
	        doc
	        /*
	         * This binds 'outDiameter' to the result of a computed attribute.
	         * There is no need to mark it as "derived".
	         */
        }
        part tire: Tire[1];
    }
    
    part def Tire {
    	attribute profileDepth: LengthValue default 6.0 [mm];
        constraint hasLegalProfileDepth {profileDepth >= 3.5 [mm]}
    	attribute height: LengthValue = 45 [mm];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_03_value_expression.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 35))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 5 8) (end 5 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 8) (end 5 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 24) (end 5 33))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 6 8) (end 6 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 8) (end 6 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 26) (end 6 37))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 12 5) (end 12 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 5) (end 12 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 28) (end 12 39))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 13 8) (end 13 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 8) (end 13 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 25) (end 13 36))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 14 8) (end 14 263))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 8) (end 14 263))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 33) (end 14 44))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 25 5) (end 25 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 5) (end 25 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 29) (end 25 40))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 27 5) (end 27 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 5) (end 27 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 23) (end 27 34))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "08101b8b2207cd146a49d671684636f3aca50fdb0e2e39ff50d5c0c7417b8155") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression"))) (kind "package") (name "15_03-Value Expression") (declared-name "15_03-Value Expression"))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_03-Value Expression"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_03-Value Expression"))) (authored (membership (kind Import) (visibility "private") (import (reference "USCustomaryUnits::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Tire"))) (kind "part def") (name "Tire") (declared-name "Tire") (parent (node (document "d0") (qualified-name "15_03-Value Expression"))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Tire::height"))) (kind "attribute") (name "height") (declared-name "height") (parent (node (document "d0") (qualified-name "15_03-Value Expression::Tire"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Tire::profileDepth"))) (kind "attribute") (name "profileDepth") (declared-name "profileDepth") (parent (node (document "d0") (qualified-name "15_03-Value Expression::Tire"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1"))) (kind "part def") (name "Vehicle_1") (declared-name "Vehicle_1") (parent (node (document "d0") (qualified-name "15_03-Value Expression"))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel"))) (kind "part") (name "leftFrontWheel") (declared-name "leftFrontWheel") (parent (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::length"))) (kind "attribute") (name "length") (declared-name "length") (parent (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel"))) (kind "part") (name "rightFrontWheel") (declared-name "rightFrontWheel") (parent (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "15_03-Value Expression"))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::hubDiameter"))) (kind "attribute") (name "hubDiameter") (declared-name "hubDiameter") (parent (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (kind "attribute") (name "outerDiameter") (declared-name "outerDiameter") (parent (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::tire"))) (kind "part") (name "tire") (declared-name "tire") (parent (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Tire")))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::width"))) (kind "attribute") (name "width") (declared-name "width") (parent (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (typing (reference "LengthValue")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "USCustomaryUnits::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Tire::height"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Tire::height"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Tire::profileDepth"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Tire::profileDepth"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_03-Value Expression::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::length"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::length"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_03-Value Expression::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::hubDiameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::hubDiameter"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::tire"))) (kind featureTyping) (ordinal 0)) (authored-target "Tire") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_03-Value Expression::Tire")))))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::width"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::width"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel"))) (target (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel"))) (target (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::tire"))) (target (node (document "d0") (qualified-name "15_03-Value Expression::Tire"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::tire"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "15_03-Value Expression::Tire::height")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_03-Value Expression::Tire::profileDepth")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::length")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::hubDiameter")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::outerDiameter")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::width")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 19) (end 1 21)) (probe (position 1 19))
      (reference
        (source (document "d0") (qualified-name "15_03-Value Expression::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 1 19) (end 1 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 19) (end 21 23)) (probe (position 21 19))
      (reference
        (source (document "d0") (qualified-name "15_03-Value Expression::Wheel::tire"))
        (kind featureTyping) (ordinal 0) (authored-target "Tire")
        (range (start 21 19) (end 21 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_03-Value Expression::Tire") (range (start 24 4) (end 24 197)))
        )
      )
    )
    (query (range (start 7 30) (end 7 35)) (probe (position 7 30))
      (reference
        (source (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 7 30) (end 7 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_03-Value Expression::Wheel") (range (start 11 4) (end 11 420)))
        )
      )
    )
    (query (range (start 8 31) (end 8 36)) (probe (position 8 31))
      (reference
        (source (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 8 31) (end 8 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_03-Value Expression::Wheel") (range (start 11 4) (end 11 420)))
        )
      )
    )
    (query (range (start 5 24) (end 5 33)) (probe (position 5 24))
      (reference
        (source (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::mass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 5 24) (end 5 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 26) (end 6 37)) (probe (position 6 26))
      (reference
        (source (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::length"))
        (kind featureTyping) (ordinal 1) (authored-target "LengthValue")
        (range (start 6 26) (end 6 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 28) (end 12 39)) (probe (position 12 28))
      (reference
        (source (document "d0") (qualified-name "15_03-Value Expression::Wheel::hubDiameter"))
        (kind featureTyping) (ordinal 1) (authored-target "LengthValue")
        (range (start 12 28) (end 12 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 25) (end 13 36)) (probe (position 13 25))
      (reference
        (source (document "d0") (qualified-name "15_03-Value Expression::Wheel::width"))
        (kind featureTyping) (ordinal 1) (authored-target "LengthValue")
        (range (start 13 25) (end 13 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 33) (end 14 44)) (probe (position 14 33))
      (reference
        (source (document "d0") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))
        (kind featureTyping) (ordinal 1) (authored-target "LengthValue")
        (range (start 14 33) (end 14 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 25 29) (end 25 40)) (probe (position 25 29))
      (reference
        (source (document "d0") (qualified-name "15_03-Value Expression::Tire::profileDepth"))
        (kind featureTyping) (ordinal 1) (authored-target "LengthValue")
        (range (start 25 29) (end 25 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 27 23) (end 27 34)) (probe (position 27 23))
      (reference
        (source (document "d0") (qualified-name "15_03-Value Expression::Tire::height"))
        (kind featureTyping) (ordinal 1) (authored-target "LengthValue")
        (range (start 27 23) (end 27 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 19) (end 2 35)) (probe (position 2 19))
      (reference
        (source (document "d0") (qualified-name "15_03-Value Expression::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "USCustomaryUnits::*")
        (range (start 2 19) (end 2 35))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
