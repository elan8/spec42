# META
~~~ini
description=SysML Training 25 (Transitions): Local Clock Example
type=file
~~~
# SOURCE
~~~sysml
package 'Local Clock Example' {
	private import ScalarValues::String;
	
	item def Start;
	item def Request;
	
	part def Server {
		part :>> localClock = new Time::Clock();

		attribute today : String;
				
		port requestPort;
		
		state ServerBehavior {
			entry; then off;
			
			state off;
			accept Start via requestPort
				then waiting;
			
			state waiting;
			accept request : Request via requestPort
				then responding;
			accept at new Time::Iso8601DateTime(today + "11:59:00")
				then off;
			
			state responding;
			accept after 5 [SI::min]
				then waiting;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/25_local_clock_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 11) (end 7 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 20) (end 9 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 14 3) (end 14 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 14 10) (end 14 10))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 17 3) (end 17 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 21 3) (end 21 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 23 3) (end 23 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 27 3) (end 27 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:c87210a718fb52b6e4333f862eb8c447e5365a87e88fe8ce7597a09f5281e838") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Request"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "localClock"))))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::responding"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::requestPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::today"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Start"))) (kind item-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::today"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
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
  (query (document "memory://snapshot/25_local_clock_example.md") (range (start 1 16) (end 1 36)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/25_local_clock_example.md") (range (start 7 11) (end 7 21)) (probe (position 7 11))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/25_local_clock_example.md") (range (start 9 20) (end 9 26)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::today"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
)
~~~
