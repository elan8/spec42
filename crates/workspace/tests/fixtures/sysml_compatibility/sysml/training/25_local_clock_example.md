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
            entry;
            then off;

            state off;
            accept Start via requestPort then waiting;

            state waiting;
            accept request : Request via requestPort then responding;
            accept at new Time :: Iso8601DateTime ( today + "11:59:00" ) then off;

            state responding;
            accept after 5 [ SI :: min ] then waiting;
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
(model
  (namespace
    (package 'Local Clock Example'
      (membership_import private -> 'ScalarValues::String'[unresolved])
      (item_def 'Start')
      (item_def 'Request')
      (part_def 'Server'
        (part_usage composite :>> 'localClock'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'today' : 'String'[unresolved])
        (port_usage composite 'requestPort')
        (state_usage composite 'ServerBehavior'
          (state_subaction_membership 'entry'
            (action_usage))
          (source_succession
            (reference_usage reference 'off'))
          (state_usage composite 'off')
          (transition_usage)
          (state_usage composite 'waiting')
          (transition_usage)
          (transition_usage)
          (state_usage composite 'responding')
          (transition_usage))))))
~~~
