# META
~~~ini
description=SysML Training 32 (Requirements): Requirement Satisfaction
type=file
~~~
# SOURCE
~~~sysml
package 'Requirement Satisfaction' {
	private import 'Requirement Definitions'::*;
	private import 'Requirement Groups'::*;
	
	action 'provide power' {
		action 'generate torque' { }
	}
	
	part vehicle_c1 : Vehicle {
		perform 'provide power';
			
		part engine_v1: Engine {
			port :>> clutchPort;
			perform 'provide power'.'generate torque' :>> generateTorque;
		}	
	}
	
	part 'Vehicle c1 Design Context' {
		
		ref vehicle_design :> vehicle_c1;
	
		satisfy vehicleSpecification by vehicle_design;
		satisfy engineSpecification by vehicle_design.engine_v1;
	
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "32_requirement_satisfaction.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 19) (end 8 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 2) (end 11 119))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 18) (end 11 24))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 19 2) (end 19 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 10) (end 21 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 34) (end 21 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 10) (end 22 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 33) (end 22 57))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Requirement Satisfaction' {
    private import 'Requirement Definitions'::*;
    private import 'Requirement Groups'::*;

    action 'provide power' {
        action 'generate torque' { }
    }

    part vehicle_c1 : Vehicle {
        perform 'provide power';

        part engine_v1: Engine {
            port :>> clutchPort;
            perform 'provide power'.'generate torque' :>> generateTorque;
        }
    }

    part 'Vehicle c1 Design Context' {

        ref vehicle_design :> vehicle_c1;

        satisfy vehicleSpecification by vehicle_design;
        satisfy engineSpecification by vehicle_design.engine_v1;

    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "db1d8b266168a720f9b2d95b9c1ed6598493ce95f740104d80a2403fa1c83680") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Requirement Satisfaction"))) (kind "package") (name "Requirement Satisfaction") (declared-name "Requirement Satisfaction") (range (start (line 0) (character 0)) (end (line 0) (character 568))))
    (element (id (node (document "d0") (qualified-name "Requirement Satisfaction::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 45))) (parent (node (document "d0") (qualified-name "Requirement Satisfaction"))) (authored (membership (kind Import) (visibility "private") (import (reference "Requirement Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Requirement Satisfaction::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 40))) (parent (node (document "d0") (qualified-name "Requirement Satisfaction"))) (authored (membership (kind Import) (visibility "private") (import (reference "Requirement Groups::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 36))))))
    (element (id (node (document "d0") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context"))) (kind "part") (name "Vehicle c1 Design Context") (declared-name "Vehicle c1 Design Context") (range (start (line 17) (character 1)) (end (line 17) (character 190))) (parent (node (document "d0") (qualified-name "Requirement Satisfaction"))))
    (element (id (node (document "d0") (qualified-name "Requirement Satisfaction::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (range (start (line 4) (character 1)) (end (line 4) (character 59))) (parent (node (document "d0") (qualified-name "Requirement Satisfaction"))) (authored (membership (kind Feature)) (relationships (perform (reference "Requirement Satisfaction::provide power::generate torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Satisfaction::provide power::generate torque"))) (kind "action") (name "generate torque") (declared-name "generate torque") (range (start (line 5) (character 2)) (end (line 5) (character 30))) (parent (node (document "d0") (qualified-name "Requirement Satisfaction::provide power"))))
    (element (id (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (kind "part") (name "vehicle_c1") (declared-name "vehicle_c1") (range (start (line 8) (character 1)) (end (line 8) (character 183))) (parent (node (document "d0") (qualified-name "Requirement Satisfaction"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 8) (character 19)) (end (line 8) (character 26)))) (perform (reference "Requirement Satisfaction::vehicle_c1::provide power") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1"))) (kind "part") (name "engine_v1") (declared-name "engine_v1") (range (start (line 11) (character 2)) (end (line 11) (character 119))) (parent (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 11) (character 18)) (end (line 11) (character 24)))) (perform (reference "Requirement Satisfaction::vehicle_c1::engine_v1::provide power::generate torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (range (start (line 12) (character 3)) (end (line 12) (character 23))) (parent (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "clutchPort") (range (start (line 12) (character 12)) (end (line 12) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1::provide power.generate torque"))) (kind "action") (name "provide power.generate torque") (declared-name "provide power.generate torque") (range (start (line 13) (character 3)) (end (line 13) (character 64))) (parent (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1"))))
    (element (id (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (range (start (line 9) (character 2)) (end (line 9) (character 26))) (parent (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Requirement Satisfaction::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Requirement Definitions::*") (range (start (line 1) (character 16)) (end (line 1) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Satisfaction::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Requirement Groups::*") (range (start (line 2) (character 16)) (end (line 2) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context"))) (kind satisfySource) (ordinal 0)) (authored-target "vehicleSpecification") (range (start (line 21) (character 10)) (end (line 21) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context"))) (kind satisfySource) (ordinal 1)) (authored-target "engineSpecification") (range (start (line 22) (character 10)) (end (line 22) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context"))) (kind satisfyTarget) (ordinal 0)) (authored-target "vehicle_design") (range (start (line 21) (character 34)) (end (line 21) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context"))) (kind satisfyTarget) (ordinal 1)) (authored-target "vehicle_design::engine_v1") (range (start (line 22) (character 33)) (end (line 22) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Satisfaction::provide power"))) (kind performSource) (ordinal 0)) (authored-target "Requirement Satisfaction::provide power::generate torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Satisfaction::provide power::generate torque")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 8) (character 19)) (end (line 8) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (kind performSource) (ordinal 0)) (authored-target "Requirement Satisfaction::vehicle_c1::provide power") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::provide power")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 11) (character 18)) (end (line 11) (character 24))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1"))) (kind performSource) (ordinal 0)) (authored-target "Requirement Satisfaction::vehicle_c1::engine_v1::provide power::generate torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1::clutchPort"))) (kind redefinition) (ordinal 0)) (authored-target "clutchPort") (range (start (line 12) (character 12)) (end (line 12) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1::clutchPort")))))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Requirement Satisfaction::provide power"))) (target (node (document "d0") (qualified-name "Requirement Satisfaction::provide power::generate torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Satisfaction::provide power"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (target (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::provide power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (kind performSource) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1::clutchPort"))) (target (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1::clutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1::clutchPort"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 11 18) (end 11 24)) (probe (position 11 18))
      (reference
        (source (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 11 18) (end 11 24))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 19) (end 8 26)) (probe (position 8 19))
      (reference
        (source (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 8 19) (end 8 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 12) (end 12 22)) (probe (position 12 12))
      (reference
        (source (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1::clutchPort"))
        (kind redefinition) (ordinal 0) (authored-target "clutchPort")
        (range (start 12 12) (end 12 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1::clutchPort") (range (start 12 3) (end 12 23)))
        )
      )
    )
    (query (range (start 21 34) (end 21 48)) (probe (position 21 34))
      (reference
        (source (document "d0") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context"))
        (kind satisfyTarget) (ordinal 0) (authored-target "vehicle_design")
        (range (start 21 34) (end 21 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 22 10) (end 22 29)) (probe (position 22 10))
      (reference
        (source (document "d0") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context"))
        (kind satisfySource) (ordinal 1) (authored-target "engineSpecification")
        (range (start 22 10) (end 22 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 36)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Requirement Satisfaction::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Requirement Groups::*")
        (range (start 2 16) (end 2 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 10) (end 21 30)) (probe (position 21 10))
      (reference
        (source (document "d0") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context"))
        (kind satisfySource) (ordinal 0) (authored-target "vehicleSpecification")
        (range (start 21 10) (end 21 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 22 33) (end 22 57)) (probe (position 22 33))
      (reference
        (source (document "d0") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context"))
        (kind satisfyTarget) (ordinal 1) (authored-target "vehicle_design::engine_v1")
        (range (start 22 33) (end 22 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 41)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Requirement Satisfaction::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Requirement Definitions::*")
        (range (start 1 16) (end 1 41))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
