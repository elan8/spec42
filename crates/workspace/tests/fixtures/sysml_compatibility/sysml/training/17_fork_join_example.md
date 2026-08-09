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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwAction,KwDef,Ident,OpenCurly,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwAction,Ident,Semicolon,
KwThen,KwFork,Semicolon,
KwThen,Ident,Semicolon,
KwThen,Ident,Semicolon,
KwThen,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwOut,Ident,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwOut,Ident,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Semicolon,
KwIn,Ident,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
KwJoin,Ident,Semicolon,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Fork Join Example''
    (import_decl private 'ScalarValues::*')
    (attribute_def 'TurnKeyToOn')
    (attribute_def 'BrakePressure')
    (action_def 'MonitorBrakePedal'
      (default_ref_usage out 'pressure' : 'BrakePressure'))
    (action_def 'MonitorTraction'
      (default_ref_usage out 'modFreq' : 'Real'))
    (action_def 'Braking'
      (default_ref_usage in 'brakePressure' : 'BrakePressure')
      (default_ref_usage in 'modulationFrequency' : 'Real'))
    (action_def 'Brake'
      (action_usage 'TurnOn')
      (source_succession
        (sysml_decl))
      (source_succession
        (default_ref_usage 'monitorBrakePedal'))
      (source_succession
        (default_ref_usage 'monitorTraction'))
      (source_succession
        (default_ref_usage 'braking'))
      (action_usage 'monitorBrakePedal' : 'MonitorBrakePedal'
        (default_ref_usage out 'brakePressure'))
      (source_succession
        (default_ref_usage 'joinNode'))
      (action_usage 'monitorTraction' : 'MonitorTraction'
        (default_ref_usage out 'modulationFrequency'))
      (source_succession
        (default_ref_usage 'joinNode'))
      (flow_usage
        (connector_end)
        (connector_end))
      (flow_usage
        (connector_end)
        (connector_end))
      (action_usage 'braking' : 'Braking'
        (default_ref_usage in 'brakePressure')
        (default_ref_usage in 'modulationFrequency'))
      (source_succession
        (default_ref_usage 'joinNode'))
      (sysml_decl 'joinNode')
      (source_succession
        (default_ref_usage 'done')))))
~~~
# FORMAT
~~~sysml
package 'Fork Join Example' {
    private import ScalarValues::*;

    attribute def TurnKeyToOn;
    attribute def BrakePressure;

    action def MonitorBrakePedal {
        out pressure : BrakePressure;
    }
    action def MonitorTraction {
        out modFreq : Real;
    }
    action def Braking {
        in brakePressure : BrakePressure;
        in modulationFrequency : Real;
    }

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
# EXPECTED
~~~
semantic.duplicate_name 'monitorBrakePedal'
semantic.duplicate_name 'monitorTraction'
semantic.duplicate_name 'joinNode'
semantic.duplicate_name 'braking'
semantic.duplicate_name 'joinNode'
semantic.duplicate_name 'joinNode'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'monitorBrakePedal'
semantic.duplicate_name 'monitorTraction'
semantic.duplicate_name 'joinNode'
semantic.duplicate_name 'braking'
semantic.duplicate_name 'joinNode'
semantic.duplicate_name 'joinNode'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Fork Join Example'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (attribute_def 'TurnKeyToOn')
      (attribute_def 'BrakePressure')
      (action_def 'MonitorBrakePedal'
        (reference_usage out reference 'pressure' : 'Fork Join Example::BrakePressure'[attribute_def]))
      (action_def 'MonitorTraction'
        (reference_usage out reference 'modFreq' : 'Real'[unresolved]))
      (action_def 'Braking'
        (reference_usage in reference 'brakePressure' : 'Fork Join Example::BrakePressure'[attribute_def])
        (reference_usage in reference 'modulationFrequency' : 'Real'[unresolved]))
      (action_def 'Brake'
        (action_usage composite 'TurnOn')
        (source_succession
          (fork_node))
        (source_succession
          (reference_usage reference 'monitorBrakePedal'))
        (source_succession
          (reference_usage reference 'monitorTraction'))
        (source_succession
          (reference_usage reference 'braking'))
        (action_usage composite 'monitorBrakePedal' : 'Fork Join Example::MonitorBrakePedal'[action_def]
          (reference_usage out reference 'brakePressure'))
        (source_succession
          (reference_usage reference 'joinNode'))
        (action_usage composite 'monitorTraction' : 'Fork Join Example::MonitorTraction'[action_def]
          (reference_usage out reference 'modulationFrequency'))
        (source_succession
          (reference_usage reference 'joinNode'))
        (flow_usage composite
          (connector_end 'monitorBrakePedal.brakePressure')
          (connector_end 'braking.brakePressure'))
        (flow_usage composite
          (connector_end 'monitorTraction.modulationFrequency')
          (connector_end 'braking.modulationFrequency'))
        (action_usage composite 'braking' : 'Fork Join Example::Braking'[action_def]
          (reference_usage in reference 'brakePressure')
          (reference_usage in reference 'modulationFrequency'))
        (source_succession
          (reference_usage reference 'joinNode'))
        (join_node 'joinNode')
        (source_succession
          (reference_usage reference 'done'))))))
~~~
