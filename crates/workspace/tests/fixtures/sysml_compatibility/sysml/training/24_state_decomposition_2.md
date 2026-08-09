# META
~~~ini
description=SysML Training 24 (States): State Decomposition-2
type=file
~~~
# SOURCE
~~~sysml
package 'State Decomposition-1' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
	
	state def VehicleStates;
		
	state vehicleStates : VehicleStates parallel {
		
		state operationalStates {
			entry; then off;
			
			state off;
			accept VehicleStartSignal 
				then starting;
				
			state starting;
			accept VehicleOnSignal
				then on;
				
			state on;
			accept VehicleOffSignal
				then off;
		}
		
		state healthStates { 
			/* ... */
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwState,KwDef,Ident,Semicolon,
KwState,Ident,Colon,Ident,KwParallel,OpenCurly,
KwState,Ident,OpenCurly,
KwEntry,Semicolon,KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,
KwThen,Ident,Semicolon,
CloseCurly,
KwState,Ident,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''State Decomposition-1''
    (attribute_def 'VehicleStartSignal')
    (attribute_def 'VehicleOnSignal')
    (attribute_def 'VehicleOffSignal')
    (state_def 'VehicleStates')
    (state_usage parallel 'vehicleStates' : 'VehicleStates'
      (state_usage 'operationalStates'
        (entry_action)
        (source_succession
          (default_ref_usage 'off'))
        (state_usage 'off')
        (target_transition)
        (state_usage 'starting')
        (target_transition)
        (state_usage 'on')
        (target_transition))
      (state_usage 'healthStates'
        (comment)))))
~~~
# FORMAT
~~~sysml
package 'State Decomposition-1' {
    attribute def VehicleStartSignal;
    attribute def VehicleOnSignal;
    attribute def VehicleOffSignal;

    state def VehicleStates;

    state vehicleStates : VehicleStates parallel {
        state operationalStates {
            entry;
            then off;

            state off;
            accept VehicleStartSignal then starting;

            state starting;
            accept VehicleOnSignal then on;

            state on;
            accept VehicleOffSignal then off;
        }

        state healthStates {
            /* ... */
        }
    }
}
~~~
# EXPECTED
~~~
semantic.duplicate_name 'off'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'off'
~~~
# SMG
~~~
(model
  (namespace
    (package 'State Decomposition-1'
      (attribute_def 'VehicleStartSignal')
      (attribute_def 'VehicleOnSignal')
      (attribute_def 'VehicleOffSignal')
      (state_def 'VehicleStates')
      (state_usage parallel 'vehicleStates' : 'State Decomposition-1::VehicleStates'[state_def]
        (state_usage composite 'operationalStates'
          (state_subaction_membership 'entry'
            (action_usage))
          (source_succession
            (reference_usage reference 'off'))
          (state_usage composite 'off')
          (transition_usage)
          (state_usage composite 'starting')
          (transition_usage)
          (state_usage composite 'on')
          (transition_usage))
        (state_usage composite 'healthStates')))))
~~~
