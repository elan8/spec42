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
  (document "memory://snapshot/15_02_basic_value_properties.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 3 33) (end 3 37))
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
        (range (start 15 25) (end 15 32))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:b41c0af10bccf301a48e6fa47472e6e3fa145471ce51d7e885f5bee961735f2a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::LengthValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Real world user models would use a quantity type\n\t\t * from the library model. A attribute def is defined\n\t\t * here to show that it is possible.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")))))
    (declaration (id (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::manufacturer"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::width"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Tire")))))
    (declaration (id (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "manufacturer")))))
    (declaration (id (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "hubDiameter")))))
    (declaration (id (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "width")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::LengthValue"))) (kind specialization) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::LengthValue")))))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::manufacturer"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::width"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tire")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire")))))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "manufacturer")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::manufacturer")))))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "hubDiameter")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter")))))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "width")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::width")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (target (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::LengthValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (target (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::manufacturer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 2))))) (target (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::width"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind string) (value "Michelin")))
    (evaluated (declaration (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 1))))) (state literal) (value (kind real) (real 18)))
    (evaluated (declaration (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 2))))) (state literal) (value (kind integer) (integer 245)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter")))
      (supertype (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::LengthValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::frenchTire")))
      (supertype (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::manufacturer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 1)))))
      (supertype (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::LengthValue")) (scopes any))
      (supertype (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 2)))))
      (supertype (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::width")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/15_02_basic_value_properties.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_02_basic_value_properties.md") (range (start 3 33) (end 3 37)) (probe (position 3 33))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::LengthValue"))) (kind specialization) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_02_basic_value_properties.md") (range (start 14 31) (end 14 42)) (probe (position 14 31))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::LengthValue")))))
    )
  )
  (query (document "memory://snapshot/15_02_basic_value_properties.md") (range (start 13 29) (end 13 35)) (probe (position 13 29))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::manufacturer"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_02_basic_value_properties.md") (range (start 15 25) (end 15 32)) (probe (position 15 25))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::width"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_02_basic_value_properties.md") (range (start 18 21) (end 18 25)) (probe (position 18 21))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (kind featureTyping) (ordinal 0) (authored-target "Tire")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire")))))
    )
  )
  (query (document "memory://snapshot/15_02_basic_value_properties.md") (range (start 19 19) (end 19 31)) (probe (position 19 19))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "manufacturer")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::manufacturer")))))
    )
  )
  (query (document "memory://snapshot/15_02_basic_value_properties.md") (range (start 20 19) (end 20 30)) (probe (position 20 19))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "hubDiameter")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter")))))
    )
  )
  (query (document "memory://snapshot/15_02_basic_value_properties.md") (range (start 21 19) (end 21 24)) (probe (position 21 19))
    (reference (id (source (node (document "memory://snapshot/15_02_basic_value_properties.md") (path (named (kind package) (name "15_02-Basic Value Properties")) (named (kind part) (name "frenchTire")) (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "width")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_02_basic_value_properties.md") (qualified-name "15_02-Basic Value Properties::Tire::width")))))
    )
  )
)
~~~
