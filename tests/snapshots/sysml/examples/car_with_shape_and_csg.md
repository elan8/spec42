# META
~~~ini
description=SysML Example (Geometry): CarWithShapeAndCSG
type=file
~~~
# SOURCE
~~~sysml
package CarWithShapeAndCSG {
	private import SpatialItems::*;
	private import ShapeItems::*;
	private import Objects::Point;
	private import Quantities::VectorQuantityValue;
	private import MeasurementReferences::CoordinateFrame;
	private import MeasurementReferences::TranslationRotationSequence;
	private import MeasurementReferences::Translation;
	private import MeasurementReferences::Rotation;
	private import SI::*;

	part def Car :> SpatialItem {
		doc
		/*
		 * Car with simple engine
		 */
	
        item :>> shape = new Cuboid(4800 [mm], 1840 [mm], 1350 [mm]);

        attribute datum :>> coordinateFrame {
            :>> mRefs = (mm, mm, mm);
        }

		part powerSource : Engine [1] :> componentParts {
			:>> ecf { 
				:>> mRefs = datum.mRefs;
				:>> transformation : TranslationRotationSequence {
					:>> source = datum;
					:>> elements = ( new Translation((3800, (1840-190)/2, 40)[datum]) );
				}
			}
		}
	}

	part def Engine :> SpatialItem {
		doc
		/*
		 * Simple 2-cylinder engine
		 * 
		 * Note: The engine shape is modeled as a rectangular box with two cylindrical holes, a gross simplification.
		 */
	
		item :>> shape [1];
		
		attribute <ecf> engineCoordinateFrame :>> coordinateFrame;		

		part rawEngineBlock :> subSpatialParts [1] {
			item :>> shape : Box [1] {
	    		:>> length = 300 [mm];
	    		:>> width = 190 [mm];
	    		:>> height = 330 [mm];
			}
		}
		
		private attribute rearCylinderSpacing = 90 [mm];
		private item cylinder1  :> subSpatialParts [1] {
			item :>> shape : Cylinder [1] {
	    		:>> radius = 55 [mm];
	    		:>> height = 350 [mm];
			}
			attribute :>> coordinateFrame {
				:>> transformation : TranslationRotationSequence {
					:>> source = ecf;
					:>> elements = (new Translation( (rearCylinderSpacing, rawEngineBlock.shape.width/2, -10)[ecf]));
				}
			}
		}
		
		private attribute cylinderSpacing = 2*cylinder1.shape.radius + 20 [mm];
		private item cylinder2  :> subSpatialParts [1] {
			item :>> shape : Cylinder [1] {
	    		:>> radius = cylinder1.shape.radius;
	    		:>> height = cylinder1.shape.height;
			}
			attribute :>> coordinateFrame {
				:>> transformation : TranslationRotationSequence {
					:>> source = ecf;
					:>> elements = ( new Translation((rearCylinderSpacing + cylinderSpacing, rawEngineBlock.shape.width/2, -10)[ecf]) );
				}
			}
		}

		/* CSG difference of rawEngineBlock minus cylinder1 minus cylinder2 */
		attribute :> differencesOf[1] {
			item :>> elements = (rawEngineBlock, cylinder1, cylinder2);
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/car_with_shape_and_csg.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 16) (end 5 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 11 17) (end 11 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 17) (end 17 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 28) (end 19 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 20 12) (end 20 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 35) (end 23 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 24 7) (end 24 10))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 25 4) (end 25 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 26 4) (end 29 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 34 20) (end 34 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 11) (end 42 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 44 44) (end 44 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 46 25) (end 46 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 47 12) (end 47 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 20) (end 47 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 48 7) (end 48 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 49 7) (end 49 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 50 7) (end 50 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 29) (end 55 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 56 12) (end 56 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 20) (end 56 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 57 7) (end 57 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 58 7) (end 58 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 60 17) (end 60 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 61 4) (end 64 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 68 40) (end 68 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 29) (end 69 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 70 12) (end 70 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 70 20) (end 70 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 71 7) (end 71 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 72 7) (end 72 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 17) (end 74 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 75 4) (end 78 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 83 15) (end 83 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 84 12) (end 84 20))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:0f2e470afa5f75ac99872a78f0b662841fb97bd610aff77b8325a31e67dc8369") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SpatialItems") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ShapeItems") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::Point") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::VectorQuantityValue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::CoordinateFrame") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::TranslationRotationSequence") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::Translation") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::Rotation") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Car with simple engine\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SpatialItem")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Car")) (anonymous (kind item) (ordinal 0))))) (kind item) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "shape")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::datum"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")) (subsetting (reference "componentParts")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Car")) (named (kind part) (name "powerSource")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "ecf")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Simple 2-cylinder engine\n\t\t * \n\t\t * Note: The engine shape is modeled as a rectangular box with two cylindrical holes, a gross simplification.\n\t\t ")) (comment (text " CSG difference of rawEngineBlock minus cylinder1 minus cylinder2 "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SpatialItem")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind item) (ordinal 0))))) (kind item) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "shape")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "differencesOf")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind item) (ordinal 0))))) (kind item) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "elements")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1"))) (kind item) (membership (kind feature) (visibility private)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility private)) (relationships (subsetting (reference "subSpatialParts")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder1")) (anonymous (kind item) (ordinal 0))))) (kind item) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Cylinder")) (redefinition (reference "shape")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder1")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2"))) (kind item) (membership (kind feature) (visibility private)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility private)) (relationships (subsetting (reference "subSpatialParts")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder2")) (anonymous (kind item) (ordinal 0))))) (kind item) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Cylinder")) (redefinition (reference "shape")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder2")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinderSpacing"))) (kind attribute) (membership (kind feature) (visibility private)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility private)) (relationships (memberAccessOperand (reference "cylinder1::shape::radius")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::engineCoordinateFrame"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (short-name "ecf")) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::rawEngineBlock"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "subSpatialParts")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind part) (name "rawEngineBlock")) (anonymous (kind item) (ordinal 0))))) (kind item) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Box")) (redefinition (reference "shape")))))
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::rearCylinderSpacing"))) (kind attribute) (membership (kind feature) (visibility private)) (feature-value (kind bind)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SpatialItems")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ShapeItems")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 8))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::CoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::Translation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::Rotation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car"))) (kind specialization) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Car")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "shape")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::datum"))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine")))))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (kind subsetting) (ordinal 0))
      (authored-target "componentParts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Car")) (named (kind part) (name "powerSource")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "ecf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine"))) (kind specialization) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind attribute) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "differencesOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "shape")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder1")) (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Cylinder")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder1")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "shape")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder1")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder2")) (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Cylinder")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder2")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "shape")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder2")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinderSpacing"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "cylinder1::shape::radius")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::engineCoordinateFrame"))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::rawEngineBlock"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind part) (name "rawEngineBlock")) (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Box")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind part) (name "rawEngineBlock")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "shape")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Car")) (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::datum"))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Car")) (named (kind part) (name "powerSource")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind attribute) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1"))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder1")) (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder1")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2"))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder2")) (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder2")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinderSpacing"))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::engineCoordinateFrame"))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::rawEngineBlock"))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind part) (name "rawEngineBlock")) (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::rawEngineBlock"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::rearCylinderSpacing"))) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinderSpacing"))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::rearCylinderSpacing"))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 90))) (unit "mm")))
    (unit (declaration (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinderSpacing"))) (ordinal 0) (authored "mm") (start 68 69) (end 68 71) (outcome (status catalog-unavailable)))
    (unit (declaration (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::rearCylinderSpacing"))) (ordinal 0) (authored "mm") (start 54 46) (end 54 48) (outcome (status catalog-unavailable)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Car")) (anonymous (kind item) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::datum")))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::powerSource")))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car")))
      (type (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Car")) (named (kind part) (name "powerSource")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::powerSource")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine")))
      (subtype (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::powerSource")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind item) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind item) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind attribute) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1")))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder1")) (anonymous (kind item) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder1")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2")))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder2")) (anonymous (kind item) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder2")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinderSpacing")))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::engineCoordinateFrame")))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::rawEngineBlock")))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind part) (name "rawEngineBlock")) (anonymous (kind item) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::rawEngineBlock")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::rearCylinderSpacing")))
      (featured-by (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "SpatialItems")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 2 16) (end 2 29)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ShapeItems")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 9 16) (end 9 21)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 8))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 3 16) (end 3 30)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::Point")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 4 16) (end 4 47)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::VectorQuantityValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 5 16) (end 5 54)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::CoordinateFrame")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 6 16) (end 6 66)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::TranslationRotationSequence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 7 16) (end 7 50)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::Translation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 8 16) (end 8 47)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::Rotation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 11 17) (end 11 28)) (probe (position 11 17))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car"))) (kind specialization) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 17 17) (end 17 22)) (probe (position 17 17))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Car")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "shape")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 19 28) (end 19 43)) (probe (position 19 28))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::datum"))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 23 21) (end 23 27)) (probe (position 23 21))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine")))))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 23 35) (end 23 49)) (probe (position 23 35))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (kind subsetting) (ordinal 0) (authored-target "componentParts")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 24 7) (end 24 10)) (probe (position 24 7))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Car")) (named (kind part) (name "powerSource")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "ecf")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 34 20) (end 34 31)) (probe (position 34 20))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine"))) (kind specialization) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 83 15) (end 83 28)) (probe (position 83 15))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind attribute) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "differencesOf")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 42 11) (end 42 16)) (probe (position 42 11))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "shape")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 84 12) (end 84 20)) (probe (position 84 12))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 55 29) (end 55 44)) (probe (position 55 29))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 56 20) (end 56 28)) (probe (position 56 20))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder1")) (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 56 12) (end 56 17)) (probe (position 56 12))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder1")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "shape")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 60 17) (end 60 32)) (probe (position 60 17))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder1")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 69 29) (end 69 44)) (probe (position 69 29))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 70 20) (end 70 28)) (probe (position 70 20))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder2")) (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 70 12) (end 70 17)) (probe (position 70 12))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder2")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "shape")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 74 17) (end 74 32)) (probe (position 74 17))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind item) (name "cylinder2")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 68 40) (end 68 62)) (probe (position 68 40))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::cylinderSpacing"))) (kind memberAccessOperand) (ordinal 0) (authored-target "cylinder1::shape::radius")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 44 44) (end 44 59)) (probe (position 44 44))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::engineCoordinateFrame"))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 46 25) (end 46 40)) (probe (position 46 25))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (qualified-name "CarWithShapeAndCSG::Engine::rawEngineBlock"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 47 20) (end 47 23)) (probe (position 47 20))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind part) (name "rawEngineBlock")) (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Box")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_shape_and_csg.md") (range (start 47 12) (end 47 17)) (probe (position 47 12))
    (reference (id (source (node (document "memory://snapshot/car_with_shape_and_csg.md") (path (named (kind package) (name "CarWithShapeAndCSG")) (named (kind part-def) (name "Engine")) (named (kind part) (name "rawEngineBlock")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "shape")
      (outcome (status unresolved)))
    )
  )
)
~~~
