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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parts.md"
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 23) (end 27 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 22) (end 28 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 30 49) (end 30 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 65) (end 37 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 44 57) (end 44 74))
      )
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b1a2e6bbe50cef288c0718fbb55ad4407154fe15db567140d5ffab5b132e7ea4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Parts"))) (kind "package") (name "Parts") (declared-name "Parts") (range (start (line 0) (character 0)) (end (line 0) (character 1882))))
    (element (id (node (document "d0") (qualified-name "Parts::Action"))) (kind "import") (name "Action") (declared-name "Action") (range (start (line 13) (character 1)) (end (line 13) (character 32))) (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::Action") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 16)) (end (line 13) (character 31))))))
    (element (id (node (document "d0") (qualified-name "Parts::Item"))) (kind "import") (name "Item") (declared-name "Item") (range (start (line 9) (character 1)) (end (line 9) (character 28))) (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "Items::Item") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 27))))))
    (element (id (node (document "d0") (qualified-name "Parts::Object"))) (kind "import") (name "Object") (declared-name "Object") (range (start (line 7) (character 1)) (end (line 7) (character 32))) (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Object") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 31))))))
    (element (id (node (document "d0") (qualified-name "Parts::Part"))) (kind "part def") (name "Part") (declared-name "Part") (range (start (line 18) (character 1)) (end (line 18) (character 1266))) (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Item") (range (start (line 18) (character 27)) (end (line 18) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::_documentation"))) (kind "documentation") (name "") (range (start (line 18) (character 1)) (end (line 18) (character 1266))) (parent (node (document "d0") (qualified-name "Parts::Part"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::done"))) (kind "part") (name "done") (declared-name "done") (range (start (line 28) (character 2)) (end (line 28) (character 33))) (parent (node (document "d0") (qualified-name "Parts::Part"))) (authored (membership (kind Feature)) (relationships (typing (reference "Part") (range (start (line 28) (character 13)) (end (line 28) (character 17)))) (redefinition (reference "Item::done") (range (start (line 28) (character 22)) (end (line 28) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (kind "state") (name "exhibitedStates") (declared-name "exhibitedStates") (range (start (line 58) (character 2)) (end (line 58) (character 167))) (parent (node (document "d0") (qualified-name "Parts::Part"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateAction") (range none)) (subsetting (reference "stateActions") (range (start (line 58) (character 59)) (end (line 58) (character 71)))) (subsetting (reference "performedActions") (range (start (line 58) (character 73)) (end (line 58) (character 89)))))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::exhibitedStates::_documentation"))) (kind "documentation") (name "") (range (start (line 58) (character 2)) (end (line 58) (character 167))) (parent (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (kind "action") (name "ownedActions") (declared-name "ownedActions") (range (start (line 44) (character 2)) (end (line 44) (character 331))) (parent (node (document "d0") (qualified-name "Parts::Part"))) (authored (membership (kind Feature)) (relationships (typing (reference "Action") (range none)) (subsetting (reference "actions") (range (start (line 44) (character 48)) (end (line 44) (character 55)))) (subsetting (reference "ownedPerformances") (range (start (line 44) (character 57)) (end (line 44) (character 74)))))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::ownedActions::_documentation"))) (kind "documentation") (name "") (range (start (line 44) (character 2)) (end (line 44) (character 331))) (parent (node (document "d0") (qualified-name "Parts::Part::ownedActions"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::ownedActions::part"))) (kind "ref") (name "part") (declared-name "part") (range (start (line 50) (character 4)) (end (line 50) (character 184))) (parent (node (document "d0") (qualified-name "Parts::Part::ownedActions"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::ownedActions::part::_documentation"))) (kind "documentation") (name "") (range (start (line 50) (character 4)) (end (line 50) (character 184))) (parent (node (document "d0") (qualified-name "Parts::Part::ownedActions::part"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))) (kind "port") (name "ownedPorts") (declared-name "ownedPorts") (range (start (line 30) (character 2)) (end (line 30) (character 139))) (parent (node (document "d0") (qualified-name "Parts::Part"))) (authored (membership (kind Feature)) (relationships (typing (reference "Port") (range none)) (subsetting (reference "ports") (range (start (line 30) (character 42)) (end (line 30) (character 47)))) (subsetting (reference "timeEnclosedOccurrences") (range (start (line 30) (character 49)) (end (line 30) (character 72)))))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::ownedPorts::_documentation"))) (kind "documentation") (name "") (range (start (line 30) (character 2)) (end (line 30) (character 139))) (parent (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (kind "state") (name "ownedStates") (declared-name "ownedStates") (range (start (line 65) (character 2)) (end (line 65) (character 151))) (parent (node (document "d0") (qualified-name "Parts::Part"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateAction") (range none)) (subsetting (reference "stateActions") (range (start (line 65) (character 51)) (end (line 65) (character 63)))) (subsetting (reference "ownedActions") (range (start (line 65) (character 65)) (end (line 65) (character 77)))))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::ownedStates::_documentation"))) (kind "documentation") (name "") (range (start (line 65) (character 2)) (end (line 65) (character 151))) (parent (node (document "d0") (qualified-name "Parts::Part::ownedStates"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (kind "action") (name "performedActions") (declared-name "performedActions") (range (start (line 37) (character 2)) (end (line 37) (character 157))) (parent (node (document "d0") (qualified-name "Parts::Part"))) (authored (membership (kind Feature)) (relationships (typing (reference "Action") (range none)) (subsetting (reference "actions") (range (start (line 37) (character 56)) (end (line 37) (character 63)))) (subsetting (reference "enactedPerformances") (range (start (line 37) (character 65)) (end (line 37) (character 84)))))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::performedActions::_documentation"))) (kind "documentation") (name "") (range (start (line 37) (character 2)) (end (line 37) (character 157))) (parent (node (document "d0") (qualified-name "Parts::Part::performedActions"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::self"))) (kind "opaque member") (name "self") (declared-name "self") (range (start (line 25) (character 2)) (end (line 25) (character 32))) (parent (node (document "d0") (qualified-name "Parts::Part"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::start"))) (kind "part") (name "start") (declared-name "start") (range (start (line 27) (character 2)) (end (line 27) (character 35))) (parent (node (document "d0") (qualified-name "Parts::Part"))) (authored (membership (kind Feature)) (relationships (typing (reference "Part") (range (start (line 27) (character 14)) (end (line 27) (character 18)))) (redefinition (reference "Item::start") (range (start (line 27) (character 23)) (end (line 27) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "Parts::Port"))) (kind "import") (name "Port") (declared-name "Port") (range (start (line 11) (character 1)) (end (line 11) (character 28))) (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "Ports::Port") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 27))))))
    (element (id (node (document "d0") (qualified-name "Parts::StateAction"))) (kind "import") (name "StateAction") (declared-name "StateAction") (range (start (line 15) (character 1)) (end (line 15) (character 36))) (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "States::StateAction") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 16)) (end (line 15) (character 35))))))
    (element (id (node (document "d0") (qualified-name "Parts::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1882))) (parent (node (document "d0") (qualified-name "Parts"))))
    (element (id (node (document "d0") (qualified-name "Parts::actions"))) (kind "import") (name "actions") (declared-name "actions") (range (start (line 14) (character 1)) (end (line 14) (character 33))) (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::actions") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 16)) (end (line 14) (character 32))))))
    (element (id (node (document "d0") (qualified-name "Parts::items"))) (kind "import") (name "items") (declared-name "items") (range (start (line 10) (character 1)) (end (line 10) (character 29))) (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "Items::items") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Parts::objects"))) (kind "import") (name "objects") (declared-name "objects") (range (start (line 8) (character 1)) (end (line 8) (character 33))) (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::objects") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 32))))))
    (element (id (node (document "d0") (qualified-name "Parts::parts"))) (kind "part") (name "parts") (declared-name "parts") (range (start (line 73) (character 1)) (end (line 73) (character 128))) (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Feature)) (relationships (typing (reference "Part") (range (start (line 73) (character 22)) (end (line 73) (character 26)))) (subsetting (reference "items") (range (start (line 73) (character 46)) (end (line 73) (character 51)))))))
    (element (id (node (document "d0") (qualified-name "Parts::parts::_documentation"))) (kind "documentation") (name "") (range (start (line 73) (character 1)) (end (line 73) (character 128))) (parent (node (document "d0") (qualified-name "Parts::parts"))))
    (element (id (node (document "d0") (qualified-name "Parts::ports"))) (kind "import") (name "ports") (declared-name "ports") (range (start (line 12) (character 1)) (end (line 12) (character 29))) (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "Ports::ports") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Parts::stateActions"))) (kind "import") (name "stateActions") (declared-name "stateActions") (range (start (line 16) (character 1)) (end (line 16) (character 37))) (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "States::stateActions") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 16)) (end (line 16) (character 36))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Parts::Action"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::Action") (range (start (line 13) (character 16)) (end (line 13) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Item"))) (kind membershipImport) (ordinal 0)) (authored-target "Items::Item") (range (start (line 9) (character 16)) (end (line 9) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Object"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Object") (range (start (line 7) (character 16)) (end (line 7) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part"))) (kind specialization) (ordinal 0)) (authored-target "Item") (range (start (line 18) (character 27)) (end (line 18) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Item")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::done"))) (kind featureTyping) (ordinal 0)) (authored-target "Part") (range (start (line 28) (character 13)) (end (line 28) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::done"))) (kind redefinition) (ordinal 0)) (authored-target "Item::done") (range (start (line 28) (character 22)) (end (line 28) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (kind featureTyping) (ordinal 0)) (authored-target "StateAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::StateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (kind subsetting) (ordinal 0)) (authored-target "stateActions") (range (start (line 58) (character 59)) (end (line 58) (character 71))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::stateActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (kind subsetting) (ordinal 1)) (authored-target "performedActions") (range (start (line 58) (character 73)) (end (line 58) (character 89))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Part::performedActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (range (start (line 44) (character 48)) (end (line 44) (character 55))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (kind subsetting) (ordinal 1)) (authored-target "ownedPerformances") (range (start (line 44) (character 57)) (end (line 44) (character 74))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))) (kind featureTyping) (ordinal 0)) (authored-target "Port") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Port")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))) (kind subsetting) (ordinal 0)) (authored-target "ports") (range (start (line 30) (character 42)) (end (line 30) (character 47))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::ports")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))) (kind subsetting) (ordinal 1)) (authored-target "timeEnclosedOccurrences") (range (start (line 30) (character 49)) (end (line 30) (character 72))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (kind featureTyping) (ordinal 0)) (authored-target "StateAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::StateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (kind subsetting) (ordinal 0)) (authored-target "stateActions") (range (start (line 65) (character 51)) (end (line 65) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::stateActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (kind subsetting) (ordinal 1)) (authored-target "ownedActions") (range (start (line 65) (character 65)) (end (line 65) (character 77))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Part::ownedActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (range (start (line 37) (character 56)) (end (line 37) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (kind subsetting) (ordinal 1)) (authored-target "enactedPerformances") (range (start (line 37) (character 65)) (end (line 37) (character 84))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::start"))) (kind featureTyping) (ordinal 0)) (authored-target "Part") (range (start (line 27) (character 14)) (end (line 27) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::start"))) (kind redefinition) (ordinal 0)) (authored-target "Item::start") (range (start (line 27) (character 23)) (end (line 27) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Port"))) (kind membershipImport) (ordinal 0)) (authored-target "Ports::Port") (range (start (line 11) (character 16)) (end (line 11) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::StateAction"))) (kind membershipImport) (ordinal 0)) (authored-target "States::StateAction") (range (start (line 15) (character 16)) (end (line 15) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::actions"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::actions") (range (start (line 14) (character 16)) (end (line 14) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::items"))) (kind membershipImport) (ordinal 0)) (authored-target "Items::items") (range (start (line 10) (character 16)) (end (line 10) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::objects"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::objects") (range (start (line 8) (character 16)) (end (line 8) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::parts"))) (kind featureTyping) (ordinal 0)) (authored-target "Part") (range (start (line 73) (character 22)) (end (line 73) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::parts"))) (kind subsetting) (ordinal 0)) (authored-target "items") (range (start (line 73) (character 46)) (end (line 73) (character 51))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::items")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::ports"))) (kind membershipImport) (ordinal 0)) (authored-target "Ports::ports") (range (start (line 12) (character 16)) (end (line 12) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::stateActions"))) (kind membershipImport) (ordinal 0)) (authored-target "States::stateActions") (range (start (line 16) (character 16)) (end (line 16) (character 36))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Parts::Part"))) (target (node (document "d0") (qualified-name "Parts::Item"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::Part"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts::Part::done"))) (target (node (document "d0") (qualified-name "Parts::Part"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::Part::done"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (target (node (document "d0") (qualified-name "Parts::StateAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (target (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (kind subsetting) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (target (node (document "d0") (qualified-name "Parts::stateActions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (target (node (document "d0") (qualified-name "Parts::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (target (node (document "d0") (qualified-name "Parts::actions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))) (target (node (document "d0") (qualified-name "Parts::Port"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))) (target (node (document "d0") (qualified-name "Parts::ports"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (target (node (document "d0") (qualified-name "Parts::StateAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (target (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (kind subsetting) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (target (node (document "d0") (qualified-name "Parts::stateActions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (target (node (document "d0") (qualified-name "Parts::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (target (node (document "d0") (qualified-name "Parts::actions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts::Part::start"))) (target (node (document "d0") (qualified-name "Parts::Part"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::Part::start"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts::parts"))) (target (node (document "d0") (qualified-name "Parts::Part"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::parts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Parts::parts"))) (target (node (document "d0") (qualified-name "Parts::items"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts::parts"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
