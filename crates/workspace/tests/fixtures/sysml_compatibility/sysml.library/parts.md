# META
~~~ini
description=Standard Library: Systems Library/Parts
type=file
~~~
# SOURCE
~~~sysml
standard library package Parts {
doc
/*
 * This package defines the base types for parts and related structural elements in the
 * SysML language.
 */

	private import Objects::Object;
	private import Objects::objects;
	private import Items::Item;
	private import Items::items;
	private import Ports::Port;
	private import Ports::ports;
	private import Actions::Action;
	private import Actions::actions;
	private import States::StateAction;
	private import States::stateActions;
	
	abstract part def Part :> Item {
		doc
		/*
		 * Part is the most general class of objects that represent all or a part of a system.
		 * Part is the base type of all PartDefinitions.
		 */
	
		ref self: Part :>> Item::self;
		
		part start: Part :>> Item::start;
		part done: Part :>> Item::done;
		
		abstract port ownedPorts: Port[0..*] :> ports, timeEnclosedOccurrences {
			doc
			/*
			 * Ports that are owned by this Part.
			 */
		}
		
		abstract ref action performedActions: Action[0..*] :> actions, enactedPerformances {
			doc
			/*
			 * Actions that are performed by this Part.
			 */
		}
		
		abstract action ownedActions: Action[0..*] :> actions, ownedPerformances {
			doc
			/*
			 * Actions that are owned by this Part.
			 */
		
		 	ref part this : Part :>> Action::this, ownedPerformances::this = that as Part {
				doc
				/*
				 * The "this" reference of an ownedAction is always its owning Part.
				 */
			}
		}
		
		abstract ref state exhibitedStates: StateAction[0..*] :> stateActions, performedActions {
			doc
			/*
			 * StateActions that are exhibited by this Part.
			 */
		}
		
		abstract state ownedStates: StateAction[0..*] :> stateActions, ownedActions {
			doc
			/*
			 * StateActions that are owned by this Part.
			 */
		}
	}
	
	abstract part parts: Part[0..*] nonunique :> items {
		doc
		/*
		 * parts is the base feature of all part properties.
		 */
	}
	
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Item'
semantic.unresolved_name 'Item::self'
semantic.unresolved_name 'Item::start'
semantic.unresolved_name 'Item::done'
semantic.unresolved_name 'Port'
semantic.unresolved_name 'ports'
semantic.unresolved_name 'timeEnclosedOccurrences'
semantic.unresolved_name 'Action'
semantic.unresolved_name 'actions'
semantic.unresolved_name 'enactedPerformances'
semantic.unresolved_name 'Action'
semantic.unresolved_name 'actions'
semantic.unresolved_name 'ownedPerformances'
semantic.unresolved_name 'Action::this'
semantic.unresolved_name 'ownedPerformances::this'
semantic.unresolved_name 'StateAction'
semantic.unresolved_name 'stateActions'
semantic.unresolved_name 'StateAction'
semantic.unresolved_name 'stateActions'
semantic.unresolved_name 'items'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Item'
semantic.unresolved_name 'Item::self'
semantic.unresolved_name 'Item::start'
semantic.unresolved_name 'Item::done'
semantic.unresolved_name 'Port'
semantic.unresolved_name 'ports'
semantic.unresolved_name 'timeEnclosedOccurrences'
semantic.unresolved_name 'Action'
semantic.unresolved_name 'actions'
semantic.unresolved_name 'enactedPerformances'
semantic.unresolved_name 'Action'
semantic.unresolved_name 'actions'
semantic.unresolved_name 'ownedPerformances'
semantic.unresolved_name 'Action::this'
semantic.unresolved_name 'ownedPerformances::this'
semantic.unresolved_name 'StateAction'
semantic.unresolved_name 'stateActions'
semantic.unresolved_name 'StateAction'
semantic.unresolved_name 'stateActions'
semantic.unresolved_name 'items'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwPort,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwRef,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwPart,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Eq,Ident,KwAs,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwRef,KwState,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwState,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Parts'
    (documentation)
    (import_decl private 'Objects::Object')
    (import_decl private 'Objects::objects')
    (import_decl private 'Items::Item')
    (import_decl private 'Items::items')
    (import_decl private 'Ports::Port')
    (import_decl private 'Ports::ports')
    (import_decl private 'Actions::Action')
    (import_decl private 'Actions::actions')
    (import_decl private 'States::StateAction')
    (import_decl private 'States::stateActions')
    (part_def abstract 'Part' :> 'Item'
      (documentation)
      (ref_usage ref 'self' : 'Part' :>> 'Item::self')
      (part_usage 'start' : 'Part' :>> 'Item::start')
      (part_usage 'done' : 'Part' :>> 'Item::done')
      (port_usage abstract 'ownedPorts' : 'Port' :> 'ports', 'timeEnclosedOccurrences' multiplicity
        (documentation))
      (action_usage abstract ref 'performedActions' : 'Action' multiplicity :> 'actions', 'enactedPerformances'
        (documentation))
      (action_usage abstract 'ownedActions' : 'Action' multiplicity :> 'actions', 'ownedPerformances'
        (documentation)
        (part_usage ref 'this' : 'Part' :>> 'Action::this', 'ownedPerformances::this' value
          (documentation)))
      (state_usage abstract ref 'exhibitedStates' : 'StateAction' :> 'stateActions', 'performedActions' multiplicity
        (documentation))
      (state_usage abstract 'ownedStates' : 'StateAction' :> 'stateActions', 'ownedActions' multiplicity
        (documentation)))
    (part_usage abstract 'parts' : 'Part' :> 'items' multiplicity nonunique
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package Parts {
    doc /*
 * This package defines the base types for parts and related structural elements in the
 * SysML language.
 */

    private import Objects::Object;
    private import Objects::objects;
    private import Items::Item;
    private import Items::items;
    private import Ports::Port;
    private import Ports::ports;
    private import Actions::Action;
    private import Actions::actions;
    private import States::StateAction;
    private import States::stateActions;

    abstract part def Part :> Item {
        doc /*
		 * Part is the most general class of objects that represent all or a part of a system.
		 * Part is the base type of all PartDefinitions.
		 */

        ref self : Part :>> Item::self;

        part start : Part :>> Item::start;
        part done : Part :>> Item::done;

        abstract port ownedPorts : Port :> ports, timeEnclosedOccurrences [0..*] {
            doc /*
			 * Ports that are owned by this Part.
			 */
        }

        abstract ref action performedActions : Action [0..*] :> actions, enactedPerformances {
            doc /*
			 * Actions that are performed by this Part.
			 */
        }

        abstract action ownedActions : Action [0..*] :> actions, ownedPerformances {
            doc /*
			 * Actions that are owned by this Part.
			 */

            ref part this : Part :>> Action::this, ownedPerformances::this = that as Part {
                doc /*
				 * The "this" reference of an ownedAction is always its owning Part.
				 */
            }
        }

        abstract ref state exhibitedStates : StateAction :> stateActions, performedActions [0..*] {
            doc /*
			 * StateActions that are exhibited by this Part.
			 */
        }

        abstract state ownedStates : StateAction :> stateActions, ownedActions [0..*] {
            doc /*
			 * StateActions that are owned by this Part.
			 */
        }
    }

    abstract part parts : Part :> items [0..*] nonunique {
        doc /*
		 * parts is the base feature of all part properties.
		 */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Parts'
      (documentation)
      (membership_import private -> 'Objects::Object'[unresolved])
      (membership_import private -> 'Objects::objects'[unresolved])
      (membership_import private -> 'Items::Item'[unresolved])
      (membership_import private -> 'Items::items'[unresolved])
      (membership_import private -> 'Ports::Port'[unresolved])
      (membership_import private -> 'Ports::ports'[unresolved])
      (membership_import private -> 'Actions::Action'[unresolved])
      (membership_import private -> 'Actions::actions'[unresolved])
      (membership_import private -> 'States::StateAction'[unresolved])
      (membership_import private -> 'States::stateActions'[unresolved])
      (part_def abstract 'Part' :> 'Item'[unresolved]
        (documentation)
        (reference_usage reference 'self' : 'Parts::Part'[part_def] :>> 'Item::self'[unresolved])
        (part_usage composite 'start' : 'Parts::Part'[part_def] :>> 'Item::start'[unresolved] :> 'Parts::parts'[part_usage][implied])
        (part_usage composite 'done' : 'Parts::Part'[part_def] :>> 'Item::done'[unresolved] :> 'Parts::parts'[part_usage][implied])
        (port_usage abstract composite 'ownedPorts' : 'Port'[unresolved] :> 'ports'[unresolved] :> 'timeEnclosedOccurrences'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (action_usage abstract reference 'performedActions' : 'Action'[unresolved] :> 'actions'[unresolved] :> 'enactedPerformances'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (action_usage abstract composite 'ownedActions' : 'Action'[unresolved] :> 'actions'[unresolved] :> 'ownedPerformances'[unresolved]
          (multiplicity_range [0..*])
          (documentation)
          (part_usage reference 'this' : 'Parts::Part'[part_def] :>> 'Action::this'[unresolved] :>> 'ownedPerformances::this'[unresolved] :> 'Parts::parts'[part_usage][implied]
            (feature_value (=))
            (documentation)))
        (state_usage abstract reference 'exhibitedStates' : 'StateAction'[unresolved] :> 'stateActions'[unresolved] :> 'Parts::Part::performedActions'[action_usage]
          (multiplicity_range [0..*])
          (documentation))
        (state_usage abstract composite 'ownedStates' : 'StateAction'[unresolved] :> 'stateActions'[unresolved] :> 'Parts::Part::ownedActions'[action_usage]
          (multiplicity_range [0..*])
          (documentation)))
      (part_usage abstract 'parts' : 'Parts::Part'[part_def] :> 'items'[unresolved]
        (multiplicity_range [0..*])
        (documentation)))))
~~~
