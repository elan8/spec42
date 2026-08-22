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
  (document "memory://snapshot/parts.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 7 16) (end 7 31))
      )
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
        (range (start 18 27) (end 18 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 21) (end 25 31))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 28) (end 30 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 30 42) (end 30 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 30 49) (end 30 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 40) (end 37 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 56) (end 37 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 65) (end 37 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 44 32) (end 44 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 44 48) (end 44 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 44 57) (end 44 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 50 29) (end 50 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 50 43) (end 50 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 58 38) (end 58 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 59) (end 58 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 30) (end 65 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 65 51) (end 65 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 73 46) (end 73 51))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:23f4184b21d02572131ed39a5454f7fdf3a3fb2b76940077e2469fcaaa795f82") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n * This package defines the base types for parts and related structural elements in the\n * SysML language.\n "))))
    (declaration (id (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::Object") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::objects") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Items::Item") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Items::items") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Ports::Port") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Ports::ports") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Actions::Action") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Actions::actions") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "States::StateAction") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "States::stateActions") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * Part is the most general class of objects that represent all or a part of a system.\n\t\t * Part is the base type of all PartDefinitions.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Item")))))
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::done"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Part")) (redefinition (reference "Item::done")))))
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::exhibitedStates"))) (kind state) (membership (kind feature) (visibility default)) (facts (modifiers abstract reference) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * StateActions that are exhibited by this Part.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateAction")) (subsetting (reference "stateActions")) (subsetting (reference "performedActions")))))
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract composite) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * Actions that are owned by this Part.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action")) (subsetting (reference "actions")) (subsetting (reference "ownedPerformances")))))
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions::this"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (documentation (doc (text "\n\t\t\t\t * The \"this\" reference of an ownedAction is always its owning Part.\n\t\t\t\t "))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Part")) (redefinition (reference "Action::this")) (redefinition (reference "ownedPerformances::this")))))
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (kind port) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * Ports that are owned by this Part.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Port")) (subsetting (reference "ports")) (subsetting (reference "timeEnclosedOccurrences")))))
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedStates"))) (kind state) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * StateActions that are owned by this Part.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateAction")) (subsetting (reference "stateActions")) (subsetting (reference "ownedActions")))))
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::performedActions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract reference) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * Actions that are performed by this Part.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action")) (subsetting (reference "actions")) (subsetting (reference "enactedPerformances")))))
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::self"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Part")) (redefinition (reference "Item::self")))))
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::start"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Part")) (redefinition (reference "Item::start")))))
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::parts"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers abstract nonunique) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * parts is the base feature of all part properties.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Part")) (subsetting (reference "items")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::Object")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::objects")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Items::Item")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Items::items")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "Ports::Port")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Ports::ports")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "Actions::Action")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "Actions::actions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "States::StateAction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "States::stateActions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part"))) (kind specialization) (ordinal 0))
      (authored-target "Item")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::done"))) (kind featureTyping) (ordinal 0))
      (authored-target "Part")
      (outcome (status resolved) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::done"))) (kind redefinition) (ordinal 0))
      (authored-target "Item::done")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::exhibitedStates"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateAction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::exhibitedStates"))) (kind subsetting) (ordinal 0))
      (authored-target "stateActions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::exhibitedStates"))) (kind subsetting) (ordinal 1))
      (authored-target "performedActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::performedActions")))))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions"))) (kind featureTyping) (ordinal 0))
      (authored-target "Action")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions"))) (kind subsetting) (ordinal 0))
      (authored-target "actions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions"))) (kind subsetting) (ordinal 1))
      (authored-target "ownedPerformances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions::this"))) (kind featureTyping) (ordinal 0))
      (authored-target "Part")
      (outcome (status resolved) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions::this"))) (kind redefinition) (ordinal 0))
      (authored-target "Action::this")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions::this"))) (kind redefinition) (ordinal 1))
      (authored-target "ownedPerformances::this")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (kind featureTyping) (ordinal 0))
      (authored-target "Port")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (kind subsetting) (ordinal 0))
      (authored-target "ports")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (kind subsetting) (ordinal 1))
      (authored-target "timeEnclosedOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedStates"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateAction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedStates"))) (kind subsetting) (ordinal 0))
      (authored-target "stateActions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedStates"))) (kind subsetting) (ordinal 1))
      (authored-target "ownedActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions")))))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::performedActions"))) (kind featureTyping) (ordinal 0))
      (authored-target "Action")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::performedActions"))) (kind subsetting) (ordinal 0))
      (authored-target "actions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::performedActions"))) (kind subsetting) (ordinal 1))
      (authored-target "enactedPerformances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "Part")
      (outcome (status resolved) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::self"))) (kind redefinition) (ordinal 0))
      (authored-target "Item::self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::start"))) (kind featureTyping) (ordinal 0))
      (authored-target "Part")
      (outcome (status resolved) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::start"))) (kind redefinition) (ordinal 0))
      (authored-target "Item::start")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::parts"))) (kind featureTyping) (ordinal 0))
      (authored-target "Part")
      (outcome (status resolved) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::parts"))) (kind subsetting) (ordinal 0))
      (authored-target "items")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::done"))) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::done"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::exhibitedStates"))) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::performedActions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::exhibitedStates"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions::this"))) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions::this"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedStates"))) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedStates"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::self"))) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::start"))) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::start"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::parts"))) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::parts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::done"))) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::exhibitedStates"))) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions"))) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions::this"))) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedStates"))) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::performedActions"))) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::self"))) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::start"))) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))
      (subtype (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::done")) (scopes any))
      (subtype (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions::this")) (scopes any))
      (subtype (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::self")) (scopes any))
      (subtype (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::start")) (scopes any))
      (subtype (node (document "memory://snapshot/parts.md") (qualified-name "Parts::parts")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::done")))
      (featured-by (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))
      (type (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")) (source direct))
      (supertype (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::exhibitedStates")))
      (featured-by (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))
      (supertype (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::performedActions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions")))
      (featured-by (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))
      (subtype (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedStates")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions::this")))
      (featured-by (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions")))
      (type (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")) (source direct))
      (supertype (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedPorts")))
      (featured-by (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))
    )
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedStates")))
      (featured-by (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))
      (supertype (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::performedActions")))
      (featured-by (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))
      (subtype (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::exhibitedStates")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::self")))
      (featured-by (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))
      (type (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")) (source direct))
      (supertype (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::start")))
      (featured-by (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))
      (type (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")) (source direct))
      (supertype (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parts.md") (qualified-name "Parts::parts")))
      (type (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")) (source direct))
      (supertype (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/parts.md") (range (start 7 16) (end 7 31)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::Object")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 8 16) (end 8 32)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::objects")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 9 16) (end 9 27)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Items::Item")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 10 16) (end 10 28)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Items::items")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 11 16) (end 11 27)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "Ports::Port")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 12 16) (end 12 28)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Ports::ports")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 13 16) (end 13 31)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "Actions::Action")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 14 16) (end 14 32)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "Actions::actions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 15 16) (end 15 35)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "States::StateAction")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 16 16) (end 16 36)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/parts.md") (path (named (kind library-package) (name "Parts")) (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "States::stateActions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 18 27) (end 18 31)) (probe (position 18 27))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part"))) (kind specialization) (ordinal 0) (authored-target "Item")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 28 13) (end 28 17)) (probe (position 28 13))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::done"))) (kind featureTyping) (ordinal 0) (authored-target "Part")
      (outcome (status resolved) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 28 22) (end 28 32)) (probe (position 28 22))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::done"))) (kind redefinition) (ordinal 0) (authored-target "Item::done")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 58 38) (end 58 49)) (probe (position 58 38))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::exhibitedStates"))) (kind featureTyping) (ordinal 0) (authored-target "StateAction")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 58 59) (end 58 71)) (probe (position 58 59))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::exhibitedStates"))) (kind subsetting) (ordinal 0) (authored-target "stateActions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 58 73) (end 58 89)) (probe (position 58 73))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::exhibitedStates"))) (kind subsetting) (ordinal 1) (authored-target "performedActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::performedActions")))))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 44 32) (end 44 38)) (probe (position 44 32))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions"))) (kind featureTyping) (ordinal 0) (authored-target "Action")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 44 48) (end 44 55)) (probe (position 44 48))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 44 57) (end 44 74)) (probe (position 44 57))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions"))) (kind subsetting) (ordinal 1) (authored-target "ownedPerformances")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 50 20) (end 50 24)) (probe (position 50 20))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions::this"))) (kind featureTyping) (ordinal 0) (authored-target "Part")
      (outcome (status resolved) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 50 29) (end 50 41)) (probe (position 50 29))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions::this"))) (kind redefinition) (ordinal 0) (authored-target "Action::this")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 50 43) (end 50 66)) (probe (position 50 43))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions::this"))) (kind redefinition) (ordinal 1) (authored-target "ownedPerformances::this")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 30 28) (end 30 32)) (probe (position 30 28))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (kind featureTyping) (ordinal 0) (authored-target "Port")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 30 42) (end 30 47)) (probe (position 30 42))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (kind subsetting) (ordinal 0) (authored-target "ports")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 30 49) (end 30 72)) (probe (position 30 49))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (kind subsetting) (ordinal 1) (authored-target "timeEnclosedOccurrences")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 65 30) (end 65 41)) (probe (position 65 30))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedStates"))) (kind featureTyping) (ordinal 0) (authored-target "StateAction")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 65 51) (end 65 63)) (probe (position 65 51))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedStates"))) (kind subsetting) (ordinal 0) (authored-target "stateActions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 65 65) (end 65 77)) (probe (position 65 65))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedStates"))) (kind subsetting) (ordinal 1) (authored-target "ownedActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::ownedActions")))))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 37 40) (end 37 46)) (probe (position 37 40))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::performedActions"))) (kind featureTyping) (ordinal 0) (authored-target "Action")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 37 56) (end 37 63)) (probe (position 37 56))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::performedActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 37 65) (end 37 84)) (probe (position 37 65))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::performedActions"))) (kind subsetting) (ordinal 1) (authored-target "enactedPerformances")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 25 12) (end 25 16)) (probe (position 25 12))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::self"))) (kind featureTyping) (ordinal 0) (authored-target "Part")
      (outcome (status resolved) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 25 21) (end 25 31)) (probe (position 25 21))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::self"))) (kind redefinition) (ordinal 0) (authored-target "Item::self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 27 14) (end 27 18)) (probe (position 27 14))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::start"))) (kind featureTyping) (ordinal 0) (authored-target "Part")
      (outcome (status resolved) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 27 23) (end 27 34)) (probe (position 27 23))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part::start"))) (kind redefinition) (ordinal 0) (authored-target "Item::start")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 73 22) (end 73 26)) (probe (position 73 22))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::parts"))) (kind featureTyping) (ordinal 0) (authored-target "Part")
      (outcome (status resolved) (target (node (document "memory://snapshot/parts.md") (qualified-name "Parts::Part")))))
    )
  )
  (query (document "memory://snapshot/parts.md") (range (start 73 46) (end 73 51)) (probe (position 73 46))
    (reference (id (source (node (document "memory://snapshot/parts.md") (qualified-name "Parts::parts"))) (kind subsetting) (ordinal 0) (authored-target "items")
      (outcome (status unresolved)))
    )
  )
)
~~~
