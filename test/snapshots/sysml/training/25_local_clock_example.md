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
  (document "25_local_clock_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 36))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "dcb0970e0c5521e251575f0e10ddb38cc7d7922cd0db06f60a3e454eac034bb7") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Local Clock Example"))) (kind "package") (name "Local Clock Example") (declared-name "Local Clock Example"))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Request"))) (kind "item def") (name "Request") (declared-name "Request") (parent (node (document "d0") (qualified-name "Local Clock Example"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server"))) (kind "part def") (name "Server") (declared-name "Server") (parent (node (document "d0") (qualified-name "Local Clock Example"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind "state") (name "ServerBehavior") (declared-name "ServerBehavior") (parent (node (document "d0") (qualified-name "Local Clock Example::Server"))) (authored (membership (kind Feature)) (relationships (transition (reference "Local Clock Example::Server::ServerBehavior::waiting")) (transition (reference "Local Clock Example::Server::ServerBehavior::responding")) (transition (reference "Local Clock Example::Server::ServerBehavior::off")) (transition (reference "Local Clock Example::Server::ServerBehavior::waiting")) (initial-state (reference "Local Clock Example::Server::ServerBehavior::off")))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::off"))) (kind "state") (name "off") (declared-name "off") (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::responding"))) (kind "state") (name "responding") (declared-name "responding") (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_off"))) (kind "transition") (name "transition_ServerBehavior_to_off") (declared-name "transition_ServerBehavior_to_off") (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_off"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_responding"))) (kind "transition") (name "transition_ServerBehavior_to_responding") (declared-name "transition_ServerBehavior_to_responding") (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_responding::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_responding"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_waiting"))) (kind "transition") (name "transition_ServerBehavior_to_waiting") (declared-name "transition_ServerBehavior_to_waiting") (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_waiting#transition"))) (kind "transition") (name "transition_ServerBehavior_to_waiting") (declared-name "transition_ServerBehavior_to_waiting") (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_waiting#transition::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_waiting#transition"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_waiting::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_waiting"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting"))) (kind "state") (name "waiting") (declared-name "waiting") (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::localClock"))) (kind "part") (name "localClock") (parent (node (document "d0") (qualified-name "Local Clock Example::Server"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "localClock")))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::requestPort"))) (kind "port") (name "requestPort") (declared-name "requestPort") (parent (node (document "d0") (qualified-name "Local Clock Example::Server"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::today"))) (kind "attribute") (name "today") (declared-name "today") (parent (node (document "d0") (qualified-name "Local Clock Example::Server"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")) (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Start"))) (kind "item def") (name "Start") (declared-name "Start") (parent (node (document "d0") (qualified-name "Local Clock Example"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::String"))) (kind "import") (name "String") (declared-name "String") (parent (node (document "d0") (qualified-name "Local Clock Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind transitionSource) (ordinal 0)) (authored-target "Local Clock Example::Server::ServerBehavior::waiting") (outcome (status resolved) (target (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting")))))
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind transitionSource) (ordinal 1)) (authored-target "Local Clock Example::Server::ServerBehavior::responding") (outcome (status resolved) (target (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::responding")))))
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind transitionSource) (ordinal 2)) (authored-target "Local Clock Example::Server::ServerBehavior::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::off")))))
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind transitionSource) (ordinal 3)) (authored-target "Local Clock Example::Server::ServerBehavior::waiting") (outcome (status resolved) (target (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting")))))
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind initialStateSource) (ordinal 0)) (authored-target "Local Clock Example::Server::ServerBehavior::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::off")))))
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::Server::localClock"))) (kind redefinition) (ordinal 0)) (authored-target "localClock") (outcome (status resolved) (target (node (document "d0") (qualified-name "Local Clock Example::Server::localClock")))))
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::Server::today"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "Local Clock Example::String")))))
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::Server::today"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "Local Clock Example::String")))))
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (target (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind transitionSource) (ordinal 2)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (target (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::responding"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (target (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (target (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind transitionSource) (ordinal 3)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (target (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Local Clock Example::Server::localClock"))) (target (node (document "d0") (qualified-name "Local Clock Example::Server::localClock"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Local Clock Example::Server::localClock"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Local Clock Example::Server::today"))) (target (node (document "d0") (qualified-name "Local Clock Example::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Local Clock Example::Server::today"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Local Clock Example::Server::today"))) (target (node (document "d0") (qualified-name "Local Clock Example::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Local Clock Example::Server::today"))) (kind featureTyping) (ordinal 1)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Local Clock Example::Server::localClock")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 9 20) (end 9 26)) (probe (position 9 20))
      (reference
        (source (document "d0") (qualified-name "Local Clock Example::Server::today"))
        (kind featureTyping) (ordinal 1) (authored-target "String")
        (range (start 9 20) (end 9 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Local Clock Example::String") (range (start 1 1) (end 1 37)))
        )
      )
    )
    (query (range (start 7 11) (end 7 21)) (probe (position 7 11))
      (reference
        (source (document "d0") (qualified-name "Local Clock Example::Server::localClock"))
        (kind redefinition) (ordinal 0) (authored-target "localClock")
        (range (start 7 11) (end 7 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Local Clock Example::Server::localClock") (range (start 7 2) (end 7 42)))
        )
      )
    )
    (query (range (start 1 16) (end 1 36)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Local Clock Example::String"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
        (range (start 1 16) (end 1 36))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
