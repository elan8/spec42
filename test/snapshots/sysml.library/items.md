# META
~~~ini
description=Standard Library: Systems Library/Items
type=file
~~~
# SOURCE
~~~sysml
standard library package Items {
	doc
	/*
	 * This package defines the base types for items and related structural elements in the
	 * SysML language.
	 */

	private import Objects::Object;
	private import Objects::objects;
	private import Parts::Part;
	private import Parts::parts;
	private import Occurrences::HappensWhile;
	private import Occurrences::JustOutsideOf;
	private import Objects::StructuredSpaceObject;
	private import Constraints::ConstraintCheck;
	private import Constraints::constraintChecks;
	private import CollectionFunctions::contains;
	private import SequenceFunctions::isEmpty;
	private import SequenceFunctions::notEmpty;
	private import SequenceFunctions::includes;
	private import SequenceFunctions::union;
	private import ControlFunctions::forAll;
	
	abstract item def Item :> Object {
		doc
		/*
		 * Item is the most general class of objects that are part of, exist in or flow through a system. 
		 * Item is the base type of all ItemDefinitions.
		 */
	
		ref self: Item :>> Object::self;
		
		item start: Item :>> startShot;
		item done: Item :>> endShot;
		
		item shape : Item :>> spaceBoundary {
		doc
			/*
			 * The shape of an Item is its spatial boundary.
			 */
		}
		
		item envelopingShapes : Item[0..*] {
            doc
			/*
			 * Each enveloping shape is the shape of an Item that spacially overlaps this Item for its
			 * entire lifetime.
			 */
			 
			ref item envelopedItem :>> that;	

			assert constraint { 
                doc
                /* 
                 * Enables two dimensional items to be enveloped by two or three dimensional shapes.
                 */             
			    innerSpaceDimension == 
    				(if envelopedItem.innerSpaceDimension == 3  | envelopedItem.outerSpaceDimension == 3? 2 
    				else envelopedItem.outerSpaceDimension - 1)
			}
			assert constraint { (that as Item).innerSpaceDimension < 3 implies notEmpty(outerSpaceDimension) }

			item envelopingItem [1];

			assert constraint {
				doc
				/* 
				 * This constraint prevents an envelopingShape from being a portion.
				 */
				 
				envelopingItem.shape.spaceTimeCoincidentOccurrences->includes(that) and
				envelopingItem.spaceTimeEnclosedOccurrences->includes(that.that) 
			}
		}
		
		item boundingShapes : StructuredSpaceObject [0..*] :> envelopingShapes {
            doc
			/*
			 * envelopingShapes that are structured space objects with every face or every edge
			 * intersecting this Item.
			 */		
            
			ref item boundingShape: Item :>> self;

			private item :>> faces {
				ref item face :>> self;
				item inter [1];
				assert constraint { contains(inter.intersectionsOf, union(face, boundingShape)) }
			}
			private item :>> edges {
				ref item edge :>> self;
				item inter [1];
				assert constraint { isEmpty(faces) implies
							contains(inter.intersectionsOf, union(edge, boundingShape)) }
			}
		}

		item voids :>> innerSpaceOccurrences [0..*] {
			doc
			/*
			 * Voids are inner space occurrences of this Item.
			 */
		}

		attribute isSolid = isEmpty(voids) {
			doc
			/*
			 * An Item is solid if it has no voids.
			 */
		}
		
		abstract item subitems: Item[0..*] :> items, subobjects {
			doc
			/*
			 * The Items that are composite subitems of this Item.
			 */
			 
			private ref redefines Item::incomingTransferSort, subobjects::incomingTransferSort;
		}
		
		abstract part subparts: Part[0..*] :> subitems, parts {
			doc
			/*
			 * The subitems of this Item that are Parts.
			 */
		}
		
		abstract constraint checkedConstraints: ConstraintCheck[0..*] :> constraintChecks, ownedPerformances {
			doc
			/*
			 * Constraints that have been checked by this Item.
			 */
		}
	}
	
	connection def Touches :> JustOutsideOf, HappensWhile {
		doc
		/*
		 * Touching items are just outside each other and happen at the same time.
		 */
	
		end touchesToo [0..*] item touchedItemToo :>> separateSpaceToo, thisOccurrence;
		end touches [0..*] item touchedItem :>> separateSpace, thatOccurrence;
	}

	abstract item items : Item[0..*] nonunique :> objects {
		doc
		/*
		 * items is the base feature of all ItemUsages.
		 */
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "items.md"
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
        (range (start 11 16) (end 11 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 16) (end 17 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 16) (end 18 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 20 16) (end 20 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 16) (end 21 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 30 21) (end 30 33))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 104 2) (end 104 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 120 40) (end 120 48))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f3876a811282b4078b6c7ee27b750afe2a8383430fe1a1f3e68a2127e1019c7f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Items"))) (kind "package") (name "Items") (declared-name "Items"))
    (element (id (node (document "d0") (qualified-name "Items::ConstraintCheck"))) (kind "import") (name "ConstraintCheck") (declared-name "ConstraintCheck") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Constraints::ConstraintCheck") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Items::HappensWhile"))) (kind "import") (name "HappensWhile") (declared-name "HappensWhile") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensWhile") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Items::Item"))) (kind "item def") (name "Item") (declared-name "Item") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Object")))))
    (element (id (node (document "d0") (qualified-name "Items::Item::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Items::Item"))))
    (element (id (node (document "d0") (qualified-name "Items::Item::isSolid"))) (kind "attribute") (name "isSolid") (declared-name "isSolid") (parent (node (document "d0") (qualified-name "Items::Item"))))
    (element (id (node (document "d0") (qualified-name "Items::Item::isSolid::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Items::Item::isSolid"))))
    (element (id (node (document "d0") (qualified-name "Items::Item::self"))) (kind "ref") (name "self") (declared-name "self") (parent (node (document "d0") (qualified-name "Items::Item"))) (authored (membership (kind Feature)) (relationships (typing (reference "Item")) (redefinition (reference "Object::self")))))
    (element (id (node (document "d0") (qualified-name "Items::Item::subparts"))) (kind "part") (name "subparts") (declared-name "subparts") (parent (node (document "d0") (qualified-name "Items::Item"))) (authored (membership (kind Feature)) (relationships (typing (reference "Part")) (subsetting (reference "subitems")) (subsetting (reference "parts")))))
    (element (id (node (document "d0") (qualified-name "Items::Item::subparts::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Items::Item::subparts"))))
    (element (id (node (document "d0") (qualified-name "Items::JustOutsideOf"))) (kind "import") (name "JustOutsideOf") (declared-name "JustOutsideOf") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::JustOutsideOf") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Items::Object"))) (kind "import") (name "Object") (declared-name "Object") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Object") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Items::Part"))) (kind "import") (name "Part") (declared-name "Part") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::Part") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Items::StructuredSpaceObject"))) (kind "import") (name "StructuredSpaceObject") (declared-name "StructuredSpaceObject") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::StructuredSpaceObject") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Items::Touches"))) (kind "connection def") (name "Touches") (declared-name "Touches") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Owning)) (relationships (specializes (reference "JustOutsideOf")) (specializes (reference "HappensWhile")))))
    (element (id (node (document "d0") (qualified-name "Items::Touches::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Items::Touches"))))
    (element (id (node (document "d0") (qualified-name "Items::Touches::touches"))) (kind "interface end") (name "touches") (declared-name "touches") (parent (node (document "d0") (qualified-name "Items::Touches"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Items::Touches::touchesToo"))) (kind "interface end") (name "touchesToo") (declared-name "touchesToo") (parent (node (document "d0") (qualified-name "Items::Touches"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Items::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Items"))))
    (element (id (node (document "d0") (qualified-name "Items::constraintChecks"))) (kind "import") (name "constraintChecks") (declared-name "constraintChecks") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Constraints::constraintChecks") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Items::contains"))) (kind "import") (name "contains") (declared-name "contains") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "CollectionFunctions::contains") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Items::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Items::includes"))) (kind "import") (name "includes") (declared-name "includes") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::includes") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Items::isEmpty"))) (kind "import") (name "isEmpty") (declared-name "isEmpty") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::isEmpty") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Items::items"))) (kind "item def") (name "items") (declared-name "items") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Owning)) (relationships (specializes (reference "objects")))))
    (element (id (node (document "d0") (qualified-name "Items::items::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Items::items"))))
    (element (id (node (document "d0") (qualified-name "Items::notEmpty"))) (kind "import") (name "notEmpty") (declared-name "notEmpty") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::notEmpty") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Items::objects"))) (kind "import") (name "objects") (declared-name "objects") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::objects") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Items::parts"))) (kind "import") (name "parts") (declared-name "parts") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::parts") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Items::union"))) (kind "import") (name "union") (declared-name "union") (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::union") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Items::ConstraintCheck"))) (kind membershipImport) (ordinal 0)) (authored-target "Constraints::ConstraintCheck") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::HappensWhile"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensWhile") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::Item"))) (kind specialization) (ordinal 0)) (authored-target "Object") (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::Object")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::Item::self"))) (kind featureTyping) (ordinal 0)) (authored-target "Item") (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::Item")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::Item::self"))) (kind redefinition) (ordinal 0)) (authored-target "Object::self") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::Item::subparts"))) (kind featureTyping) (ordinal 0)) (authored-target "Part") (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::Item::subparts"))) (kind subsetting) (ordinal 0)) (authored-target "subitems") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::Item::subparts"))) (kind subsetting) (ordinal 1)) (authored-target "parts") (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::parts")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::JustOutsideOf"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::JustOutsideOf") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::Object"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Object") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::Part"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::Part") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::StructuredSpaceObject"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::StructuredSpaceObject") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::Touches"))) (kind specialization) (ordinal 0)) (authored-target "JustOutsideOf") (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::JustOutsideOf")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::Touches"))) (kind specialization) (ordinal 1)) (authored-target "HappensWhile") (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::HappensWhile")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::Touches::touches"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::Touches::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::Touches::touchesToo"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::Touches::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::constraintChecks"))) (kind membershipImport) (ordinal 0)) (authored-target "Constraints::constraintChecks") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::contains"))) (kind membershipImport) (ordinal 0)) (authored-target "CollectionFunctions::contains") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::includes"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::includes") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::isEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::isEmpty") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::items"))) (kind specialization) (ordinal 0)) (authored-target "objects") (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::objects")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::notEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::notEmpty") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::objects"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::objects") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::parts"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::parts") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::union"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::union") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Items::Item"))) (target (node (document "d0") (qualified-name "Items::Object"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Items::Item"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Items::Item::self"))) (target (node (document "d0") (qualified-name "Items::Item"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Items::Item::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Items::Item::subparts"))) (target (node (document "d0") (qualified-name "Items::Part"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Items::Item::subparts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Items::Item::subparts"))) (target (node (document "d0") (qualified-name "Items::parts"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Items::Item::subparts"))) (kind subsetting) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Items::Touches"))) (target (node (document "d0") (qualified-name "Items::HappensWhile"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Items::Touches"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Items::Touches"))) (target (node (document "d0") (qualified-name "Items::JustOutsideOf"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Items::Touches"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Items::Touches::touches"))) (target (node (document "d0") (qualified-name "Items::Touches::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Items::Touches::touches"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Items::Touches::touchesToo"))) (target (node (document "d0") (qualified-name "Items::Touches::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Items::Touches::touchesToo"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Items::items"))) (target (node (document "d0") (qualified-name "Items::objects"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Items::items"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 120 26) (end 120 30)) (probe (position 120 26))
      (reference
        (source (document "d0") (qualified-name "Items::Item::subparts"))
        (kind featureTyping) (ordinal 0) (authored-target "Part")
        (range (start 120 26) (end 120 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Items::Part") (range (start 9 1) (end 9 28)))
        )
      )
    )
    (query (range (start 30 11) (end 30 16)) (probe (position 30 11))
      (reference
        (source (document "d0") (qualified-name "Items::Item::self"))
        (kind featureTyping) (ordinal 0) (authored-target "Item")
        (range (start 30 11) (end 30 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Items::Item") (range (start 23 1) (end 23 2898)))
        )
      )
    )
    (query (range (start 120 50) (end 120 55)) (probe (position 120 50))
      (reference
        (source (document "d0") (qualified-name "Items::Item::subparts"))
        (kind subsetting) (ordinal 1) (authored-target "parts")
        (range (start 120 50) (end 120 55))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Items::parts") (range (start 10 1) (end 10 29)))
        )
      )
    )
    (query (range (start 23 27) (end 23 33)) (probe (position 23 27))
      (reference
        (source (document "d0") (qualified-name "Items::Item"))
        (kind specialization) (ordinal 0) (authored-target "Object")
        (range (start 23 27) (end 23 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Items::Object") (range (start 7 1) (end 7 32)))
        )
      )
    )
    (query (range (start 0 0) (end 0 7)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "Items::items"))
        (kind specialization) (ordinal 0) (authored-target "objects")
        (range (start 0 0) (end 0 7))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Items::objects") (range (start 8 1) (end 8 33)))
        )
      )
    )
    (query (range (start 120 40) (end 120 48)) (probe (position 120 40))
      (reference
        (source (document "d0") (qualified-name "Items::Item::subparts"))
        (kind subsetting) (ordinal 0) (authored-target "subitems")
        (range (start 120 40) (end 120 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 27)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Items::Part"))
        (kind membershipImport) (ordinal 0) (authored-target "Parts::Part")
        (range (start 9 16) (end 9 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 28)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "Items::parts"))
        (kind membershipImport) (ordinal 0) (authored-target "Parts::parts")
        (range (start 10 16) (end 10 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 30 21) (end 30 33)) (probe (position 30 21))
      (reference
        (source (document "d0") (qualified-name "Items::Item::self"))
        (kind redefinition) (ordinal 0) (authored-target "Object::self")
        (range (start 30 21) (end 30 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 135 42) (end 135 54)) (probe (position 135 42))
      (reference
        (source (document "d0") (qualified-name "Items::Touches"))
        (kind specialization) (ordinal 1) (authored-target "HappensWhile")
        (range (start 135 42) (end 135 54))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Items::HappensWhile") (range (start 11 1) (end 11 42)))
        )
      )
    )
    (query (range (start 135 27) (end 135 40)) (probe (position 135 27))
      (reference
        (source (document "d0") (qualified-name "Items::Touches"))
        (kind specialization) (ordinal 0) (authored-target "JustOutsideOf")
        (range (start 135 27) (end 135 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Items::JustOutsideOf") (range (start 12 1) (end 12 43)))
        )
      )
    )
    (query (range (start 7 16) (end 7 31)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Items::Object"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::Object")
        (range (start 7 16) (end 7 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 32)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "Items::objects"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::objects")
        (range (start 8 16) (end 8 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 16) (end 20 40)) (probe (position 20 16))
      (reference
        (source (document "d0") (qualified-name "Items::union"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::union")
        (range (start 20 16) (end 20 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 16) (end 21 40)) (probe (position 21 16))
      (reference
        (source (document "d0") (qualified-name "Items::forAll"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
        (range (start 21 16) (end 21 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 41)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "Items::HappensWhile"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensWhile")
        (range (start 11 16) (end 11 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 42)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "Items::JustOutsideOf"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::JustOutsideOf")
        (range (start 12 16) (end 12 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 16) (end 17 42)) (probe (position 17 16))
      (reference
        (source (document "d0") (qualified-name "Items::isEmpty"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::isEmpty")
        (range (start 17 16) (end 17 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 16) (end 18 43)) (probe (position 18 16))
      (reference
        (source (document "d0") (qualified-name "Items::notEmpty"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::notEmpty")
        (range (start 18 16) (end 18 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 16) (end 19 43)) (probe (position 19 16))
      (reference
        (source (document "d0") (qualified-name "Items::includes"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::includes")
        (range (start 19 16) (end 19 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 16) (end 14 44)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "Items::ConstraintCheck"))
        (kind membershipImport) (ordinal 0) (authored-target "Constraints::ConstraintCheck")
        (range (start 14 16) (end 14 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 16) (end 15 45)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "Items::constraintChecks"))
        (kind membershipImport) (ordinal 0) (authored-target "Constraints::constraintChecks")
        (range (start 15 16) (end 15 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 16) (end 16 45)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "Items::contains"))
        (kind membershipImport) (ordinal 0) (authored-target "CollectionFunctions::contains")
        (range (start 16 16) (end 16 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 46)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "Items::StructuredSpaceObject"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::StructuredSpaceObject")
        (range (start 13 16) (end 13 46))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
