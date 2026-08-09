# META
~~~ini
description=SysML Training 24 (States): State Decomposition-1
type=file
~~~
# SOURCE
~~~sysml
package 'State Decomposition-1' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
	
	state def VehicleStates;
		
	state vehicleStates : VehicleStates {
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
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwState,KwDef,Ident,Semicolon,
KwState,Ident,Colon,Ident,OpenCurly,
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
    (state_usage 'vehicleStates' : 'VehicleStates'
      (entry_action)
      (source_succession
        (default_ref_usage 'off'))
      (state_usage 'off')
      (target_transition)
      (state_usage 'starting')
      (target_transition)
      (state_usage 'on')
      (target_transition))))
~~~
# FORMAT
~~~sysml
package 'State Decomposition-1' {
    attribute def VehicleStartSignal;
    attribute def VehicleOnSignal;
    attribute def VehicleOffSignal;

    state def VehicleStates;

    state vehicleStates : VehicleStates {
        entry;
        then off;

        state off;
        accept VehicleStartSignal then starting;

        state starting;
        accept VehicleOnSignal then on;

        state on;
        accept VehicleOffSignal then off;
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
      (state_usage 'vehicleStates' : 'State Decomposition-1::VehicleStates'[state_def]
        (state_subaction_membership 'entry'
          (action_usage))
        (source_succession
          (reference_usage reference 'off'))
        (state_usage composite 'off')
        (transition_usage)
        (state_usage composite 'starting')
        (transition_usage)
        (state_usage composite 'on')
        (transition_usage)))))
~~~
