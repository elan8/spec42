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
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwItem,Ident,Colon,Ident,ColonGtGt,Ident,Semicolon,
KwItem,Ident,Colon,Ident,ColonGtGt,Ident,Semicolon,
KwItem,Ident,Colon,Ident,ColonGtGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwItem,Ident,ColonGtGt,Ident,Semicolon,
KwAssert,KwConstraint,OpenCurly,
KwDoc,
RegularComment,
Ident,EqEq,
OpenParen,KwIf,Ident,Dot,Ident,EqEq,DecimalValue,Pipe,Ident,Dot,Ident,EqEq,DecimalValue,Question,DecimalValue,
KwElse,Ident,Dot,Ident,Minus,DecimalValue,CloseParen,
CloseCurly,
KwAssert,KwConstraint,OpenCurly,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,OpenAngle,DecimalValue,KwImplies,Ident,OpenParen,Ident,CloseParen,CloseCurly,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAssert,KwConstraint,OpenCurly,
KwDoc,
RegularComment,
Ident,Dot,Ident,Dot,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,KwAnd,
Ident,Dot,Ident,Arrow,Ident,OpenParen,Ident,Dot,Ident,CloseParen,
CloseCurly,
CloseCurly,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwItem,Ident,Colon,Ident,ColonGtGt,Ident,Semicolon,
KwPrivate,KwItem,ColonGtGt,Ident,OpenCurly,
KwRef,KwItem,Ident,ColonGtGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,Dot,Ident,Comma,Ident,OpenParen,Ident,Comma,Ident,CloseParen,CloseParen,CloseCurly,
CloseCurly,
KwPrivate,KwItem,ColonGtGt,Ident,OpenCurly,
KwRef,KwItem,Ident,ColonGtGt,Ident,Semicolon,
KwItem,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,KwImplies,
Ident,OpenParen,Ident,Dot,Ident,Comma,Ident,OpenParen,Ident,Comma,Ident,CloseParen,CloseParen,CloseCurly,
CloseCurly,
CloseCurly,
KwItem,Ident,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Eq,Ident,OpenParen,Ident,CloseParen,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwRef,KwRedefines,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwConstraint,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwConnection,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwItem,Ident,ColonGtGt,Ident,Comma,Ident,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwItem,Ident,ColonGtGt,Ident,Comma,Ident,Semicolon,
CloseCurly,
KwAbstract,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Items'
    (documentation)
    (import_decl private 'Objects::Object')
    (import_decl private 'Objects::objects')
    (import_decl private 'Parts::Part')
    (import_decl private 'Parts::parts')
    (import_decl private 'Occurrences::HappensWhile')
    (import_decl private 'Occurrences::JustOutsideOf')
    (import_decl private 'Objects::StructuredSpaceObject')
    (import_decl private 'Constraints::ConstraintCheck')
    (import_decl private 'Constraints::constraintChecks')
    (import_decl private 'CollectionFunctions::contains')
    (import_decl private 'SequenceFunctions::isEmpty')
    (import_decl private 'SequenceFunctions::notEmpty')
    (import_decl private 'SequenceFunctions::includes')
    (import_decl private 'SequenceFunctions::union')
    (import_decl private 'ControlFunctions::forAll')
    (item_def abstract 'Item' :> 'Object'
      (documentation)
      (ref_usage ref 'self' : 'Item' :>> 'Object::self')
      (item_usage 'start' : 'Item' :>> 'startShot')
      (item_usage 'done' : 'Item' :>> 'endShot')
      (item_usage 'shape' : 'Item' :>> 'spaceBoundary'
        (documentation))
      (item_usage 'envelopingShapes' : 'Item' multiplicity
        (documentation)
        (item_usage ref 'envelopedItem' :>> 'that')
        (sysml_decl
          (documentation)
          (result_expr_member))
        (sysml_decl
          (result_expr_member))
        (item_usage 'envelopingItem' multiplicity)
        (sysml_decl
          (documentation)
          (result_expr_member)))
      (item_usage 'boundingShapes' : 'StructuredSpaceObject' :> 'envelopingShapes' multiplicity
        (documentation)
        (item_usage ref 'boundingShape' : 'Item' :>> 'self')
        (item_usage private :>> 'faces'
          (item_usage ref 'face' :>> 'self')
          (item_usage 'inter' multiplicity)
          (sysml_decl
            (result_expr_member)))
        (item_usage private :>> 'edges'
          (item_usage ref 'edge' :>> 'self')
          (item_usage 'inter' multiplicity)
          (sysml_decl
            (result_expr_member))))
      (item_usage 'voids' :>> 'innerSpaceOccurrences' multiplicity
        (documentation))
      (attribute_usage 'isSolid' value
        (documentation))
      (item_usage abstract 'subitems' : 'Item' :> 'items', 'subobjects' multiplicity
        (documentation)
        (ref_usage private ref :>> 'Item::incomingTransferSort', 'subobjects::incomingTransferSort'))
      (part_usage abstract 'subparts' : 'Part' :> 'subitems', 'parts' multiplicity
        (documentation))
      (constraint_usage abstract 'checkedConstraints' : 'ConstraintCheck' multiplicity :> 'constraintChecks', 'ownedPerformances'
        (documentation)))
    (connection_def 'Touches' :> 'JustOutsideOf', 'HappensWhile'
      (documentation)
      (interface_end end 'touchesToo' :>> 'separateSpaceToo', 'thisOccurrence' multiplicity)
      (interface_end end 'touches' :>> 'separateSpace', 'thatOccurrence' multiplicity))
    (item_usage abstract 'items' : 'Item' :> 'objects' multiplicity nonunique
      (documentation))))
~~~
# EXPECTED
~~~
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.unresolved_name 'Object'
semantic.unresolved_name 'Object::self'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'spaceBoundary'
semantic.unresolved_name 'that'
semantic.unresolved_name 'StructuredSpaceObject'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'innerSpaceOccurrences'
semantic.unresolved_name 'subobjects'
semantic.unresolved_name 'Item::incomingTransferSort'
semantic.unresolved_name 'subobjects::incomingTransferSort'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'parts'
semantic.unresolved_name 'ConstraintCheck'
semantic.unresolved_name 'constraintChecks'
semantic.unresolved_name 'ownedPerformances'
semantic.unresolved_name 'JustOutsideOf'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'separateSpaceToo'
semantic.unresolved_name 'thisOccurrence'
semantic.unresolved_name 'separateSpace'
semantic.unresolved_name 'thatOccurrence'
semantic.unresolved_name 'objects'
~~~
# PROBLEMS
~~~
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.unresolved_name 'Object'
semantic.unresolved_name 'Object::self'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'spaceBoundary'
semantic.unresolved_name 'that'
semantic.unresolved_name 'StructuredSpaceObject'
semantic.unresolved_name 'faces'
semantic.unresolved_name 'edges'
semantic.unresolved_name 'innerSpaceOccurrences'
semantic.unresolved_name 'subobjects'
semantic.unresolved_name 'Item::incomingTransferSort'
semantic.unresolved_name 'subobjects::incomingTransferSort'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'parts'
semantic.unresolved_name 'ConstraintCheck'
semantic.unresolved_name 'constraintChecks'
semantic.unresolved_name 'ownedPerformances'
semantic.unresolved_name 'JustOutsideOf'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'separateSpaceToo'
semantic.unresolved_name 'thisOccurrence'
semantic.unresolved_name 'separateSpace'
semantic.unresolved_name 'thatOccurrence'
semantic.unresolved_name 'objects'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f3876a811282b4078b6c7ee27b750afe2a8383430fe1a1f3e68a2127e1019c7f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Items"))) (kind "package") (name "Items") (declared-name "Items") (range (start (line 0) (character 0)) (end (line 0) (character 4121))))
    (element (id (node (document "d0") (qualified-name "Items::ConstraintCheck"))) (kind "import") (name "ConstraintCheck") (declared-name "ConstraintCheck") (range (start (line 14) (character 1)) (end (line 14) (character 45))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Constraints::ConstraintCheck") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 16)) (end (line 14) (character 44))))))
    (element (id (node (document "d0") (qualified-name "Items::HappensWhile"))) (kind "import") (name "HappensWhile") (declared-name "HappensWhile") (range (start (line 11) (character 1)) (end (line 11) (character 42))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensWhile") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Items::Item"))) (kind "item def") (name "Item") (declared-name "Item") (range (start (line 23) (character 1)) (end (line 23) (character 2898))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Object") (range (start (line 23) (character 27)) (end (line 23) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Items::Item::_documentation"))) (kind "documentation") (name "") (range (start (line 23) (character 1)) (end (line 23) (character 2898))) (parent (node (document "d0") (qualified-name "Items::Item"))))
    (element (id (node (document "d0") (qualified-name "Items::Item::isSolid"))) (kind "attribute") (name "isSolid") (declared-name "isSolid") (range (start (line 104) (character 2)) (end (line 104) (character 105))) (parent (node (document "d0") (qualified-name "Items::Item"))))
    (element (id (node (document "d0") (qualified-name "Items::Item::isSolid::_documentation"))) (kind "documentation") (name "") (range (start (line 104) (character 2)) (end (line 104) (character 105))) (parent (node (document "d0") (qualified-name "Items::Item::isSolid"))))
    (element (id (node (document "d0") (qualified-name "Items::Item::self"))) (kind "ref") (name "self") (declared-name "self") (range (start (line 30) (character 2)) (end (line 30) (character 34))) (parent (node (document "d0") (qualified-name "Items::Item"))) (authored (membership (kind Feature)) (relationships (typing (reference "Item") (range (start (line 30) (character 11)) (end (line 30) (character 16)))) (redefinition (reference "Object::self") (range (start (line 30) (character 21)) (end (line 30) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Items::Item::subparts"))) (kind "part") (name "subparts") (declared-name "subparts") (range (start (line 120) (character 2)) (end (line 120) (character 129))) (parent (node (document "d0") (qualified-name "Items::Item"))) (authored (membership (kind Feature)) (relationships (typing (reference "Part") (range (start (line 120) (character 26)) (end (line 120) (character 30)))) (subsetting (reference "subitems") (range (start (line 120) (character 40)) (end (line 120) (character 48)))) (subsetting (reference "parts") (range (start (line 120) (character 50)) (end (line 120) (character 55)))))))
    (element (id (node (document "d0") (qualified-name "Items::Item::subparts::_documentation"))) (kind "documentation") (name "") (range (start (line 120) (character 2)) (end (line 120) (character 129))) (parent (node (document "d0") (qualified-name "Items::Item::subparts"))))
    (element (id (node (document "d0") (qualified-name "Items::JustOutsideOf"))) (kind "import") (name "JustOutsideOf") (declared-name "JustOutsideOf") (range (start (line 12) (character 1)) (end (line 12) (character 43))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::JustOutsideOf") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Items::Object"))) (kind "import") (name "Object") (declared-name "Object") (range (start (line 7) (character 1)) (end (line 7) (character 32))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Object") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 31))))))
    (element (id (node (document "d0") (qualified-name "Items::Part"))) (kind "import") (name "Part") (declared-name "Part") (range (start (line 9) (character 1)) (end (line 9) (character 28))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::Part") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 27))))))
    (element (id (node (document "d0") (qualified-name "Items::StructuredSpaceObject"))) (kind "import") (name "StructuredSpaceObject") (declared-name "StructuredSpaceObject") (range (start (line 13) (character 1)) (end (line 13) (character 47))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::StructuredSpaceObject") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 16)) (end (line 13) (character 46))))))
    (element (id (node (document "d0") (qualified-name "Items::Touches"))) (kind "connection def") (name "Touches") (declared-name "Touches") (range (start (line 135) (character 1)) (end (line 135) (character 310))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Owning)) (relationships (specializes (reference "JustOutsideOf") (range (start (line 135) (character 27)) (end (line 135) (character 40)))) (specializes (reference "HappensWhile") (range (start (line 135) (character 42)) (end (line 135) (character 54)))))))
    (element (id (node (document "d0") (qualified-name "Items::Touches::_documentation"))) (kind "documentation") (name "") (range (start (line 135) (character 1)) (end (line 135) (character 310))) (parent (node (document "d0") (qualified-name "Items::Touches"))))
    (element (id (node (document "d0") (qualified-name "Items::Touches::touches"))) (kind "interface end") (name "touches") (declared-name "touches") (range (start (line 142) (character 2)) (end (line 142) (character 72))) (parent (node (document "d0") (qualified-name "Items::Touches"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Items::Touches::touchesToo"))) (kind "interface end") (name "touchesToo") (declared-name "touchesToo") (range (start (line 141) (character 2)) (end (line 141) (character 81))) (parent (node (document "d0") (qualified-name "Items::Touches"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Items::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 4121))) (parent (node (document "d0") (qualified-name "Items"))))
    (element (id (node (document "d0") (qualified-name "Items::constraintChecks"))) (kind "import") (name "constraintChecks") (declared-name "constraintChecks") (range (start (line 15) (character 1)) (end (line 15) (character 46))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Constraints::constraintChecks") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 16)) (end (line 15) (character 45))))))
    (element (id (node (document "d0") (qualified-name "Items::contains"))) (kind "import") (name "contains") (declared-name "contains") (range (start (line 16) (character 1)) (end (line 16) (character 46))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "CollectionFunctions::contains") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 16)) (end (line 16) (character 45))))))
    (element (id (node (document "d0") (qualified-name "Items::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (range (start (line 21) (character 1)) (end (line 21) (character 41))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 21) (character 16)) (end (line 21) (character 40))))))
    (element (id (node (document "d0") (qualified-name "Items::includes"))) (kind "import") (name "includes") (declared-name "includes") (range (start (line 19) (character 1)) (end (line 19) (character 44))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::includes") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 19) (character 16)) (end (line 19) (character 43))))))
    (element (id (node (document "d0") (qualified-name "Items::isEmpty"))) (kind "import") (name "isEmpty") (declared-name "isEmpty") (range (start (line 17) (character 1)) (end (line 17) (character 43))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::isEmpty") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 17) (character 16)) (end (line 17) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Items::items"))) (kind "item def") (name "items") (declared-name "items") (range (start (line 145) (character 1)) (end (line 145) (character 126))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Owning)) (relationships (specializes (reference "objects") (range (start (line 0) (character 0)) (end (line 0) (character 7)))))))
    (element (id (node (document "d0") (qualified-name "Items::items::_documentation"))) (kind "documentation") (name "") (range (start (line 145) (character 1)) (end (line 145) (character 126))) (parent (node (document "d0") (qualified-name "Items::items"))))
    (element (id (node (document "d0") (qualified-name "Items::notEmpty"))) (kind "import") (name "notEmpty") (declared-name "notEmpty") (range (start (line 18) (character 1)) (end (line 18) (character 44))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::notEmpty") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 18) (character 16)) (end (line 18) (character 43))))))
    (element (id (node (document "d0") (qualified-name "Items::objects"))) (kind "import") (name "objects") (declared-name "objects") (range (start (line 8) (character 1)) (end (line 8) (character 33))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::objects") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 32))))))
    (element (id (node (document "d0") (qualified-name "Items::parts"))) (kind "import") (name "parts") (declared-name "parts") (range (start (line 10) (character 1)) (end (line 10) (character 29))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::parts") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Items::union"))) (kind "import") (name "union") (declared-name "union") (range (start (line 20) (character 1)) (end (line 20) (character 41))) (parent (node (document "d0") (qualified-name "Items"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::union") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 20) (character 16)) (end (line 20) (character 40))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Items::ConstraintCheck"))) (kind membershipImport) (ordinal 0)) (authored-target "Constraints::ConstraintCheck") (range (start (line 14) (character 16)) (end (line 14) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::HappensWhile"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensWhile") (range (start (line 11) (character 16)) (end (line 11) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::Item"))) (kind specialization) (ordinal 0)) (authored-target "Object") (range (start (line 23) (character 27)) (end (line 23) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::Object")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::Item::self"))) (kind featureTyping) (ordinal 0)) (authored-target "Item") (range (start (line 30) (character 11)) (end (line 30) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::Item")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::Item::self"))) (kind redefinition) (ordinal 0)) (authored-target "Object::self") (range (start (line 30) (character 21)) (end (line 30) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::Item::subparts"))) (kind featureTyping) (ordinal 0)) (authored-target "Part") (range (start (line 120) (character 26)) (end (line 120) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::Item::subparts"))) (kind subsetting) (ordinal 0)) (authored-target "subitems") (range (start (line 120) (character 40)) (end (line 120) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::Item::subparts"))) (kind subsetting) (ordinal 1)) (authored-target "parts") (range (start (line 120) (character 50)) (end (line 120) (character 55))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::parts")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::JustOutsideOf"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::JustOutsideOf") (range (start (line 12) (character 16)) (end (line 12) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::Object"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Object") (range (start (line 7) (character 16)) (end (line 7) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::Part"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::Part") (range (start (line 9) (character 16)) (end (line 9) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::StructuredSpaceObject"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::StructuredSpaceObject") (range (start (line 13) (character 16)) (end (line 13) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::Touches"))) (kind specialization) (ordinal 0)) (authored-target "JustOutsideOf") (range (start (line 135) (character 27)) (end (line 135) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::JustOutsideOf")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::Touches"))) (kind specialization) (ordinal 1)) (authored-target "HappensWhile") (range (start (line 135) (character 42)) (end (line 135) (character 54))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::HappensWhile")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::Touches::touches"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::Touches::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::Touches::touchesToo"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::Touches::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::constraintChecks"))) (kind membershipImport) (ordinal 0)) (authored-target "Constraints::constraintChecks") (range (start (line 15) (character 16)) (end (line 15) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::contains"))) (kind membershipImport) (ordinal 0)) (authored-target "CollectionFunctions::contains") (range (start (line 16) (character 16)) (end (line 16) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (range (start (line 21) (character 16)) (end (line 21) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::includes"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::includes") (range (start (line 19) (character 16)) (end (line 19) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::isEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::isEmpty") (range (start (line 17) (character 16)) (end (line 17) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::items"))) (kind specialization) (ordinal 0)) (authored-target "objects") (range (start (line 0) (character 0)) (end (line 0) (character 7))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Items::objects")))))
    (reference (id (source (node (document "d0") (qualified-name "Items::notEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::notEmpty") (range (start (line 18) (character 16)) (end (line 18) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::objects"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::objects") (range (start (line 8) (character 16)) (end (line 8) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::parts"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::parts") (range (start (line 10) (character 16)) (end (line 10) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Items::union"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::union") (range (start (line 20) (character 16)) (end (line 20) (character 40))) (outcome (status unresolved)))
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
