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
  (document "15_12_compound_value_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
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
        (range (start 11 8) (end 11 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 8) (end 12 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 8) (end 13 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 4) (end 16 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 5) (end 19 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 8) (end 21 33))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "75aec00f8071926fcecfd7ec6e45f6564ea0ccc21920b6a6df5e833e1c1b915d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type"))) (kind "package") (name "15_12-Compound Value Type") (declared-name "15_12-Compound Value Type"))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_12-Compound Value Type"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::LengthValue"))) (kind "attribute def") (name "LengthValue") (declared-name "LengthValue") (parent (node (document "d0") (qualified-name "15_12-Compound Value Type"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector"))) (kind "attribute def") (name "PositionVector") (declared-name "PositionVector") (parent (node (document "d0") (qualified-name "15_12-Compound Value Type"))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector::x"))) (kind "attribute") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector::y"))) (kind "attribute") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector::z"))) (kind "attribute") (name "z") (declared-name "z") (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo"))) (kind "attribute def") (name "TireInfo") (declared-name "TireInfo") (parent (node (document "d0") (qualified-name "15_12-Compound Value Type"))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (kind "attribute") (name "hubDiameter") (declared-name "hubDiameter") (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::manufacturer"))) (kind "attribute") (name "manufacturer") (declared-name "manufacturer") (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::placement"))) (kind "attribute") (name "placement") (declared-name "placement") (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo"))) (authored (membership (kind Feature)) (relationships (typing (reference "PositionVector")))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::width"))) (kind "attribute") (name "width") (declared-name "width") (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer")))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (kind "attribute def") (name "frenchTireInfo") (declared-name "frenchTireInfo") (parent (node (document "d0") (qualified-name "15_12-Compound Value Type"))) (authored (membership (kind Owning)) (relationships (typing (reference "TireInfo")))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::hubDiameter"))) (kind "attribute") (name "hubDiameter") (declared-name "hubDiameter") (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "hubDiameter")))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::manufacturer"))) (kind "attribute") (name "manufacturer") (declared-name "manufacturer") (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "manufacturer")))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::width"))) (kind "attribute") (name "width") (declared-name "width") (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "width")))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::in"))) (kind "import") (name "in") (declared-name "in") (parent (node (document "d0") (qualified-name "15_12-Compound Value Type"))) (authored (membership (kind Import) (visibility "private") (import (reference "USCustomaryUnits::in") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::LengthValue"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector::x"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector::y"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector::z"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::LengthValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::manufacturer"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::placement"))) (kind featureTyping) (ordinal 0)) (authored-target "PositionVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector")))))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::width"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (kind featureTyping) (ordinal 0)) (authored-target "TireInfo") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo")))))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::hubDiameter"))) (kind redefinition) (ordinal 0)) (authored-target "hubDiameter") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::hubDiameter")))))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::manufacturer"))) (kind redefinition) (ordinal 0)) (authored-target "manufacturer") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::manufacturer")))))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::width"))) (kind redefinition) (ordinal 0)) (authored-target "width") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::width")))))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::in"))) (kind membershipImport) (ordinal 0)) (authored-target "USCustomaryUnits::in") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::LengthValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::placement"))) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::placement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::hubDiameter"))) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::hubDiameter"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::hubDiameter"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::manufacturer"))) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::manufacturer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::manufacturer"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::width"))) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::width"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::width"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 28 19) (end 28 24)) (probe (position 28 19))
      (reference
        (source (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::width"))
        (kind redefinition) (ordinal 0) (authored-target "width")
        (range (start 28 19) (end 28 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::width") (range (start 28 5) (end 28 31)))
        )
      )
    )
    (query (range (start 27 19) (end 27 30)) (probe (position 27 19))
      (reference
        (source (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::hubDiameter"))
        (kind redefinition) (ordinal 0) (authored-target "hubDiameter")
        (range (start 27 19) (end 27 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::hubDiameter") (range (start 27 5) (end 27 44)))
        )
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "15_12-Compound Value Type::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 26 19) (end 26 31)) (probe (position 26 19))
      (reference
        (source (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::manufacturer"))
        (kind redefinition) (ordinal 0) (authored-target "manufacturer")
        (range (start 26 19) (end 26 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::manufacturer") (range (start 26 5) (end 26 45)))
        )
      )
    )
    (query (range (start 2 16) (end 2 38)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "15_12-Compound Value Type::in"))
        (kind membershipImport) (ordinal 0) (authored-target "USCustomaryUnits::in")
        (range (start 2 16) (end 2 38))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
