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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Items"))) (name "Items") (declared-name "Items")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Items::ConstraintCheck"))) (name "ConstraintCheck") (declared-name "ConstraintCheck"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Items::HappensWhile"))) (name "HappensWhile") (declared-name "HappensWhile"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Items::Item"))) (name "Item") (declared-name "Item")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Items::Item::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Items::Item")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Items::Item::isSolid"))) (name "isSolid") (declared-name "isSolid") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Items::Item"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Items::Item::isSolid::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Items::Item")))))
              )
            )
            (element (kind "ref") (id (node (document "d0") (qualified-name "Items::Item::self"))) (name "self") (declared-name "self") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Items::Item")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Items::Item::subparts"))) (name "subparts") (declared-name "subparts") (declared (properties (abstract true) (composite true) (reference false) (ordered false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Items::Item"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Items::Item::subparts::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Items::Item")))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Items::JustOutsideOf"))) (name "JustOutsideOf") (declared-name "JustOutsideOf"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Items::Object"))) (name "Object") (declared-name "Object"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Items::Part"))) (name "Part") (declared-name "Part"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Items::StructuredSpaceObject"))) (name "StructuredSpaceObject") (declared-name "StructuredSpaceObject"))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "Items::Touches"))) (name "Touches") (declared-name "Touches")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Items::Touches::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Items::Touches")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Items::Touches::touches"))) (name "touches") (declared-name "touches") (declared (properties (end true)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Items::Touches")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Items::Touches::touchesToo"))) (name "touchesToo") (declared-name "touchesToo") (declared (properties (end true)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Items::Touches")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Items::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "Items::constraintChecks"))) (name "constraintChecks") (declared-name "constraintChecks"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Items::contains"))) (name "contains") (declared-name "contains"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Items::forAll"))) (name "forAll") (declared-name "forAll"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Items::includes"))) (name "includes") (declared-name "includes"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Items::isEmpty"))) (name "isEmpty") (declared-name "isEmpty"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Items::items"))) (name "items") (declared-name "items")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Items::items::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Items::items")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Items::notEmpty"))) (name "notEmpty") (declared-name "notEmpty"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Items::objects"))) (name "objects") (declared-name "objects"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Items::parts"))) (name "parts") (declared-name "parts"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Items::union"))) (name "union") (declared-name "union"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Items::Item::_documentation"))) (to (node (document "d0") (qualified-name "Items::Item"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Items::Item::isSolid::_documentation"))) (to (node (document "d0") (qualified-name "Items::Item::isSolid"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Items::Item::subparts::_documentation"))) (to (node (document "d0") (qualified-name "Items::Item::subparts"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Items::Touches::_documentation"))) (to (node (document "d0") (qualified-name "Items::Touches"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Items::_documentation"))) (to (node (document "d0") (qualified-name "Items"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Items::items::_documentation"))) (to (node (document "d0") (qualified-name "Items::items"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Items::Item::self"))) (to (node (document "d0") (qualified-name "Items::Item"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
