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
  (document "memory://snapshot/items.md"
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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 23 27) (end 23 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 30 21) (end 30 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 23) (end 32 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 33 22) (end 33 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 35 24) (end 35 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 49 30) (end 49 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 56 7) (end 56 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 57 9) (end 58 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 60 23) (end 60 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 60 70) (end 60 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 60 79) (end 60 98))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 70 4) (end 70 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 71 4) (end 71 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 75 24) (end 75 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 84 20) (end 84 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 24) (end 87 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 33) (end 87 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 56) (end 87 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 89 20) (end 89 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 92 24) (end 92 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 92 32) (end 92 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 93 7) (end 93 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 93 16) (end 93 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 93 39) (end 93 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 97 17) (end 97 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 104 22) (end 104 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 111 2) (end 118 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 120 26) (end 120 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 120 40) (end 120 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 120 50) (end 120 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 127 2) (end 132 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 135 27) (end 135 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 135 42) (end 135 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 145 1) (end 150 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 145 1) (end 150 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:daa13b93f12aed86a5f80d15202cdbac7a5a02465d59c2d47eb22cd2e236a815") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::Object") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::objects") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Parts::Part") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Parts::parts") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensWhile") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::JustOutsideOf") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::StructuredSpaceObject") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Constraints::ConstraintCheck") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Constraints::constraintChecks") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "CollectionFunctions::contains") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::isEmpty") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 11))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::notEmpty") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 12))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::includes") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 13))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::union") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 14))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::forAll") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Object"))))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StructuredSpaceObject"))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind item) (ordinal 0))))) (kind item) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (redefinition (reference "faces"))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind item) (ordinal 1))))) (kind item) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (redefinition (reference "edges"))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "face")) (expressionOperand (reference "boundingShape")) (memberAccessOperand (reference "inter::intersectionsOf")) (invocationCallee (reference "contains")) (invocationCallee (reference "union"))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "faces")) (expressionOperand (reference "edge")) (expressionOperand (reference "boundingShape")) (memberAccessOperand (reference "inter::intersectionsOf")) (invocationCallee (reference "isEmpty")) (invocationCallee (reference "contains")) (invocationCallee (reference "union"))))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::edge"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "self"))))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::face"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "self"))))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::inter"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::inter"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::boundingShape"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Item")) (redefinition (reference "self"))))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::done"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Item")) (redefinition (reference "endShot"))))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::envelopingShapes"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Item"))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "innerSpaceDimension"))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 1))))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "outerSpaceDimension")) (invocationCallee (reference "notEmpty"))))
    (declaration (id (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 2))))) (kind constraint) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::envelopingShapes::envelopedItem"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "that"))))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::envelopingShapes::envelopingItem"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::isSolid"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "voids")) (invocationCallee (reference "isEmpty"))))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::self"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Item")) (redefinition (reference "Object::self"))))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::shape"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Item")) (redefinition (reference "spaceBoundary"))))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::start"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Item")) (redefinition (reference "startShot"))))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::subparts"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Part")) (subsetting (reference "subitems")) (subsetting (reference "parts"))))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::voids"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "innerSpaceOccurrences"))))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Touches"))) (kind connection-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "JustOutsideOf")) (specialization (reference "HappensWhile"))))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Touches::touches"))) (kind connection) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/items.md") (qualified-name "Items::Touches::touchesToo"))) (kind connection) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::Object")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::objects")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Parts::Part")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Parts::parts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensWhile")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::JustOutsideOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::StructuredSpaceObject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "Constraints::ConstraintCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "Constraints::constraintChecks")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "CollectionFunctions::contains")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::notEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::includes")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 13))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::union")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 14))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item"))) (kind specialization) (ordinal 0))
      (authored-target "Object")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes"))) (kind featureTyping) (ordinal 0))
      (authored-target "StructuredSpaceObject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "faces")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind item) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "edges")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "face")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::face")))))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "faces")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "boundingShape")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::boundingShape")))))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "edge")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::edge")))))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 2))
      (authored-target "boundingShape")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::boundingShape")))))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "inter::intersectionsOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "inter::intersectionsOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "contains")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "isEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 1))
      (authored-target "union")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 1))
      (authored-target "contains")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 2))
      (authored-target "union")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::edge"))) (kind redefinition) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::self")))))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::face"))) (kind redefinition) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::self")))))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::boundingShape"))) (kind featureTyping) (ordinal 0))
      (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item")))))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::boundingShape"))) (kind redefinition) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::self")))))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::done"))) (kind featureTyping) (ordinal 0))
      (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item")))))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::done"))) (kind redefinition) (ordinal 0))
      (authored-target "endShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::envelopingShapes"))) (kind featureTyping) (ordinal 0))
      (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item")))))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "innerSpaceDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 1))))) (kind expressionOperand) (ordinal 0))
      (authored-target "outerSpaceDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 1))))) (kind invocationCallee) (ordinal 0))
      (authored-target "notEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::envelopingShapes::envelopedItem"))) (kind redefinition) (ordinal 0))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::isSolid"))) (kind expressionOperand) (ordinal 0))
      (authored-target "voids")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::voids")))))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::isSolid"))) (kind invocationCallee) (ordinal 0))
      (authored-target "isEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item")))))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::self"))) (kind redefinition) (ordinal 0))
      (authored-target "Object::self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::shape"))) (kind featureTyping) (ordinal 0))
      (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item")))))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::shape"))) (kind redefinition) (ordinal 0))
      (authored-target "spaceBoundary")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::start"))) (kind featureTyping) (ordinal 0))
      (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item")))))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::start"))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::subparts"))) (kind featureTyping) (ordinal 0))
      (authored-target "Part")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::subparts"))) (kind subsetting) (ordinal 0))
      (authored-target "subitems")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::subparts"))) (kind subsetting) (ordinal 1))
      (authored-target "parts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::voids"))) (kind redefinition) (ordinal 0))
      (authored-target "innerSpaceOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Touches"))) (kind specialization) (ordinal 0))
      (authored-target "JustOutsideOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Touches"))) (kind specialization) (ordinal 1))
      (authored-target "HappensWhile")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::face"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::boundingShape"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::edge"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::boundingShape"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::edge"))) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::edge"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::face"))) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::face"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::boundingShape"))) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::boundingShape"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::boundingShape"))) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::boundingShape"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::done"))) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::done"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::envelopingShapes"))) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::envelopingShapes"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::isSolid"))) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::voids"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::isSolid"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::self"))) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::shape"))) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::shape"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::start"))) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::start"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::isSolid"))) (value (kind non-constant)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/items.md") (range (start 7 16) (end 7 31)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::Object")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 8 16) (end 8 32)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::objects")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 9 16) (end 9 27)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Parts::Part")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 10 16) (end 10 28)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Parts::parts")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 11 16) (end 11 41)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensWhile")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 12 16) (end 12 42)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::JustOutsideOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 13 16) (end 13 46)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::StructuredSpaceObject")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 14 16) (end 14 44)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "Constraints::ConstraintCheck")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 15 16) (end 15 45)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "Constraints::constraintChecks")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 16 16) (end 16 45)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "CollectionFunctions::contains")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 17 16) (end 17 42)) (probe (position 17 16))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 18 16) (end 18 43)) (probe (position 18 16))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::notEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 19 16) (end 19 43)) (probe (position 19 16))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::includes")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 20 16) (end 20 40)) (probe (position 20 16))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 13))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::union")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 21 16) (end 21 40)) (probe (position 21 16))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind import) (ordinal 14))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 23 27) (end 23 33)) (probe (position 23 27))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item"))) (kind specialization) (ordinal 0) (authored-target "Object")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 75 24) (end 75 45)) (probe (position 75 24))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes"))) (kind featureTyping) (ordinal 0) (authored-target "StructuredSpaceObject")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 84 20) (end 84 25)) (probe (position 84 20))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "faces")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 89 20) (end 89 25)) (probe (position 89 20))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind item) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "edges")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 87 62) (end 87 66)) (probe (position 87 62))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "face")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::face")))))
  )
  (query (document "memory://snapshot/items.md") (range (start 92 32) (end 92 37)) (probe (position 92 32))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "faces")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 87 68) (end 87 81)) (probe (position 87 68))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "boundingShape")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::boundingShape")))))
  )
  (query (document "memory://snapshot/items.md") (range (start 93 45) (end 93 49)) (probe (position 93 45))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "edge")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::edge")))))
  )
  (query (document "memory://snapshot/items.md") (range (start 93 51) (end 93 64)) (probe (position 93 51))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 2) (authored-target "boundingShape")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::boundingShape")))))
  )
  (query (document "memory://snapshot/items.md") (range (start 87 33) (end 87 54)) (probe (position 87 33))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "inter::intersectionsOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 93 16) (end 93 37)) (probe (position 93 16))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "inter::intersectionsOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 87 24) (end 87 32)) (probe (position 87 24))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "contains")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 92 24) (end 92 31)) (probe (position 92 24))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "isEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 87 56) (end 87 61)) (probe (position 87 56))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 1) (authored-target "union")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 93 7) (end 93 15)) (probe (position 93 7))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 1) (authored-target "contains")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 93 39) (end 93 44)) (probe (position 93 39))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 2) (authored-target "union")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 90 22) (end 90 26)) (probe (position 90 22))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::edge"))) (kind redefinition) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::self")))))
  )
  (query (document "memory://snapshot/items.md") (range (start 85 22) (end 85 26)) (probe (position 85 22))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::::face"))) (kind redefinition) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::self")))))
  )
  (query (document "memory://snapshot/items.md") (range (start 82 27) (end 82 31)) (probe (position 82 27))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::boundingShape"))) (kind featureTyping) (ordinal 0) (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item")))))
  )
  (query (document "memory://snapshot/items.md") (range (start 82 36) (end 82 40)) (probe (position 82 36))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::boundingShapes::boundingShape"))) (kind redefinition) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::self")))))
  )
  (query (document "memory://snapshot/items.md") (range (start 33 13) (end 33 17)) (probe (position 33 13))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::done"))) (kind featureTyping) (ordinal 0) (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item")))))
  )
  (query (document "memory://snapshot/items.md") (range (start 33 22) (end 33 29)) (probe (position 33 22))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::done"))) (kind redefinition) (ordinal 0) (authored-target "endShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 42 26) (end 42 30)) (probe (position 42 26))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::envelopingShapes"))) (kind featureTyping) (ordinal 0) (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item")))))
  )
  (query (document "memory://snapshot/items.md") (range (start 56 7) (end 56 26)) (probe (position 56 7))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "innerSpaceDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 60 79) (end 60 98)) (probe (position 60 79))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 1))))) (kind expressionOperand) (ordinal 0) (authored-target "outerSpaceDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 60 70) (end 60 78)) (probe (position 60 70))
    (reference (id (source (node (document "memory://snapshot/items.md") (anonymous (kind constraint) (ordinal 1))))) (kind invocationCallee) (ordinal 0) (authored-target "notEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 49 30) (end 49 34)) (probe (position 49 30))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::envelopingShapes::envelopedItem"))) (kind redefinition) (ordinal 0) (authored-target "that")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 104 30) (end 104 35)) (probe (position 104 30))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::isSolid"))) (kind expressionOperand) (ordinal 0) (authored-target "voids")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::voids")))))
  )
  (query (document "memory://snapshot/items.md") (range (start 104 22) (end 104 29)) (probe (position 104 22))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::isSolid"))) (kind invocationCallee) (ordinal 0) (authored-target "isEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 30 12) (end 30 16)) (probe (position 30 12))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::self"))) (kind featureTyping) (ordinal 0) (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item")))))
  )
  (query (document "memory://snapshot/items.md") (range (start 30 21) (end 30 33)) (probe (position 30 21))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::self"))) (kind redefinition) (ordinal 0) (authored-target "Object::self")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 35 15) (end 35 19)) (probe (position 35 15))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::shape"))) (kind featureTyping) (ordinal 0) (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item")))))
  )
  (query (document "memory://snapshot/items.md") (range (start 35 24) (end 35 37)) (probe (position 35 24))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::shape"))) (kind redefinition) (ordinal 0) (authored-target "spaceBoundary")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 32 14) (end 32 18)) (probe (position 32 14))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::start"))) (kind featureTyping) (ordinal 0) (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/items.md") (qualified-name "Items::Item")))))
  )
  (query (document "memory://snapshot/items.md") (range (start 32 23) (end 32 32)) (probe (position 32 23))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::start"))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 120 26) (end 120 30)) (probe (position 120 26))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::subparts"))) (kind featureTyping) (ordinal 0) (authored-target "Part")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 120 40) (end 120 48)) (probe (position 120 40))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::subparts"))) (kind subsetting) (ordinal 0) (authored-target "subitems")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 120 50) (end 120 55)) (probe (position 120 50))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::subparts"))) (kind subsetting) (ordinal 1) (authored-target "parts")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 97 17) (end 97 38)) (probe (position 97 17))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Item::voids"))) (kind redefinition) (ordinal 0) (authored-target "innerSpaceOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 135 27) (end 135 40)) (probe (position 135 27))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Touches"))) (kind specialization) (ordinal 0) (authored-target "JustOutsideOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/items.md") (range (start 135 42) (end 135 54)) (probe (position 135 42))
    (reference (id (source (node (document "memory://snapshot/items.md") (qualified-name "Items::Touches"))) (kind specialization) (ordinal 1) (authored-target "HappensWhile")
      (outcome (status unresolved)))
  )
)
~~~
