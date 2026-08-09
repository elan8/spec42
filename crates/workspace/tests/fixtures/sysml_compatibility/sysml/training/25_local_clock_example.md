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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Local Clock Example"))) (name "Local Clock Example") (declared-name "Local Clock Example")
      (contains
        (element (kind "item def") (id (node (document "d0") (qualified-name "Local Clock Example::Request"))) (name "Request") (declared-name "Request"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Local Clock Example::Server"))) (name "Server") (declared-name "Server") (declared)
          (contains
            (element (kind "state") (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (name "ServerBehavior") (declared-name "ServerBehavior") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Local Clock Example::Server"))))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "Local Clock Example::Server")))))
                (element (kind "state") (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::off"))) (name "off") (declared-name "off") (effective (featuring-type (node (document "d0") (qualified-name "Local Clock Example::Server")))))
                (element (kind "state") (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::responding"))) (name "responding") (declared-name "responding") (effective (featuring-type (node (document "d0") (qualified-name "Local Clock Example::Server")))))
                (element (kind "transition") (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_off"))) (name "transition_ServerBehavior_to_off") (declared-name "transition_ServerBehavior_to_off") (effective (featuring-type (node (document "d0") (qualified-name "Local Clock Example::Server"))))
                  (contains
                    (element (kind "transition trigger") (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_off::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "Local Clock Example::Server")))))
                  )
                )
                (element (kind "transition") (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_responding"))) (name "transition_ServerBehavior_to_responding") (declared-name "transition_ServerBehavior_to_responding") (effective (featuring-type (node (document "d0") (qualified-name "Local Clock Example::Server"))))
                  (contains
                    (element (kind "transition trigger") (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_responding::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "Local Clock Example::Server")))))
                  )
                )
                (element (kind "transition") (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_waiting"))) (name "transition_ServerBehavior_to_waiting") (declared-name "transition_ServerBehavior_to_waiting") (effective (featuring-type (node (document "d0") (qualified-name "Local Clock Example::Server"))))
                  (contains
                    (element (kind "transition trigger") (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_waiting::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "Local Clock Example::Server")))))
                  )
                )
                (element (kind "transition") (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_waiting#transition"))) (name "transition_ServerBehavior_to_waiting") (declared-name "transition_ServerBehavior_to_waiting") (effective (featuring-type (node (document "d0") (qualified-name "Local Clock Example::Server"))))
                  (contains
                    (element (kind "transition trigger") (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::transition_ServerBehavior_to_waiting#transition::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "Local Clock Example::Server")))))
                  )
                )
                (element (kind "state") (id (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting"))) (name "waiting") (declared-name "waiting") (effective (featuring-type (node (document "d0") (qualified-name "Local Clock Example::Server")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Local Clock Example::Server::localClock"))) (name "localClock") (declared (properties (composite true) (reference false) (ordered false)) (feature-value (kind bound) (expression (kind "constructor") (reference "Time::Clock")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Local Clock Example::Server"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Local Clock Example::Server::localClock"))) (role feature-value))))
            (element (kind "port") (id (node (document "d0") (qualified-name "Local Clock Example::Server::requestPort"))) (name "requestPort") (declared-name "requestPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Local Clock Example::Server")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Local Clock Example::Server::today"))) (name "today") (declared-name "today") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Local Clock Example::Server")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "Local Clock Example::Start"))) (name "Start") (declared-name "Start"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Local Clock Example::String"))) (name "String") (declared-name "String"))
      )
    )
  )
  (relationships
    (initialState (status resolved) (from (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (to (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (to (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (to (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::responding"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior"))) (to (node (document "d0") (qualified-name "Local Clock Example::Server::ServerBehavior::waiting"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
