# META
~~~ini
description=SysML Training 36 (Variability): Variation Configuration
type=file
~~~
# SOURCE
~~~sysml
package 'Variation Configuration' {
	private import 'Variation Usages'::*;
	
	part vehicle4Cyl :> vehicleFamily {
		part redefines engine = engine::'4cylEngine';
		part redefines transmission = transmission::manualTransmission;
	}
	
	part vehicle6Cyl :> vehicleFamily {
		part redefines engine = engine::'6cylEngine';
		part redefines transmission = transmission::manualTransmission;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "36_variation_configuration.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 21) (end 3 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 21) (end 8 34))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Variation Configuration' {
    private import 'Variation Usages'::*;

    part vehicle4Cyl :> vehicleFamily {
        part redefines engine = engine::'4cylEngine';
        part redefines transmission = transmission::manualTransmission;
    }

    part vehicle6Cyl :> vehicleFamily {
        part redefines engine = engine::'6cylEngine';
        part redefines transmission = transmission::manualTransmission;
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "cabfe4b1e990a2ea6bdd43b63eccc2082bd306a61ff96452d90f5802bd7fb495") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Variation Configuration"))) (kind "package") (name "Variation Configuration") (declared-name "Variation Configuration"))
    (element (id (node (document "d0") (qualified-name "Variation Configuration::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Variation Configuration"))) (authored (membership (kind Import) (visibility "private") (import (reference "Variation Usages::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl"))) (kind "part") (name "vehicle4Cyl") (declared-name "vehicle4Cyl") (parent (node (document "d0") (qualified-name "Variation Configuration"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicleFamily")))))
    (element (id (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::engine"))) (kind "part") (name "engine") (parent (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine")))))
    (element (id (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::transmission"))) (kind "part") (name "transmission") (parent (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "transmission")))))
    (element (id (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl"))) (kind "part") (name "vehicle6Cyl") (declared-name "vehicle6Cyl") (parent (node (document "d0") (qualified-name "Variation Configuration"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicleFamily")))))
    (element (id (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::engine"))) (kind "part") (name "engine") (parent (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine")))))
    (element (id (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::transmission"))) (kind "part") (name "transmission") (parent (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "transmission")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Variation Configuration::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Variation Usages::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl"))) (kind subsetting) (ordinal 0)) (authored-target "vehicleFamily") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::engine"))) (kind redefinition) (ordinal 0)) (authored-target "engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::transmission"))) (kind redefinition) (ordinal 0)) (authored-target "transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl"))) (kind subsetting) (ordinal 0)) (authored-target "vehicleFamily") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::engine"))) (kind redefinition) (ordinal 0)) (authored-target "engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::transmission"))) (kind redefinition) (ordinal 0)) (authored-target "transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::transmission")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::engine"))) (target (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::engine"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::transmission"))) (target (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::transmission"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::engine"))) (target (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::engine"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::transmission"))) (target (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::transmission"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::engine")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::transmission")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::engine")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::transmission")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 17) (end 4 23)) (probe (position 4 17))
      (reference
        (source (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::engine"))
        (kind redefinition) (ordinal 0) (authored-target "engine")
        (range (start 4 17) (end 4 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::engine") (range (start 4 2) (end 4 47)))
        )
      )
    )
    (query (range (start 9 17) (end 9 23)) (probe (position 9 17))
      (reference
        (source (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::engine"))
        (kind redefinition) (ordinal 0) (authored-target "engine")
        (range (start 9 17) (end 9 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::engine") (range (start 9 2) (end 9 47)))
        )
      )
    )
    (query (range (start 5 17) (end 5 29)) (probe (position 5 17))
      (reference
        (source (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::transmission"))
        (kind redefinition) (ordinal 0) (authored-target "transmission")
        (range (start 5 17) (end 5 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::transmission") (range (start 5 2) (end 5 65)))
        )
      )
    )
    (query (range (start 10 17) (end 10 29)) (probe (position 10 17))
      (reference
        (source (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::transmission"))
        (kind redefinition) (ordinal 0) (authored-target "transmission")
        (range (start 10 17) (end 10 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::transmission") (range (start 10 2) (end 10 65)))
        )
      )
    )
    (query (range (start 3 21) (end 3 34)) (probe (position 3 21))
      (reference
        (source (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl"))
        (kind subsetting) (ordinal 0) (authored-target "vehicleFamily")
        (range (start 3 21) (end 3 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 21) (end 8 34)) (probe (position 8 21))
      (reference
        (source (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl"))
        (kind subsetting) (ordinal 0) (authored-target "vehicleFamily")
        (range (start 8 21) (end 8 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Variation Configuration::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Variation Usages::*")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
