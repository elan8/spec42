# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_02-Basic Value Properties
type=file
~~~
# SOURCE
~~~sysml
package '15_02-Basic Value Properties' {
	private import ScalarValues::*;
	
    attribute def LengthValue :> Real {
		doc
		/*
		 * Real world user models would use a quantity type
		 * from the library model. A attribute def is defined
		 * here to show that it is possible.
		 */
	}

    part def Tire {
    	attribute manufacturer: String;
        attribute hubDiameter: LengthValue;
        attribute width: Integer;
    }
    
    part frenchTire: Tire {
    	attribute :>> manufacturer = "Michelin";
    	attribute :>> hubDiameter = 18.0;
    	attribute :>> width = 245;
    }
    
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_02_basic_value_properties.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 4) (end 3 208))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 5) (end 13 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 29) (end 13 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 8) (end 15 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 25) (end 15 32))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package '15_02-Basic Value Properties' {
    private import ScalarValues::*;

    attribute def LengthValue :> Real {
        doc
        /*
		 * Real world user models would use a quantity type
		 * from the library model. A attribute def is defined
		 * here to show that it is possible.
		 */
    }

    part def Tire {
        attribute manufacturer: String;
        attribute hubDiameter: LengthValue;
        attribute width: Integer;
    }

    part frenchTire: Tire {
        attribute :>> manufacturer = "Michelin";
        attribute :>> hubDiameter = 18.0;
        attribute :>> width = 245;
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "aa86b8bc5ff2287f83c2b8418bc1c3b2425aa9952b0e18a9c699699feeac6cd6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties"))) (kind "package") (name "15_02-Basic Value Properties") (declared-name "15_02-Basic Value Properties"))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue"))) (kind "attribute def") (name "LengthValue") (declared-name "LengthValue") (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue"))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire"))) (kind "part def") (name "Tire") (declared-name "Tire") (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties"))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (kind "attribute") (name "hubDiameter") (declared-name "hubDiameter") (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::manufacturer"))) (kind "attribute") (name "manufacturer") (declared-name "manufacturer") (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")) (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::width"))) (kind "attribute") (name "width") (declared-name "width") (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer")) (typing (reference "Integer")))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (kind "part") (name "frenchTire") (declared-name "frenchTire") (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties"))) (authored (membership (kind Feature)) (relationships (typing (reference "Tire")))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter"))) (kind "attribute") (name "hubDiameter") (declared-name "hubDiameter") (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "hubDiameter")))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer"))) (kind "attribute") (name "manufacturer") (declared-name "manufacturer") (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "manufacturer")))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width"))) (kind "attribute") (name "width") (declared-name "width") (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "width")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::manufacturer"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::manufacturer"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::width"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::width"))) (kind featureTyping) (ordinal 1)) (authored-target "Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (kind featureTyping) (ordinal 0)) (authored-target "Tire") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire")))))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter"))) (kind redefinition) (ordinal 0)) (authored-target "hubDiameter") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter")))))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer"))) (kind redefinition) (ordinal 0)) (authored-target "manufacturer") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer")))))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width"))) (kind redefinition) (ordinal 0)) (authored-target "width") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter"))) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer"))) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width"))) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter")) (expression (status "ok") (value (integer 18))))
    (node (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer")) (expression (status "ok") (value (string "Michelin"))))
    (node (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width")) (expression (status "ok") (value (integer 245))))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 18 21) (end 18 25)) (probe (position 18 21))
      (reference
        (source (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))
        (kind featureTyping) (ordinal 0) (authored-target "Tire")
        (range (start 18 21) (end 18 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_02-Basic Value Properties::Tire") (range (start 12 4) (end 12 140)))
        )
      )
    )
    (query (range (start 21 19) (end 21 24)) (probe (position 21 19))
      (reference
        (source (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width"))
        (kind redefinition) (ordinal 0) (authored-target "width")
        (range (start 21 19) (end 21 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width") (range (start 21 5) (end 21 31)))
        )
      )
    )
    (query (range (start 13 29) (end 13 35)) (probe (position 13 29))
      (reference
        (source (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::manufacturer"))
        (kind featureTyping) (ordinal 1) (authored-target "String")
        (range (start 13 29) (end 13 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 25) (end 15 32)) (probe (position 15 25))
      (reference
        (source (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::width"))
        (kind featureTyping) (ordinal 1) (authored-target "Integer")
        (range (start 15 25) (end 15 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 31) (end 14 42)) (probe (position 14 31))
      (reference
        (source (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))
        (kind featureTyping) (ordinal 1) (authored-target "LengthValue")
        (range (start 14 31) (end 14 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue") (range (start 3 4) (end 3 208)))
        )
      )
    )
    (query (range (start 20 19) (end 20 30)) (probe (position 20 19))
      (reference
        (source (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter"))
        (kind redefinition) (ordinal 0) (authored-target "hubDiameter")
        (range (start 20 19) (end 20 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter") (range (start 20 5) (end 20 38)))
        )
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "15_02-Basic Value Properties::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 19) (end 19 31)) (probe (position 19 19))
      (reference
        (source (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer"))
        (kind redefinition) (ordinal 0) (authored-target "manufacturer")
        (range (start 19 19) (end 19 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer") (range (start 19 5) (end 19 45)))
        )
      )
    )
  )
)
~~~
