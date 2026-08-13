# META
~~~ini
description=SysML Training 17 (Control): Fork Join Example
type=file
~~~
# SOURCE
~~~sysml
package 'Fork Join Example' {
	private import ScalarValues::*;
	
	attribute def TurnKeyToOn;
	attribute def BrakePressure;
	
	action def MonitorBrakePedal { out pressure : BrakePressure; }
	action def MonitorTraction { out modFreq : Real; }
	action def Braking { in brakePressure : BrakePressure; in modulationFrequency : Real; }
	
	action def Brake {
		action TurnOn;
		
		then fork;
			then monitorBrakePedal;
			then monitorTraction;
			then braking;
		
		action monitorBrakePedal : MonitorBrakePedal {
			out brakePressure;
		}
		then joinNode;
		
		action monitorTraction : MonitorTraction {
			out modulationFrequency;
		}
		then joinNode;
		
		flow from monitorBrakePedal.brakePressure to braking.brakePressure;
		flow from monitorTraction.modulationFrequency to braking.modulationFrequency; 
		
		action braking : Braking {
			in brakePressure; 
			in modulationFrequency;
		}
		then joinNode;
		
		join joinNode;
		then done;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/17_fork_join_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 6 32) (end 6 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 7 30) (end 7 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 8 22) (end 8 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 8 56) (end 8 86))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 13 2) (end 13 12))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 14 3) (end 14 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 15 3) (end 15 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 16 3) (end 16 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 19 3) (end 19 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 21 2) (end 21 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 24 3) (end 24 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 26 2) (end 26 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 28 2) (end 28 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 29 2) (end 29 79))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 32 3) (end 32 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 33 3) (end 33 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 35 2) (end 35 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 37 2) (end 37 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 38 2) (end 38 12))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:bfd0f6f0ecd96368346fb336be4a0114fe7f6569804fe19d2da39e31b80c690e") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_fork_join_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake::TurnOn"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake::braking"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Braking"))))
    (declaration (id (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake::monitorBrakePedal"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MonitorBrakePedal"))))
    (declaration (id (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake::monitorTraction"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MonitorTraction"))))
    (declaration (id (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::BrakePressure"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Braking"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::MonitorBrakePedal"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::MonitorTraction"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::TurnKeyToOn"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/17_fork_join_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake::braking"))) (kind featureTyping) (ordinal 0))
      (authored-target "Braking")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Braking")))))
    (reference (id (source (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake::monitorBrakePedal"))) (kind featureTyping) (ordinal 0))
      (authored-target "MonitorBrakePedal")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::MonitorBrakePedal")))))
    (reference (id (source (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake::monitorTraction"))) (kind featureTyping) (ordinal 0))
      (authored-target "MonitorTraction")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::MonitorTraction")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake::braking"))) (target (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Braking"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake::braking"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake::monitorBrakePedal"))) (target (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::MonitorBrakePedal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake::monitorBrakePedal"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake::monitorTraction"))) (target (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::MonitorTraction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake::monitorTraction"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/17_fork_join_example.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/17_fork_join_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/17_fork_join_example.md") (range (start 31 19) (end 31 26)) (probe (position 31 19))
    (reference (id (source (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake::braking"))) (kind featureTyping) (ordinal 0) (authored-target "Braking")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Braking")))))
  )
  (query (document "memory://snapshot/17_fork_join_example.md") (range (start 18 29) (end 18 46)) (probe (position 18 29))
    (reference (id (source (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake::monitorBrakePedal"))) (kind featureTyping) (ordinal 0) (authored-target "MonitorBrakePedal")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::MonitorBrakePedal")))))
  )
  (query (document "memory://snapshot/17_fork_join_example.md") (range (start 23 27) (end 23 42)) (probe (position 23 27))
    (reference (id (source (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::Brake::monitorTraction"))) (kind featureTyping) (ordinal 0) (authored-target "MonitorTraction")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_fork_join_example.md") (qualified-name "Fork Join Example::MonitorTraction")))))
  )
)
~~~
