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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 36))
      )
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
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 11 2) (end 11 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 21 3) (end 22 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 17) (end 23 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:c87210a718fb52b6e4333f862eb8c447e5365a87e88fe8ce7597a09f5281e838") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Request"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "localClock")))))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "off")))))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "waiting")) (transitionTrigger (reference "Start")))))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 1))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "responding")))))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "today")) (transitionTarget (reference "off")) (invocationCallee (reference "Time::Iso8601DateTime")))))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 3))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "waiting")))))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 3)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::responding"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::requestPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::today"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Start"))) (kind item-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::off")))))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2))))) (kind expressionOperand) (ordinal 0))
      (authored-target "today")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::today")))))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "waiting")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting")))))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0))
      (authored-target "responding")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::responding")))))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::off")))))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 3))))) (kind transitionTarget) (ordinal 0))
      (authored-target "waiting")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting")))))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Start")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Start")))))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Time::Iso8601DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::today"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::today"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::responding"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 3))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 3))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Start"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 3))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 3)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 3))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::off"))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::responding"))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting"))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::requestPort"))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::today"))) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server"))) (provenance implied))
  )
  (evaluation
    (unit (declaration (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 3))))) (ordinal 0) (authored "SI::min") (start 27 19) (end 27 26) (outcome (status catalog-unavailable)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server")))
    )
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior")))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server")))
    )
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior")))
    )
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior")))
    )
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior")))
    )
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior")))
    )
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 3)))))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior")))
    )
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2)))))
    )
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 3)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 3)))))
    )
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::off")))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior")))
    )
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::responding")))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior")))
    )
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting")))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior")))
    )
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::requestPort")))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server")))
    )
    (declaration (id (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::today")))
      (featured-by (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/25_local_clock_example.md") (range (start 1 16) (end 1 36)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/25_local_clock_example.md") (range (start 7 11) (end 7 21)) (probe (position 7 11))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "localClock")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/25_local_clock_example.md") (range (start 14 15) (end 14 18)) (probe (position 14 15))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::off")))))
    )
  )
  (query (document "memory://snapshot/25_local_clock_example.md") (range (start 23 39) (end 23 44)) (probe (position 23 39))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2))))) (kind expressionOperand) (ordinal 0) (authored-target "today")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::today")))))
    )
  )
  (query (document "memory://snapshot/25_local_clock_example.md") (range (start 18 9) (end 18 16)) (probe (position 18 9))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "waiting")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting")))))
    )
  )
  (query (document "memory://snapshot/25_local_clock_example.md") (range (start 22 9) (end 22 19)) (probe (position 22 9))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0) (authored-target "responding")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::responding")))))
    )
  )
  (query (document "memory://snapshot/25_local_clock_example.md") (range (start 24 9) (end 24 12)) (probe (position 24 9))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::off")))))
    )
  )
  (query (document "memory://snapshot/25_local_clock_example.md") (range (start 28 9) (end 28 16)) (probe (position 28 9))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 3))))) (kind transitionTarget) (ordinal 0) (authored-target "waiting")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting")))))
    )
  )
  (query (document "memory://snapshot/25_local_clock_example.md") (range (start 17 10) (end 17 15)) (probe (position 17 10))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0) (authored-target "Start")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Start")))))
    )
  )
  (query (document "memory://snapshot/25_local_clock_example.md") (range (start 23 17) (end 23 38)) (probe (position 23 17))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (path (named (kind package) (name "Local Clock Example")) (named (kind part-def) (name "Server")) (named (kind state) (name "ServerBehavior")) (anonymous (kind transition) (ordinal 2))))) (kind invocationCallee) (ordinal 0) (authored-target "Time::Iso8601DateTime")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/25_local_clock_example.md") (range (start 9 20) (end 9 26)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/25_local_clock_example.md") (qualified-name "Local Clock Example::Server::today"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
)
~~~
