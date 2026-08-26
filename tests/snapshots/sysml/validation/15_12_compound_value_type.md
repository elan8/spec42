# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_12-Compound Value Type
type=file
~~~
# SOURCE
~~~sysml
package '15_12-Compound Value Type' {
	private import ScalarValues::*;
	private import USCustomaryUnits::'in';
	
	/*
	 * Real world user models would use quantity and vector types
	 * from library models. They are included here for the purpose
	 * of showing how such attribute defs can be defined.
	 */

    attribute def PositionVector {
        attribute x: Real[1];
        attribute y: Real[1];
        attribute z: Real[1];
    }
    
    attribute def LengthValue :> Real;

    attribute def TireInfo {
    	attribute manufacturer: String;
        attribute hubDiameter: LengthValue;
        attribute width: Integer;
        attribute placement: PositionVector[0..1];
    }
    
    attribute frenchTireInfo: TireInfo {
    	attribute :>> manufacturer = "Michelin";
    	attribute :>> hubDiameter = 18.0['in'];
    	attribute :>> width = 245;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/15_12_compound_value_type.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 21) (end 11 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 21) (end 12 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 21) (end 13 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 16 33) (end 16 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 29) (end 19 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 25) (end 21 32))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:767f47c2c6e2ae319e0161421c9a488e978c3d9927883dea7367747fe290e617") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type"))) (kind package) (membership (kind owning) (visibility default)) (documentation (comment (text "\n\t * Real world user models would use quantity and vector types\n\t * from library models. They are included here for the purpose\n\t * of showing how such attribute defs can be defined.\n\t "))))
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "USCustomaryUnits::in") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::LengthValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector::x"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector::y"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector::z"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")))))
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::manufacturer"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::placement"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PositionVector")))))
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::width"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TireInfo")))))
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "manufacturer")))))
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "hubDiameter")))))
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "width")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "USCustomaryUnits::in")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::LengthValue"))) (kind specialization) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector::z"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::LengthValue")))))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::manufacturer"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::placement"))) (kind featureTyping) (ordinal 0))
      (authored-target "PositionVector")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector")))))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::width"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (kind featureTyping) (ordinal 0))
      (authored-target "TireInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo")))))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "manufacturer")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::manufacturer")))))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "hubDiameter")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter")))))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "width")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::width")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::LengthValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::placement"))) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::placement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::manufacturer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 2))))) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::width"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector::x"))) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector::y"))) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector::z"))) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::manufacturer"))) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::placement"))) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::width"))) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 2))))) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind string) (value "Michelin")))
    (evaluated (declaration (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 1))))) (state literal) (value (kind quantity) (magnitude (value (kind real) (real 18))) (unit "in")))
    (evaluated (declaration (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 2))))) (state literal) (value (kind integer) (integer 245)))
    (unit (declaration (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 1))))) (ordinal 0) (authored "in") (start 27 38) (end 27 42) (outcome (status catalog-unavailable)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::LengthValue")))
      (subtype (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector")))
      (subtype (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::placement")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector::x")))
      (featured-by (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector")))
    )
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector::y")))
      (featured-by (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector")))
    )
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector::z")))
      (featured-by (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector")))
    )
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo")))
      (subtype (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::frenchTireInfo")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter")))
      (featured-by (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo")))
      (type (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::LengthValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::LengthValue")) (source direct))
      (supertype (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::LengthValue")) (scopes any))
      (subtype (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::manufacturer")))
      (featured-by (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo")))
      (subtype (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::placement")))
      (featured-by (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo")))
      (type (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector")) (provenance authored))
      (effective-type (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector")) (source direct))
      (supertype (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::width")))
      (featured-by (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo")))
      (subtype (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 2)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::frenchTireInfo")))
      (type (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo")) (provenance authored))
      (effective-type (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo")) (source direct))
      (supertype (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::frenchTireInfo")))
      (supertype (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::manufacturer")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::frenchTireInfo")))
      (effective-type (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::LengthValue")) (source inherited) (from (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))))
      (supertype (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::LengthValue")) (scopes any))
      (supertype (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::frenchTireInfo")))
      (supertype (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::width")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/15_12_compound_value_type.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_12_compound_value_type.md") (range (start 2 16) (end 2 38)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "USCustomaryUnits::in")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_12_compound_value_type.md") (range (start 16 33) (end 16 37)) (probe (position 16 33))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::LengthValue"))) (kind specialization) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_12_compound_value_type.md") (range (start 11 21) (end 11 25)) (probe (position 11 21))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector::x"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_12_compound_value_type.md") (range (start 12 21) (end 12 25)) (probe (position 12 21))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector::y"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_12_compound_value_type.md") (range (start 13 21) (end 13 25)) (probe (position 13 21))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector::z"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_12_compound_value_type.md") (range (start 20 31) (end 20 42)) (probe (position 20 31))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::LengthValue")))))
    )
  )
  (query (document "memory://snapshot/15_12_compound_value_type.md") (range (start 19 29) (end 19 35)) (probe (position 19 29))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::manufacturer"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_12_compound_value_type.md") (range (start 22 29) (end 22 43)) (probe (position 22 29))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::placement"))) (kind featureTyping) (ordinal 0) (authored-target "PositionVector")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::PositionVector")))))
    )
  )
  (query (document "memory://snapshot/15_12_compound_value_type.md") (range (start 21 25) (end 21 32)) (probe (position 21 25))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::width"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_12_compound_value_type.md") (range (start 25 30) (end 25 38)) (probe (position 25 30))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (kind featureTyping) (ordinal 0) (authored-target "TireInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo")))))
    )
  )
  (query (document "memory://snapshot/15_12_compound_value_type.md") (range (start 26 19) (end 26 31)) (probe (position 26 19))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "manufacturer")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::manufacturer")))))
    )
  )
  (query (document "memory://snapshot/15_12_compound_value_type.md") (range (start 27 19) (end 27 30)) (probe (position 27 19))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "hubDiameter")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter")))))
    )
  )
  (query (document "memory://snapshot/15_12_compound_value_type.md") (range (start 28 19) (end 28 24)) (probe (position 28 19))
    (reference (id (source (node (document "memory://snapshot/15_12_compound_value_type.md") (path (named (kind package) (name "15_12-Compound Value Type")) (named (kind attribute) (name "frenchTireInfo")) (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "width")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_12_compound_value_type.md") (qualified-name "15_12-Compound Value Type::TireInfo::width")))))
    )
  )
)
~~~
