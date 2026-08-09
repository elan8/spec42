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
    doc /*
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
        doc /*
		 * Item is the most general class of objects that are part of, exist in or flow through a system. 
		 * Item is the base type of all ItemDefinitions.
		 */

        ref self : Item :>> Object::self;

        item start : Item :>> startShot;
        item done : Item :>> endShot;

        item shape : Item :>> spaceBoundary {
            doc /*
			 * The shape of an Item is its spatial boundary.
			 */
        }

        item envelopingShapes : Item [0..*] {
            doc /*
			 * Each enveloping shape is the shape of an Item that spacially overlaps this Item for its
			 * entire lifetime.
			 */

            ref item envelopedItem :>> that;

            assert constraint {
                doc /* 
                 * Enables two dimensional items to be enveloped by two or three dimensional shapes.
                 */
                = innerSpaceDimension == (if envelopedItem.innerSpaceDimension == 3 | envelopedItem.outerSpaceDimension == 3 ? 2 else envelopedItem.outerSpaceDimension - 1);
            }
            assert constraint {
                = (that as Item).innerSpaceDimension < 3 implies notEmpty(outerSpaceDimension);
            }

            item envelopingItem [1];

            assert constraint {
                doc /* 
				 * This constraint prevents an envelopingShape from being a portion.
				 */

                = envelopingItem.shape.spaceTimeCoincidentOccurrences->includes(that) and envelopingItem.spaceTimeEnclosedOccurrences->includes(that.that);
            }
        }

        item boundingShapes : StructuredSpaceObject :> envelopingShapes [0..*] {
            doc /*
			 * envelopingShapes that are structured space objects with every face or every edge
			 * intersecting this Item.
			 */

            ref item boundingShape : Item :>> self;

            private item :>> faces {
                ref item face :>> self;
                item inter [1];
                assert constraint {
                    = contains(inter.intersectionsOf, union(face, boundingShape));
                }
            }
            private item :>> edges {
                ref item edge :>> self;
                item inter [1];
                assert constraint {
                    = isEmpty(faces) implies contains(inter.intersectionsOf, union(edge, boundingShape));
                }
            }
        }

        item voids :>> innerSpaceOccurrences [0..*] {
            doc /*
			 * Voids are inner space occurrences of this Item.
			 */
        }

        attribute isSolid = isEmpty(voids) {
            doc /*
			 * An Item is solid if it has no voids.
			 */
        }

        abstract item subitems : Item :> items, subobjects [0..*] {
            doc /*
			 * The Items that are composite subitems of this Item.
			 */

            private ref  redefines Item::incomingTransferSort, subobjects::incomingTransferSort;
        }

        abstract part subparts : Part :> subitems, parts [0..*] {
            doc /*
			 * The subitems of this Item that are Parts.
			 */
        }

        abstract constraint checkedConstraints : ConstraintCheck [0..*] :> constraintChecks, ownedPerformances {
            doc /*
			 * Constraints that have been checked by this Item.
			 */
        }
    }

    connection def Touches :> JustOutsideOf, HappensWhile {
        doc /*
		 * Touching items are just outside each other and happen at the same time.
		 */

        end [0..*] touchesToo :>> separateSpaceToo, thisOccurrence;
        end [0..*] touches :>> separateSpace, thatOccurrence;
    }

    abstract item items : Item :> objects [0..*] nonunique {
        doc /*
		 * items is the base feature of all ItemUsages.
		 */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Items'
      (documentation)
      (membership_import private -> 'Objects::Object'[unresolved])
      (membership_import private -> 'Objects::objects'[unresolved])
      (membership_import private -> 'Parts::Part'[unresolved])
      (membership_import private -> 'Parts::parts'[unresolved])
      (membership_import private -> 'Occurrences::HappensWhile'[unresolved])
      (membership_import private -> 'Occurrences::JustOutsideOf'[unresolved])
      (membership_import private -> 'Objects::StructuredSpaceObject'[unresolved])
      (membership_import private -> 'Constraints::ConstraintCheck'[unresolved])
      (membership_import private -> 'Constraints::constraintChecks'[unresolved])
      (membership_import private -> 'CollectionFunctions::contains'[unresolved])
      (membership_import private -> 'SequenceFunctions::isEmpty'[unresolved])
      (membership_import private -> 'SequenceFunctions::notEmpty'[unresolved])
      (membership_import private -> 'SequenceFunctions::includes'[unresolved])
      (membership_import private -> 'SequenceFunctions::union'[unresolved])
      (membership_import private -> 'ControlFunctions::forAll'[unresolved])
      (item_def abstract 'Item' :> 'Object'[unresolved]
        (documentation)
        (reference_usage reference 'self' : 'Items::Item'[item_def] :>> 'Object::self'[unresolved])
        (item_usage composite 'start' : 'Items::Item'[item_def] :>> 'startShot'[unresolved] :> 'Items::Item::subitems'[item_usage][implied])
        (item_usage composite 'done' : 'Items::Item'[item_def] :>> 'endShot'[unresolved] :> 'Items::Item::subitems'[item_usage][implied])
        (item_usage composite 'shape' : 'Items::Item'[item_def] :>> 'spaceBoundary'[unresolved] :> 'Items::Item::subitems'[item_usage][implied]
          (documentation))
        (item_usage composite 'envelopingShapes' : 'Items::Item'[item_def] :> 'Items::Item::subitems'[item_usage][implied]
          (multiplicity_range [0..*])
          (documentation)
          (item_usage reference 'envelopedItem' :>> 'that'[unresolved] :> 'Items::items'[item_usage][implied])
          (assert_constraint_usage
            (documentation)
            (result_expr_membership))
          (assert_constraint_usage
            (result_expr_membership))
          (item_usage composite 'envelopingItem' :> 'Items::Item::subitems'[item_usage][implied]
            (multiplicity_range [1]))
          (assert_constraint_usage
            (documentation)
            (result_expr_membership)))
        (item_usage composite 'boundingShapes' : 'StructuredSpaceObject'[unresolved] :> 'Items::Item::envelopingShapes'[item_usage]
          (multiplicity_range [0..*])
          (documentation)
          (item_usage reference 'boundingShape' : 'Items::Item'[item_def] :>> 'Items::Item::self'[reference_usage] :> 'Items::items'[item_usage][implied])
          (item_usage composite :>> 'faces'[unresolved] :> 'Items::Item::subitems'[item_usage][implied]
            (item_usage reference 'face' :>> 'Items::Item::self'[reference_usage] :> 'Items::items'[item_usage][implied])
            (item_usage composite 'inter' :> 'Items::Item::subitems'[item_usage][implied]
              (multiplicity_range [1]))
            (assert_constraint_usage
              (result_expr_membership)))
          (item_usage composite :>> 'edges'[unresolved] :> 'Items::Item::subitems'[item_usage][implied]
            (item_usage reference 'edge' :>> 'Items::Item::self'[reference_usage] :> 'Items::items'[item_usage][implied])
            (item_usage composite 'inter' :> 'Items::Item::subitems'[item_usage][implied]
              (multiplicity_range [1]))
            (assert_constraint_usage
              (result_expr_membership))))
        (item_usage composite 'voids' :>> 'innerSpaceOccurrences'[unresolved] :> 'Items::Item::subitems'[item_usage][implied]
          (multiplicity_range [0..*])
          (documentation))
        (attribute_usage composite 'isSolid'
          (feature_value (=))
          (documentation))
        (item_usage abstract composite 'subitems' : 'Items::Item'[item_def] :> 'Items::items'[item_usage] :> 'subobjects'[unresolved]
          (multiplicity_range [0..*])
          (documentation)
          (reference_usage reference :>> 'Item::incomingTransferSort'[unresolved] :>> 'subobjects::incomingTransferSort'[unresolved]))
        (part_usage abstract composite 'subparts' : 'Part'[unresolved] :> 'Items::Item::subitems'[item_usage] :> 'parts'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (constraint_usage abstract composite 'checkedConstraints' : 'ConstraintCheck'[unresolved] :> 'constraintChecks'[unresolved] :> 'ownedPerformances'[unresolved]
          (multiplicity_range [0..*])
          (documentation)))
      (connection_def 'Touches' :> 'JustOutsideOf'[unresolved] :> 'HappensWhile'[unresolved]
        (documentation)
        (port_usage end 'touchesToo' :>> 'separateSpaceToo'[unresolved] :>> 'thisOccurrence'[unresolved]
          (multiplicity_range [0..*]))
        (port_usage end 'touches' :>> 'separateSpace'[unresolved] :>> 'thatOccurrence'[unresolved]
          (multiplicity_range [0..*])))
      (item_usage abstract 'items' : 'Items::Item'[item_def] :> 'objects'[unresolved]
        (multiplicity_range [0..*])
        (documentation)))))
~~~
