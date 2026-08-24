# META
~~~ini
description=SysML Training 11 (Interfaces): Interface Example
type=file
~~~
# SOURCE
~~~sysml
package 'Interface Example' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	
	interface def FuelInterface {
		end supplierPort : FuelOutPort;
		end consumerPort : FuelInPort;
	}
	
	part vehicle : Vehicle {	
		part tankAssy : FuelTankAssembly;		
		part eng : Engine;
		
		interface : FuelInterface connect 
			supplierPort ::> tankAssy.fuelTankPort to 
			consumerPort ::> eng.engineFuelPort;
	} 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/11_interface_example.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 21) (end 6 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 21) (end 7 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 18) (end 11 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 13) (end 12 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 20) (end 15 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 20) (end 16 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e10b2e8059df16eab752aec5114ca304700425b79ee933845ff073fb3eb5aac1") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (path (named (kind package) (name "Interface Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Port Example") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface::consumerPort"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelInPort")))))
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface::supplierPort"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelOutPort")))))
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (path (named (kind package) (name "Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (kind interface) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelInterface")) (connectorEnd (reference "tankAssy::fuelTankPort")) (connectorEnd (reference "eng::engineFuelPort")))))
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle::eng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle::tankAssy"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelTankAssembly")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (path (named (kind package) (name "Interface Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Port Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface::consumerPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelInPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface::supplierPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelOutPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (path (named (kind package) (name "Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelInterface")
      (outcome (status resolved) (target (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface")))))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (path (named (kind package) (name "Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "tankAssy::fuelTankPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (path (named (kind package) (name "Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "eng::engineFuelPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle::tankAssy"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelTankAssembly")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle"))) (target (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/11_interface_example.md") (path (named (kind package) (name "Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (target (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11_interface_example.md") (path (named (kind package) (name "Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface::consumerPort"))) (target (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface::supplierPort"))) (target (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/11_interface_example.md") (path (named (kind package) (name "Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (target (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle::eng"))) (target (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle::tankAssy"))) (target (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface")))
      (positional-ends (authored 2) (effective 2))
      (subtype (node (document "memory://snapshot/11_interface_example.md") (path (named (kind package) (name "Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface::consumerPort")))
      (featured-by (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface")))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface::supplierPort")))
      (featured-by (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface")))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::Vehicle")))
      (subtype (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle")))
      (type (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (path (named (kind package) (name "Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0)))))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle")))
      (type (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface")) (provenance authored))
      (effective-type (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface")) (source direct))
      (supertype (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle::eng")))
      (featured-by (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle::tankAssy")))
      (featured-by (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/11_interface_example.md") (range (start 1 16) (end 1 33)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (path (named (kind package) (name "Interface Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Port Example")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/11_interface_example.md") (range (start 7 21) (end 7 31)) (probe (position 7 21))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface::consumerPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelInPort")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/11_interface_example.md") (range (start 6 21) (end 6 32)) (probe (position 6 21))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface::supplierPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelOutPort")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/11_interface_example.md") (range (start 10 16) (end 10 23)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/11_interface_example.md") (range (start 14 14) (end 14 27)) (probe (position 14 14))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (path (named (kind package) (name "Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "FuelInterface")
      (outcome (status resolved) (target (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::FuelInterface")))))
    )
  )
  (query (document "memory://snapshot/11_interface_example.md") (range (start 15 20) (end 15 41)) (probe (position 15 20))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (path (named (kind package) (name "Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "tankAssy::fuelTankPort")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/11_interface_example.md") (range (start 16 20) (end 16 38)) (probe (position 16 20))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (path (named (kind package) (name "Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "eng::engineFuelPort")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/11_interface_example.md") (range (start 12 13) (end 12 19)) (probe (position 12 13))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/11_interface_example.md") (range (start 11 18) (end 11 34)) (probe (position 11 18))
    (reference (id (source (node (document "memory://snapshot/11_interface_example.md") (qualified-name "Interface Example::vehicle::tankAssy"))) (kind featureTyping) (ordinal 0) (authored-target "FuelTankAssembly")
      (outcome (status unresolved)))
    )
  )
)
~~~
