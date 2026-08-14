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
  (document "memory://snapshot/15_03_value_expression.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 24) (end 5 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 26) (end 6 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 28) (end 12 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 25) (end 13 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 33) (end 14 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 29) (end 25 40))
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
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:ab6e6b6a6cd6a866eb38d6f8518d8a43a43df59027973ba7299092fb8c8c5b88") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (path (named (kind package) (name "15_03-Value Expression")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (path (named (kind package) (name "15_03-Value Expression")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "USCustomaryUnits") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::hasLegalProfileDepth"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "profileDepth")))))
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::height"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")))))
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::profileDepth"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (default true) (operator false)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")))))
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::length"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")))))
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")))))
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::hubDiameter"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")))))
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower expression) (upper expression))) (documentation (doc (text "\n\t         * This binds 'outDiameter' to the result of a computed attribute.\n\t         * There is no need to mark it as \"derived\".\n\t         "))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")) (expressionOperand (reference "hubDiameter")) (memberAccessOperand (reference "tire::height")))))
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::tire"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Tire")))))
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::width"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (path (named (kind package) (name "15_03-Value Expression")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (path (named (kind package) (name "15_03-Value Expression")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "USCustomaryUnits")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::hasLegalProfileDepth"))) (kind expressionOperand) (ordinal 0))
      (authored-target "profileDepth")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::profileDepth")))))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::height"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::profileDepth"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::length"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::hubDiameter"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (kind expressionOperand) (ordinal 0))
      (authored-target "hubDiameter")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::hubDiameter")))))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "tire::height")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::height")))))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::tire"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tire")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire")))))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::width"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::hasLegalProfileDepth"))) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::profileDepth"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::hasLegalProfileDepth"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel"))) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel"))) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::hubDiameter"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::height"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::tire"))) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::tire"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::hasLegalProfileDepth"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::height"))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 45))) (unit "mm")))
    (evaluated (declaration (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::profileDepth"))) (state literal) (value (kind quantity) (magnitude (value (kind real) (real 6))) (unit "mm")))
    (evaluated (declaration (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::length"))) (state literal) (value (kind quantity) (magnitude (value (kind real) (real 4.82))) (unit "m")))
    (evaluated (declaration (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::mass"))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 1200))) (unit "kg")))
    (evaluated (declaration (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::hubDiameter"))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 18))) (unit "in")))
    (evaluated (declaration (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::width"))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 245))) (unit "mm")))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire")))
      (subtype (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::tire")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::hasLegalProfileDepth")))
      (featured-by (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire")))
    )
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::height")))
      (featured-by (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire")))
    )
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::profileDepth")))
      (featured-by (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire")))
    )
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel")))
      (featured-by (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1")))
      (type (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel")) (source direct))
      (supertype (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::length")))
      (featured-by (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1")))
    )
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::mass")))
      (featured-by (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1")))
    )
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel")))
      (featured-by (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1")))
      (type (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel")) (source direct))
      (supertype (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel")))
      (subtype (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel")) (scopes any))
      (subtype (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::hubDiameter")))
      (featured-by (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel")))
    )
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::outerDiameter")))
      (featured-by (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel")))
    )
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::tire")))
      (featured-by (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel")))
      (type (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire")) (provenance authored))
      (effective-type (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire")) (source direct))
      (supertype (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::width")))
      (featured-by (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/15_03_value_expression.md") (range (start 1 19) (end 1 24)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (path (named (kind package) (name "15_03-Value Expression")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_03_value_expression.md") (range (start 2 19) (end 2 38)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (path (named (kind package) (name "15_03-Value Expression")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "USCustomaryUnits")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_03_value_expression.md") (range (start 26 41) (end 26 53)) (probe (position 26 41))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::hasLegalProfileDepth"))) (kind expressionOperand) (ordinal 0) (authored-target "profileDepth")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::profileDepth")))))
    )
  )
  (query (document "memory://snapshot/15_03_value_expression.md") (range (start 27 23) (end 27 34)) (probe (position 27 23))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::height"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_03_value_expression.md") (range (start 25 29) (end 25 40)) (probe (position 25 29))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::profileDepth"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_03_value_expression.md") (range (start 7 30) (end 7 35)) (probe (position 7 30))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel")))))
    )
  )
  (query (document "memory://snapshot/15_03_value_expression.md") (range (start 6 26) (end 6 37)) (probe (position 6 26))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::length"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_03_value_expression.md") (range (start 5 24) (end 5 33)) (probe (position 5 24))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_03_value_expression.md") (range (start 8 31) (end 8 36)) (probe (position 8 31))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel")))))
    )
  )
  (query (document "memory://snapshot/15_03_value_expression.md") (range (start 12 28) (end 12 39)) (probe (position 12 28))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::hubDiameter"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_03_value_expression.md") (range (start 14 33) (end 14 44)) (probe (position 14 33))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_03_value_expression.md") (range (start 14 48) (end 14 59)) (probe (position 14 48))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (kind expressionOperand) (ordinal 0) (authored-target "hubDiameter")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::hubDiameter")))))
    )
  )
  (query (document "memory://snapshot/15_03_value_expression.md") (range (start 14 66) (end 14 77)) (probe (position 14 66))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (kind memberAccessOperand) (ordinal 0) (authored-target "tire::height")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire::height")))))
    )
  )
  (query (document "memory://snapshot/15_03_value_expression.md") (range (start 21 19) (end 21 23)) (probe (position 21 19))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::tire"))) (kind featureTyping) (ordinal 0) (authored-target "Tire")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Tire")))))
    )
  )
  (query (document "memory://snapshot/15_03_value_expression.md") (range (start 13 25) (end 13 36)) (probe (position 13 25))
    (reference (id (source (node (document "memory://snapshot/15_03_value_expression.md") (qualified-name "15_03-Value Expression::Wheel::width"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
    )
  )
)
~~~
