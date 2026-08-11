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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Parts"))) (name "Parts") (declared-name "Parts")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Parts::Action"))) (name "Action") (declared-name "Action"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Parts::Item"))) (name "Item") (declared-name "Item"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Parts::Object"))) (name "Object") (declared-name "Object"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Parts::Part"))) (name "Part") (declared-name "Part") (declared (properties (abstract true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Parts::Part::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Parts::Part")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Parts::Part::done"))) (name "done") (declared-name "done") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Parts::Part")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (name "exhibitedStates") (declared-name "exhibitedStates") (declared (properties (abstract true) (composite false) (reference true)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Parts::Part"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Parts::Part::exhibitedStates::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Parts::Part")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (name "ownedActions") (declared-name "ownedActions") (declared (properties (abstract true)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Parts::Part"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Parts::Part::ownedActions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Parts::Part")))))
                (element (kind "ref") (id (node (document "d0") (qualified-name "Parts::Part::ownedActions::part"))) (name "part") (declared-name "part") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Parts::Part"))))
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "Parts::Part::ownedActions::part::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Parts::Part")))))
                  )
                )
              )
            )
            (element (kind "port") (id (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))) (name "ownedPorts") (declared-name "ownedPorts") (declared (properties (abstract true)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Parts::Part"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Parts::Part::ownedPorts::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Parts::Part")))))
              )
            )
            (element (kind "state") (id (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (name "ownedStates") (declared-name "ownedStates") (declared (properties (abstract true)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Parts::Part"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Parts::Part::ownedStates::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Parts::Part")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (name "performedActions") (declared-name "performedActions") (declared (properties (abstract true) (composite false) (reference true)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Parts::Part"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Parts::Part::performedActions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Parts::Part")))))
              )
            )
            (element (kind "opaque member") (id (node (document "d0") (qualified-name "Parts::Part::self"))) (name "self") (declared-name "self") (effective (featuring-type (node (document "d0") (qualified-name "Parts::Part")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Parts::Part::start"))) (name "start") (declared-name "start") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Parts::Part")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Parts::Port"))) (name "Port") (declared-name "Port"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Parts::StateAction"))) (name "StateAction") (declared-name "StateAction"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Parts::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "Parts::actions"))) (name "actions") (declared-name "actions"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Parts::items"))) (name "items") (declared-name "items"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Parts::objects"))) (name "objects") (declared-name "objects"))
        (element (kind "part") (id (node (document "d0") (qualified-name "Parts::parts"))) (name "parts") (declared-name "parts") (declared (properties (abstract true) (ordered false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Parts::parts::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Parts::Part")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Parts::ports"))) (name "ports") (declared-name "ports"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Parts::stateActions"))) (name "stateActions") (declared-name "stateActions"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Parts::Part::_documentation"))) (to (node (document "d0") (qualified-name "Parts::Part"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Parts::Part::exhibitedStates::_documentation"))) (to (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Parts::Part::ownedActions::_documentation"))) (to (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Parts::Part::ownedActions::part::_documentation"))) (to (node (document "d0") (qualified-name "Parts::Part::ownedActions::part"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Parts::Part::ownedPorts::_documentation"))) (to (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Parts::Part::ownedStates::_documentation"))) (to (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Parts::Part::performedActions::_documentation"))) (to (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Parts::_documentation"))) (to (node (document "d0") (qualified-name "Parts"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Parts::parts::_documentation"))) (to (node (document "d0") (qualified-name "Parts::parts"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (to (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (to (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Parts::parts"))) (to (node (document "d0") (qualified-name "Parts::items"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Parts::Part::done"))) (to (node (document "d0") (qualified-name "Parts::Part"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Parts::Part::start"))) (to (node (document "d0") (qualified-name "Parts::Part"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Parts::parts"))) (to (node (document "d0") (qualified-name "Parts::Part"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts::Part"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts::Part::done"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts::Part::start"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts::parts"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/parts.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 18 1) (end 18 1266))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 27 2) (end 27 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 28 2) (end 28 33))
      )
    )
  )
)
~~~
