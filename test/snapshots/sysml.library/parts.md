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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b1a2e6bbe50cef288c0718fbb55ad4407154fe15db567140d5ffab5b132e7ea4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Parts"))) (kind "package") (name "Parts") (declared-name "Parts"))
    (element (id (node (document "d0") (qualified-name "Parts::Action"))) (kind "import") (name "Action") (declared-name "Action") (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::Action") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Parts::Item"))) (kind "import") (name "Item") (declared-name "Item") (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "Items::Item") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Parts::Object"))) (kind "import") (name "Object") (declared-name "Object") (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Object") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Parts::Part"))) (kind "part def") (name "Part") (declared-name "Part") (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Item")))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Parts::Part"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::done"))) (kind "part") (name "done") (declared-name "done") (parent (node (document "d0") (qualified-name "Parts::Part"))) (authored (membership (kind Feature)) (relationships (typing (reference "Part")) (redefinition (reference "Item::done")))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (kind "state") (name "exhibitedStates") (declared-name "exhibitedStates") (parent (node (document "d0") (qualified-name "Parts::Part"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateAction")) (subsetting (reference "stateActions")) (subsetting (reference "performedActions")))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::exhibitedStates::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (kind "action") (name "ownedActions") (declared-name "ownedActions") (parent (node (document "d0") (qualified-name "Parts::Part"))) (authored (membership (kind Feature)) (relationships (typing (reference "Action")) (subsetting (reference "actions")) (subsetting (reference "ownedPerformances")))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::ownedActions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Parts::Part::ownedActions"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::ownedActions::part"))) (kind "ref") (name "part") (declared-name "part") (parent (node (document "d0") (qualified-name "Parts::Part::ownedActions"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::ownedActions::part::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Parts::Part::ownedActions::part"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))) (kind "port") (name "ownedPorts") (declared-name "ownedPorts") (parent (node (document "d0") (qualified-name "Parts::Part"))) (authored (membership (kind Feature)) (relationships (typing (reference "Port")) (subsetting (reference "ports")) (subsetting (reference "timeEnclosedOccurrences")))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::ownedPorts::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (kind "state") (name "ownedStates") (declared-name "ownedStates") (parent (node (document "d0") (qualified-name "Parts::Part"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateAction")) (subsetting (reference "stateActions")) (subsetting (reference "ownedActions")))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::ownedStates::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Parts::Part::ownedStates"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (kind "action") (name "performedActions") (declared-name "performedActions") (parent (node (document "d0") (qualified-name "Parts::Part"))) (authored (membership (kind Feature)) (relationships (typing (reference "Action")) (subsetting (reference "actions")) (subsetting (reference "enactedPerformances")))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::performedActions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Parts::Part::performedActions"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::self"))) (kind "opaque member") (name "self") (declared-name "self") (parent (node (document "d0") (qualified-name "Parts::Part"))))
    (element (id (node (document "d0") (qualified-name "Parts::Part::start"))) (kind "part") (name "start") (declared-name "start") (parent (node (document "d0") (qualified-name "Parts::Part"))) (authored (membership (kind Feature)) (relationships (typing (reference "Part")) (redefinition (reference "Item::start")))))
    (element (id (node (document "d0") (qualified-name "Parts::Port"))) (kind "import") (name "Port") (declared-name "Port") (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "Ports::Port") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Parts::StateAction"))) (kind "import") (name "StateAction") (declared-name "StateAction") (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "States::StateAction") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Parts::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Parts"))))
    (element (id (node (document "d0") (qualified-name "Parts::actions"))) (kind "import") (name "actions") (declared-name "actions") (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::actions") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Parts::items"))) (kind "import") (name "items") (declared-name "items") (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "Items::items") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Parts::objects"))) (kind "import") (name "objects") (declared-name "objects") (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::objects") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Parts::parts"))) (kind "part") (name "parts") (declared-name "parts") (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Feature)) (relationships (typing (reference "Part")) (subsetting (reference "items")))))
    (element (id (node (document "d0") (qualified-name "Parts::parts::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Parts::parts"))))
    (element (id (node (document "d0") (qualified-name "Parts::ports"))) (kind "import") (name "ports") (declared-name "ports") (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "Ports::ports") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Parts::stateActions"))) (kind "import") (name "stateActions") (declared-name "stateActions") (parent (node (document "d0") (qualified-name "Parts"))) (authored (membership (kind Import) (visibility "private") (import (reference "States::stateActions") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Parts::Action"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::Action") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Item"))) (kind membershipImport) (ordinal 0)) (authored-target "Items::Item") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Object"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Object") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part"))) (kind specialization) (ordinal 0)) (authored-target "Item") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Item")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::done"))) (kind featureTyping) (ordinal 0)) (authored-target "Part") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::done"))) (kind redefinition) (ordinal 0)) (authored-target "Item::done") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (kind featureTyping) (ordinal 0)) (authored-target "StateAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::StateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (kind subsetting) (ordinal 0)) (authored-target "stateActions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::stateActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::exhibitedStates"))) (kind subsetting) (ordinal 1)) (authored-target "performedActions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Part::performedActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedActions"))) (kind subsetting) (ordinal 1)) (authored-target "ownedPerformances") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))) (kind featureTyping) (ordinal 0)) (authored-target "Port") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Port")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))) (kind subsetting) (ordinal 0)) (authored-target "ports") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::ports")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedPorts"))) (kind subsetting) (ordinal 1)) (authored-target "timeEnclosedOccurrences") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (kind featureTyping) (ordinal 0)) (authored-target "StateAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::StateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (kind subsetting) (ordinal 0)) (authored-target "stateActions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::stateActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::ownedStates"))) (kind subsetting) (ordinal 1)) (authored-target "ownedActions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Part::ownedActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::performedActions"))) (kind subsetting) (ordinal 1)) (authored-target "enactedPerformances") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::start"))) (kind featureTyping) (ordinal 0)) (authored-target "Part") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Part::start"))) (kind redefinition) (ordinal 0)) (authored-target "Item::start") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::Port"))) (kind membershipImport) (ordinal 0)) (authored-target "Ports::Port") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::StateAction"))) (kind membershipImport) (ordinal 0)) (authored-target "States::StateAction") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::actions"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::actions") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::items"))) (kind membershipImport) (ordinal 0)) (authored-target "Items::items") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::objects"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::objects") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::parts"))) (kind featureTyping) (ordinal 0)) (authored-target "Part") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::parts"))) (kind subsetting) (ordinal 0)) (authored-target "items") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts::items")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts::ports"))) (kind membershipImport) (ordinal 0)) (authored-target "Ports::ports") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Parts::stateActions"))) (kind membershipImport) (ordinal 0)) (authored-target "States::stateActions") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 18 27) (end 18 31)) (probe (position 18 27))
      (reference
        (source (document "d0") (qualified-name "Parts::Part"))
        (kind specialization) (ordinal 0) (authored-target "Item")
        (range (start 18 27) (end 18 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts::Item") (range (start 9 1) (end 9 28)))
        )
      )
    )
    (query (range (start 27 14) (end 27 18)) (probe (position 27 14))
      (reference
        (source (document "d0") (qualified-name "Parts::Part::start"))
        (kind featureTyping) (ordinal 0) (authored-target "Part")
        (range (start 27 14) (end 27 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts::Part") (range (start 18 1) (end 18 1266)))
        )
      )
    )
    (query (range (start 28 13) (end 28 17)) (probe (position 28 13))
      (reference
        (source (document "d0") (qualified-name "Parts::Part::done"))
        (kind featureTyping) (ordinal 0) (authored-target "Part")
        (range (start 28 13) (end 28 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts::Part") (range (start 18 1) (end 18 1266)))
        )
      )
    )
    (query (range (start 73 22) (end 73 26)) (probe (position 73 22))
      (reference
        (source (document "d0") (qualified-name "Parts::parts"))
        (kind featureTyping) (ordinal 0) (authored-target "Part")
        (range (start 73 22) (end 73 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts::Part") (range (start 18 1) (end 18 1266)))
        )
      )
    )
    (query (range (start 30 42) (end 30 47)) (probe (position 30 42))
      (reference
        (source (document "d0") (qualified-name "Parts::Part::ownedPorts"))
        (kind subsetting) (ordinal 0) (authored-target "ports")
        (range (start 30 42) (end 30 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts::ports") (range (start 12 1) (end 12 29)))
        )
      )
    )
    (query (range (start 73 46) (end 73 51)) (probe (position 73 46))
      (reference
        (source (document "d0") (qualified-name "Parts::parts"))
        (kind subsetting) (ordinal 0) (authored-target "items")
        (range (start 73 46) (end 73 51))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts::items") (range (start 10 1) (end 10 29)))
        )
      )
    )
    (query (range (start 37 56) (end 37 63)) (probe (position 37 56))
      (reference
        (source (document "d0") (qualified-name "Parts::Part::performedActions"))
        (kind subsetting) (ordinal 0) (authored-target "actions")
        (range (start 37 56) (end 37 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts::actions") (range (start 14 1) (end 14 33)))
        )
      )
    )
    (query (range (start 44 48) (end 44 55)) (probe (position 44 48))
      (reference
        (source (document "d0") (qualified-name "Parts::Part::ownedActions"))
        (kind subsetting) (ordinal 0) (authored-target "actions")
        (range (start 44 48) (end 44 55))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts::actions") (range (start 14 1) (end 14 33)))
        )
      )
    )
    (query (range (start 28 22) (end 28 32)) (probe (position 28 22))
      (reference
        (source (document "d0") (qualified-name "Parts::Part::done"))
        (kind redefinition) (ordinal 0) (authored-target "Item::done")
        (range (start 28 22) (end 28 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 27)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Parts::Item"))
        (kind membershipImport) (ordinal 0) (authored-target "Items::Item")
        (range (start 9 16) (end 9 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 27)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "Parts::Port"))
        (kind membershipImport) (ordinal 0) (authored-target "Ports::Port")
        (range (start 11 16) (end 11 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 27 23) (end 27 34)) (probe (position 27 23))
      (reference
        (source (document "d0") (qualified-name "Parts::Part::start"))
        (kind redefinition) (ordinal 0) (authored-target "Item::start")
        (range (start 27 23) (end 27 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 28)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "Parts::items"))
        (kind membershipImport) (ordinal 0) (authored-target "Items::items")
        (range (start 10 16) (end 10 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 28)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "Parts::ports"))
        (kind membershipImport) (ordinal 0) (authored-target "Ports::ports")
        (range (start 12 16) (end 12 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 58 59) (end 58 71)) (probe (position 58 59))
      (reference
        (source (document "d0") (qualified-name "Parts::Part::exhibitedStates"))
        (kind subsetting) (ordinal 0) (authored-target "stateActions")
        (range (start 58 59) (end 58 71))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts::stateActions") (range (start 16 1) (end 16 37)))
        )
      )
    )
    (query (range (start 65 51) (end 65 63)) (probe (position 65 51))
      (reference
        (source (document "d0") (qualified-name "Parts::Part::ownedStates"))
        (kind subsetting) (ordinal 0) (authored-target "stateActions")
        (range (start 65 51) (end 65 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts::stateActions") (range (start 16 1) (end 16 37)))
        )
      )
    )
    (query (range (start 65 65) (end 65 77)) (probe (position 65 65))
      (reference
        (source (document "d0") (qualified-name "Parts::Part::ownedStates"))
        (kind subsetting) (ordinal 1) (authored-target "ownedActions")
        (range (start 65 65) (end 65 77))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts::Part::ownedActions") (range (start 44 2) (end 44 331)))
        )
      )
    )
    (query (range (start 7 16) (end 7 31)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Parts::Object"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::Object")
        (range (start 7 16) (end 7 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 31)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "Parts::Action"))
        (kind membershipImport) (ordinal 0) (authored-target "Actions::Action")
        (range (start 13 16) (end 13 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 32)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "Parts::objects"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::objects")
        (range (start 8 16) (end 8 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 16) (end 14 32)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "Parts::actions"))
        (kind membershipImport) (ordinal 0) (authored-target "Actions::actions")
        (range (start 14 16) (end 14 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 58 73) (end 58 89)) (probe (position 58 73))
      (reference
        (source (document "d0") (qualified-name "Parts::Part::exhibitedStates"))
        (kind subsetting) (ordinal 1) (authored-target "performedActions")
        (range (start 58 73) (end 58 89))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts::Part::performedActions") (range (start 37 2) (end 37 157)))
        )
      )
    )
    (query (range (start 44 57) (end 44 74)) (probe (position 44 57))
      (reference
        (source (document "d0") (qualified-name "Parts::Part::ownedActions"))
        (kind subsetting) (ordinal 1) (authored-target "ownedPerformances")
        (range (start 44 57) (end 44 74))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 16) (end 15 35)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "Parts::StateAction"))
        (kind membershipImport) (ordinal 0) (authored-target "States::StateAction")
        (range (start 15 16) (end 15 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 37 65) (end 37 84)) (probe (position 37 65))
      (reference
        (source (document "d0") (qualified-name "Parts::Part::performedActions"))
        (kind subsetting) (ordinal 1) (authored-target "enactedPerformances")
        (range (start 37 65) (end 37 84))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 16) (end 16 36)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "Parts::stateActions"))
        (kind membershipImport) (ordinal 0) (authored-target "States::stateActions")
        (range (start 16 16) (end 16 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 30 49) (end 30 72)) (probe (position 30 49))
      (reference
        (source (document "d0") (qualified-name "Parts::Part::ownedPorts"))
        (kind subsetting) (ordinal 1) (authored-target "timeEnclosedOccurrences")
        (range (start 30 49) (end 30 72))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
