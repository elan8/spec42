# META
~~~ini
description=SysML Training 31 (Constraints): Constraints Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Constraints Example-2' {
	private import ISQ::*;
	private import SI::*;
	private import NumericalFunctions::*;
	
	part def Engine;
	part def Transmission;
	
	constraint def MassConstraint {
		attribute partMasses : MassValue[0..*];
		attribute massLimit : MassValue;
			
		sum(partMasses) <= massLimit
	}
	
	part def Vehicle {
		constraint massConstraint : MassConstraint {
			redefines partMasses = (chassisMass, engine.mass, transmission.mass);
			redefines massLimit = 2500[kg];
		}
		
		attribute chassisMass : MassValue;
		
		part engine : Engine {
			attribute mass : MassValue;
		}
		
		part transmission : Engine {
			attribute mass : MassValue;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "31_constraints_example_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 2) (end 21 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 26) (end 21 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 3) (end 24 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 20) (end 24 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 3) (end 28 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 20) (end 28 29))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Constraints Example-2' {
	private import ISQ::*;
	private import SI::*;
	private import NumericalFunctions::*;
	
	part def Engine;
	part def Transmission;
	
	constraint def MassConstraint {
		attribute partMasses : MassValue[0..*];
		attribute massLimit : MassValue;
			
		sum(partMasses) <= massLimit
	}
	
	part def Vehicle {
		constraint massConstraint : MassConstraint {
			redefines partMasses = (chassisMass, engine.mass, transmission.mass);
			redefines massLimit = 2500[kg];
		}
		
		attribute chassisMass : MassValue;
		
		part engine : Engine {
			attribute mass : MassValue;
		}
		
		part transmission : Engine {
			attribute mass : MassValue;
		}
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "fa0b14c61bf599f448ff402c67fe5b3a9a22f9c5767709f51660d42113631618") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Constraints Example-2"))) (kind "package") (name "Constraints Example-2") (declared-name "Constraints Example-2") (range (start (line 0) (character 0)) (end (line 0) (character 671))))
    (element (id (node (document "d0") (qualified-name "Constraints Example-2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 23))) (parent (node (document "d0") (qualified-name "Constraints Example-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 19))))))
    (element (id (node (document "d0") (qualified-name "Constraints Example-2::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 22))) (parent (node (document "d0") (qualified-name "Constraints Example-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 18))))))
    (element (id (node (document "d0") (qualified-name "Constraints Example-2::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 38))) (parent (node (document "d0") (qualified-name "Constraints Example-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Constraints Example-2::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 5) (character 1)) (end (line 5) (character 17))) (parent (node (document "d0") (qualified-name "Constraints Example-2"))))
    (element (id (node (document "d0") (qualified-name "Constraints Example-2::MassConstraint"))) (kind "constraint def") (name "MassConstraint") (declared-name "MassConstraint") (range (start (line 8) (character 1)) (end (line 8) (character 147))) (parent (node (document "d0") (qualified-name "Constraints Example-2"))))
    (element (id (node (document "d0") (qualified-name "Constraints Example-2::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (range (start (line 6) (character 1)) (end (line 6) (character 23))) (parent (node (document "d0") (qualified-name "Constraints Example-2"))))
    (element (id (node (document "d0") (qualified-name "Constraints Example-2::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 15) (character 1)) (end (line 15) (character 353))) (parent (node (document "d0") (qualified-name "Constraints Example-2"))))
    (element (id (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::chassisMass"))) (kind "attribute") (name "chassisMass") (declared-name "chassisMass") (range (start (line 21) (character 2)) (end (line 21) (character 36))) (parent (node (document "d0") (qualified-name "Constraints Example-2::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 21) (character 26)) (end (line 21) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 23) (character 2)) (end (line 23) (character 59))) (parent (node (document "d0") (qualified-name "Constraints Example-2::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 23) (character 16)) (end (line 23) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::engine::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 24) (character 3)) (end (line 24) (character 30))) (parent (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 24) (character 20)) (end (line 24) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 27) (character 2)) (end (line 27) (character 65))) (parent (node (document "d0") (qualified-name "Constraints Example-2::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 27) (character 22)) (end (line 27) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::transmission::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 28) (character 3)) (end (line 28) (character 30))) (parent (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::transmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 28) (character 20)) (end (line 28) (character 29)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Constraints Example-2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 1) (character 16)) (end (line 1) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraints Example-2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 2) (character 16)) (end (line 2) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraints Example-2::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (range (start (line 3) (character 16)) (end (line 3) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::chassisMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::chassisMass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 21) (character 26)) (end (line 21) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 23) (character 16)) (end (line 23) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Constraints Example-2::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::engine::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::engine::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 24) (character 20)) (end (line 24) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 27) (character 22)) (end (line 27) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Constraints Example-2::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::transmission::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::transmission::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 28) (character 20)) (end (line 28) (character 29))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::engine"))) (target (node (document "d0") (qualified-name "Constraints Example-2::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::transmission"))) (target (node (document "d0") (qualified-name "Constraints Example-2::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Constraints Example-2::Vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Constraints Example-2::MassConstraint")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
