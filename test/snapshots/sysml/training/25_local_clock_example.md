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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,Eq,Ident,Ident,ColonColon,Ident,OpenParen,CloseParen,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Semicolon,
KwState,Ident,OpenCurly,
KwEntry,Semicolon,KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,KwVia,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,Colon,Ident,KwVia,Ident,
KwThen,Ident,Semicolon,
KwAccept,Ident,Ident,Ident,ColonColon,Ident,OpenParen,Ident,Plus,StringValue,CloseParen,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,KwAfter,DecimalValue,OpenSquare,Ident,ColonColon,Ident,CloseSquare,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Local Clock Example''
    (import_decl private 'ScalarValues::String')
    (item_def 'Start')
    (item_def 'Request')
    (part_def 'Server'
      (part_usage :>> 'localClock' value)
      (attribute_usage 'today' : 'String')
      (port_usage 'requestPort')
      (state_usage 'ServerBehavior'
        (entry_action)
        (source_succession
          (default_ref_usage 'off'))
        (state_usage 'off')
        (target_transition)
        (state_usage 'waiting')
        (target_transition)
        (target_transition)
        (state_usage 'responding')
        (target_transition)))))
~~~
# EXPECTED
~~~
semantic.duplicate_name 'off'
semantic.unresolved_name 'localClock'
semantic.unresolved_name 'String'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'off'
semantic.unresolved_name 'localClock'
semantic.unresolved_name 'String'
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f2c06ab46cdcc62799091ebe0852bc26ed4d8b94b27af787d4deef8fff254c5b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Local Clock Example"))) (kind "package") (name "Local Clock Example") (declared-name "Local Clock Example") (range (start (line 0) (character 0)) (end (line 0) (character 581))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Request"))) (kind "item def") (name "Request") (declared-name "Request") (range (start (line 4) (character 1)) (end (line 4) (character 18))) (parent (node (document "d0") (qualified-name "Local Clock Example"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server"))) (kind "part def") (name "Server") (declared-name "Server") (range (start (line 6) (character 1)) (end (line 6) (character 469))) (parent (node (document "d0") (qualified-name "Local Clock Example"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind "state") (name "ServerBehavior") (declared-name "ServerBehavior") (range (start (line 13) (character 2)) (end (line 13) (character 347))) (parent (node (document "d0") (qualified-name "Local Clock Example::Server"))) (authored (membership (kind Feature)) (relationships (transition (reference "Local Clock Example::Server::ServerBehavior::waiting") (range none)) (transition (reference "Local Clock Example::Server::ServerBehavior::responding") (range none)) (transition (reference "Local Clock Example::Server::ServerBehavior::off") (range none)) (transition (reference "Local Clock Example::Server::ServerBehavior::waiting") (range none)) (initial-state (reference "Local Clock Example::Server::ServerBehavior::off") (range none)))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 14) (character 3)) (end (line 14) (character 9))) (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::off"))) (kind "state") (name "off") (declared-name "off") (range (start (line 16) (character 3)) (end (line 16) (character 13))) (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::responding"))) (kind "state") (name "responding") (declared-name "responding") (range (start (line 26) (character 3)) (end (line 26) (character 20))) (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_off"))) (kind "transition") (name "transition_ServerBehavior_to_off") (declared-name "transition_ServerBehavior_to_off") (range (start (line 23) (character 3)) (end (line 23) (character 72))) (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 23) (character 3)) (end (line 23) (character 72))) (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_off"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_responding"))) (kind "transition") (name "transition_ServerBehavior_to_responding") (declared-name "transition_ServerBehavior_to_responding") (range (start (line 21) (character 3)) (end (line 21) (character 64))) (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_responding::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 21) (character 3)) (end (line 21) (character 64))) (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_responding"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_waiting"))) (kind "transition") (name "transition_ServerBehavior_to_waiting") (declared-name "transition_ServerBehavior_to_waiting") (range (start (line 17) (character 3)) (end (line 17) (character 49))) (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_waiting#transition"))) (kind "transition") (name "transition_ServerBehavior_to_waiting") (declared-name "transition_ServerBehavior_to_waiting") (range (start (line 27) (character 3)) (end (line 27) (character 45))) (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_waiting#transition::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 27) (character 3)) (end (line 27) (character 45))) (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_waiting#transition"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_waiting::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 17) (character 3)) (end (line 17) (character 49))) (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_waiting"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting"))) (kind "state") (name "waiting") (declared-name "waiting") (range (start (line 20) (character 3)) (end (line 20) (character 17))) (parent (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::localClock"))) (kind "part") (name "localClock") (range (start (line 7) (character 2)) (end (line 7) (character 42))) (parent (node (document "d0") (qualified-name "Local Clock Example::Server"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "localClock") (range (start (line 7) (character 11)) (end (line 7) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::requestPort"))) (kind "port") (name "requestPort") (declared-name "requestPort") (range (start (line 11) (character 2)) (end (line 11) (character 19))) (parent (node (document "d0") (qualified-name "Local Clock Example::Server"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Server::today"))) (kind "attribute") (name "today") (declared-name "today") (range (start (line 9) (character 2)) (end (line 9) (character 27))) (parent (node (document "d0") (qualified-name "Local Clock Example::Server"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 9) (character 20)) (end (line 9) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::Start"))) (kind "item def") (name "Start") (declared-name "Start") (range (start (line 3) (character 1)) (end (line 3) (character 16))) (parent (node (document "d0") (qualified-name "Local Clock Example"))))
    (element (id (node (document "d0") (qualified-name "Local Clock Example::String"))) (kind "import") (name "String") (declared-name "String") (range (start (line 1) (character 1)) (end (line 1) (character 37))) (parent (node (document "d0") (qualified-name "Local Clock Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 36))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind transitionSource) (ordinal 0)) (authored-target "Local Clock Example::Server::ServerBehavior::waiting") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting")))))
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind transitionSource) (ordinal 1)) (authored-target "Local Clock Example::Server::ServerBehavior::responding") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::responding")))))
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind transitionSource) (ordinal 2)) (authored-target "Local Clock Example::Server::ServerBehavior::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::off")))))
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind transitionSource) (ordinal 3)) (authored-target "Local Clock Example::Server::ServerBehavior::waiting") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting")))))
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (kind initialStateSource) (ordinal 0)) (authored-target "Local Clock Example::Server::ServerBehavior::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::off")))))
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::Server::localClock"))) (kind redefinition) (ordinal 0)) (authored-target "localClock") (range (start (line 7) (character 11)) (end (line 7) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Local Clock Example::Server::localClock")))))
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::Server::today"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Local Clock Example::String")))))
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::Server::today"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 9) (character 20)) (end (line 9) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Local Clock Example::String")))))
    (reference (id (source (node (document "d0") (qualified-name "Local Clock Example::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (range (start (line 1) (character 16)) (end (line 1) (character 36))) (outcome (status unresolved)))
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
