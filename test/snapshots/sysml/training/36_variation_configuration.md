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
    (element (id (node (document "d0") (qualified-name "Variation Configuration"))) (kind "package") (name "Variation Configuration") (declared-name "Variation Configuration") (range (start (line 0) (character 0)) (end (line 0) (character 390))))
    (element (id (node (document "d0") (qualified-name "Variation Configuration::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 38))) (parent (node (document "d0") (qualified-name "Variation Configuration"))) (authored (membership (kind Import) (visibility "private") (import (reference "Variation Usages::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl"))) (kind "part") (name "vehicle4Cyl") (declared-name "vehicle4Cyl") (range (start (line 3) (character 1)) (end (line 3) (character 153))) (parent (node (document "d0") (qualified-name "Variation Configuration"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicleFamily") (range (start (line 3) (character 21)) (end (line 3) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::engine"))) (kind "part") (name "engine") (range (start (line 4) (character 2)) (end (line 4) (character 47))) (parent (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine") (range (start (line 4) (character 17)) (end (line 4) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::transmission"))) (kind "part") (name "transmission") (range (start (line 5) (character 2)) (end (line 5) (character 65))) (parent (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "transmission") (range (start (line 5) (character 17)) (end (line 5) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl"))) (kind "part") (name "vehicle6Cyl") (declared-name "vehicle6Cyl") (range (start (line 8) (character 1)) (end (line 8) (character 153))) (parent (node (document "d0") (qualified-name "Variation Configuration"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicleFamily") (range (start (line 8) (character 21)) (end (line 8) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::engine"))) (kind "part") (name "engine") (range (start (line 9) (character 2)) (end (line 9) (character 47))) (parent (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine") (range (start (line 9) (character 17)) (end (line 9) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::transmission"))) (kind "part") (name "transmission") (range (start (line 10) (character 2)) (end (line 10) (character 65))) (parent (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "transmission") (range (start (line 10) (character 17)) (end (line 10) (character 29)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Variation Configuration::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Variation Usages::*") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl"))) (kind subsetting) (ordinal 0)) (authored-target "vehicleFamily") (range (start (line 3) (character 21)) (end (line 3) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::engine"))) (kind redefinition) (ordinal 0)) (authored-target "engine") (range (start (line 4) (character 17)) (end (line 4) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::transmission"))) (kind redefinition) (ordinal 0)) (authored-target "transmission") (range (start (line 5) (character 17)) (end (line 5) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl"))) (kind subsetting) (ordinal 0)) (authored-target "vehicleFamily") (range (start (line 8) (character 21)) (end (line 8) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::engine"))) (kind redefinition) (ordinal 0)) (authored-target "engine") (range (start (line 9) (character 17)) (end (line 9) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::transmission"))) (kind redefinition) (ordinal 0)) (authored-target "transmission") (range (start (line 10) (character 17)) (end (line 10) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::transmission")))))
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
