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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Fork Join Example"))) (name "Fork Join Example") (declared-name "Fork Join Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Fork Join Example::*"))) (name "*") (declared-name "*"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (name "Brake") (declared-name "Brake")
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Fork Join Example::Brake::TurnOn"))) (name "TurnOn") (declared-name "TurnOn") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Fork Join Example::Brake")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Fork Join Example::Brake::braking"))) (name "braking") (declared-name "braking") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Fork Join Example::Brake"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Fork Join Example::Brake::braking::brakePressure"))) (name "brakePressure") (declared-name "brakePressure") (effective (featuring-type (node (document "d0") (qualified-name "Fork Join Example::Braking")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Fork Join Example::Brake::braking::modulationFrequency"))) (name "modulationFrequency") (declared-name "modulationFrequency") (effective (featuring-type (node (document "d0") (qualified-name "Fork Join Example::Braking")))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "Fork Join Example::Brake::from"))) (name "from") (declared-name "from") (effective (featuring-type (node (document "d0") (qualified-name "Fork Join Example::Brake")))))
            (element (kind "flow") (id (node (document "d0") (qualified-name "Fork Join Example::Brake::from#flow"))) (name "from") (declared-name "from") (effective (featuring-type (node (document "d0") (qualified-name "Fork Join Example::Brake")))))
            (element (kind "join") (id (node (document "d0") (qualified-name "Fork Join Example::Brake::joinNode"))) (name "join") (declared-name "join") (effective (featuring-type (node (document "d0") (qualified-name "Fork Join Example::Brake")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal"))) (name "monitorBrakePedal") (declared-name "monitorBrakePedal") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Fork Join Example::Brake"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal::brakePressure"))) (name "brakePressure") (declared-name "brakePressure") (effective (featuring-type (node (document "d0") (qualified-name "Fork Join Example::MonitorBrakePedal")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction"))) (name "monitorTraction") (declared-name "monitorTraction") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Fork Join Example::Brake"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction::modulationFrequency"))) (name "modulationFrequency") (declared-name "modulationFrequency") (effective (featuring-type (node (document "d0") (qualified-name "Fork Join Example::MonitorTraction")))))
              )
            )
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Fork Join Example::BrakePressure"))) (name "BrakePressure") (declared-name "BrakePressure") (declared (properties (ordered false) (unique true))))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Fork Join Example::Braking"))) (name "Braking") (declared-name "Braking")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Fork Join Example::Braking::brakePressure"))) (name "brakePressure") (declared-name "brakePressure") (effective (featuring-type (node (document "d0") (qualified-name "Fork Join Example::Braking")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Fork Join Example::Braking::modulationFrequency"))) (name "modulationFrequency") (declared-name "modulationFrequency") (effective (featuring-type (node (document "d0") (qualified-name "Fork Join Example::Braking")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Fork Join Example::MonitorBrakePedal"))) (name "MonitorBrakePedal") (declared-name "MonitorBrakePedal")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Fork Join Example::MonitorBrakePedal::pressure"))) (name "pressure") (declared-name "pressure") (effective (featuring-type (node (document "d0") (qualified-name "Fork Join Example::MonitorBrakePedal")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Fork Join Example::MonitorTraction"))) (name "MonitorTraction") (declared-name "MonitorTraction")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Fork Join Example::MonitorTraction::modFreq"))) (name "modFreq") (declared-name "modFreq") (effective (featuring-type (node (document "d0") (qualified-name "Fork Join Example::MonitorTraction")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Fork Join Example::TurnKeyToOn"))) (name "TurnKeyToOn") (declared-name "TurnKeyToOn") (declared (properties (ordered false) (unique true))))
      )
    )
  )
  (relationships
    (flow (status resolved) (from (node (document "d0") (qualified-name "Fork Join Example::Brake::braking"))) (to (node (document "d0") (qualified-name "Fork Join Example::Brake::joinNode"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "Fork Join Example::Brake::joinNode"))) (to (node (document "d0") (qualified-name "Fork Join Example::Brake::joinNode"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal"))) (to (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction"))) (to (node (document "d0") (qualified-name "Fork Join Example::Brake::braking"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (to (node (document "d0") (qualified-name "Fork Join Example::Brake::TurnOn"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (to (node (document "d0") (qualified-name "Fork Join Example::Brake::braking"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (to (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (to (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Fork Join Example::Brake::braking"))) (to (node (document "d0") (qualified-name "Fork Join Example::Braking"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal"))) (to (node (document "d0") (qualified-name "Fork Join Example::MonitorBrakePedal"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction"))) (to (node (document "d0") (qualified-name "Fork Join Example::MonitorTraction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Fork Join Example::Braking::brakePressure"))) (to (node (document "d0") (qualified-name "Fork Join Example::BrakePressure"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Fork Join Example::MonitorBrakePedal::pressure"))) (to (node (document "d0") (qualified-name "Fork Join Example::BrakePressure"))))
  )
  (pending-relationships
    (flow (status pending) (document "d0") (source-qualified "Fork Join Example::Brake") (target-qualified "Fork Join Example::Brake::fork"))
    (flow (status pending) (document "d0") (source-qualified "Fork Join Example::Brake::fork") (target-qualified "Fork Join Example::Brake::monitorBrakePedal"))
    (flow (status pending) (document "d0") (source-qualified "Fork Join Example::Brake::joinNode") (target-qualified "Fork Join Example::Brake::done"))
  )
  (pending-expression-relationships
  )
)
~~~
