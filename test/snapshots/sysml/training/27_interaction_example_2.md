# META
~~~ini
description=SysML Training 27 (Occurrences): Interaction Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Interaction Example-2' {
	private import 'Event Occurrence Example'::*;
	
	item def SetSpeed;
	item def SensedSpeed;
	item def FuelCommand;
	
	occurrence def CruiseControlInteraction {
		
		ref part driver : Driver {
			event setSpeedMessage.sourceEvent;
		}
		
		ref part vehicle : Vehicle {
			part cruiseController : CruiseController {
				event setSpeedMessage.targetEvent;		
				then event sensedSpeedMessage.targetEvent;		
				then event fuelCommandMessage.sourceEvent;
			}
			
			part speedometer : Speedometer {
				event sensedSpeedMessage.sourceEvent;
			}
			
			part engine : Engine {
				event fuelCommandMessage.targetEvent;
			}
		}
		
		message setSpeedMessage of SetSpeed;	
		then message sensedSpeedMessage of SensedSpeed;
		message fuelCommandMessage of FuelCommand;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/27_interaction_example_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 9 2) (end 11 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 13 2) (end 27 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 29 2) (end 29 38))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 30 2) (end 31 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 31 2) (end 31 44))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:a7d64d59aa843b02caf516aad5856077fb1fec8581e6cedd28cdcf6c23048bec") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Event Occurrence Example") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::FuelCommand"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::SensedSpeed"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::SetSpeed"))) (kind item-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Event Occurrence Example")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/27_interaction_example_2.md") (range (start 1 16) (end 1 45)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Event Occurrence Example")
      (outcome (status unresolved)))
  )
)
~~~
