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
  (document "car_with_shape_and_csg.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 26))
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
        (range (start 9 16) (end 9 18))
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
        (range (start 19 28) (end 19 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 21) (end 23 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 35) (end 23 49))
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
        (range (start 44 44) (end 44 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 46 25) (end 46 40))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 54 2) (end 54 50))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 68 2) (end 68 73))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 83 2) (end 83 102))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "1d3e8936b4ebdeed61bdc5d2c1441bbcbb77ae2fac103cb96a76be34ff46e896") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (kind "package") (name "CarWithShapeAndCSG") (declared-name "CarWithShapeAndCSG"))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "SpatialItems::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "ShapeItems::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))) (kind "part def") (name "Car") (declared-name "Car") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SpatialItem")))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::datum"))) (kind "attribute") (name "datum") (declared-name "datum") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (kind "part") (name "powerSource") (declared-name "powerSource") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")) (subsetting (reference "componentParts")))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource::ecf"))) (kind "attribute") (name "ecf") (declared-name "ecf") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "ecf")))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape"))) (kind "item") (name "shape") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "shape")))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::CoordinateFrame"))) (kind "import") (name "CoordinateFrame") (declared-name "CoordinateFrame") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::CoordinateFrame") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SpatialItem")))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1"))) (kind "item") (name "cylinder1") (declared-name "cylinder1") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2"))) (kind "item") (name "cylinder2") (declared-name "cylinder2") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinderSpacing"))) (kind "attribute") (name "cylinderSpacing") (declared-name "cylinderSpacing") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::engineCoordinateFrame"))) (kind "attribute") (name "engineCoordinateFrame") (declared-name "engineCoordinateFrame") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::rawEngineBlock"))) (kind "part") (name "rawEngineBlock") (declared-name "rawEngineBlock") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::rearCylinderSpacing"))) (kind "attribute") (name "rearCylinderSpacing") (declared-name "rearCylinderSpacing") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::shape"))) (kind "item") (name "shape") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "shape")))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Point"))) (kind "import") (name "Point") (declared-name "Point") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Point") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Rotation"))) (kind "import") (name "Rotation") (declared-name "Rotation") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::Rotation") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Translation"))) (kind "import") (name "Translation") (declared-name "Translation") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::Translation") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::TranslationRotationSequence"))) (kind "import") (name "TranslationRotationSequence") (declared-name "TranslationRotationSequence") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::TranslationRotationSequence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::VectorQuantityValue"))) (kind "import") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::VectorQuantityValue") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SpatialItems::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ShapeItems::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))) (kind specialization) (ordinal 0)) (authored-target "SpatialItem") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::datum"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (kind subsetting) (ordinal 0)) (authored-target "componentParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource::ecf"))) (kind redefinition) (ordinal 0)) (authored-target "ecf") (outcome (status resolved) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource::ecf")))))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape"))) (kind redefinition) (ordinal 0)) (authored-target "shape") (outcome (status resolved) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape")))))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::CoordinateFrame"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::CoordinateFrame") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))) (kind specialization) (ordinal 0)) (authored-target "SpatialItem") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::engineCoordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::rawEngineBlock"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::shape"))) (kind redefinition) (ordinal 0)) (authored-target "shape") (outcome (status resolved) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::shape")))))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Point"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Point") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Rotation"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::Rotation") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Translation"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::Translation") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::TranslationRotationSequence"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::TranslationRotationSequence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::VectorQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::VectorQuantityValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource::ecf"))) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource::ecf"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource::ecf"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape"))) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1::coordinateFrame"))) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2::coordinateFrame"))) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::shape"))) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::shape"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::shape"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinderSpacing")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::rearCylinderSpacing")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 9 16) (end 9 18)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 9 16) (end 9 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 17) (end 17 22)) (probe (position 17 17))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape"))
        (kind redefinition) (ordinal 0) (authored-target "shape")
        (range (start 17 17) (end 17 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape") (range (start 17 8) (end 17 69)))
        )
      )
    )
    (query (range (start 42 11) (end 42 16)) (probe (position 42 11))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::shape"))
        (kind redefinition) (ordinal 0) (authored-target "shape")
        (range (start 42 11) (end 42 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::shape") (range (start 42 2) (end 42 21)))
        )
      )
    )
    (query (range (start 23 21) (end 23 27)) (probe (position 23 21))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 23 21) (end 23 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 24 3) (end 24 10)) (probe (position 24 3))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource::ecf"))
        (kind redefinition) (ordinal 0) (authored-target "ecf")
        (range (start 24 3) (end 24 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource::ecf") (range (start 24 3) (end 24 207)))
        )
      )
    )
    (query (range (start 2 16) (end 2 26)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "ShapeItems::*")
        (range (start 2 16) (end 2 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 17) (end 11 28)) (probe (position 11 17))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))
        (kind specialization) (ordinal 0) (authored-target "SpatialItem")
        (range (start 11 17) (end 11 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 34 20) (end 34 31)) (probe (position 34 20))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))
        (kind specialization) (ordinal 0) (authored-target "SpatialItem")
        (range (start 34 20) (end 34 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "SpatialItems::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 30)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::Point"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::Point")
        (range (start 3 16) (end 3 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 23 35) (end 23 49)) (probe (position 23 35))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))
        (kind subsetting) (ordinal 0) (authored-target "componentParts")
        (range (start 23 35) (end 23 49))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 28) (end 19 43)) (probe (position 19 28))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::Car::datum"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 19 28) (end 19 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 44 44) (end 44 59)) (probe (position 44 44))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::engineCoordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 44 44) (end 44 59))
        (outcome (status unresolved))
      )
    )
    (query (range (start 46 25) (end 46 40)) (probe (position 46 25))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::rawEngineBlock"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 46 25) (end 46 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 60 17) (end 60 32)) (probe (position 60 17))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 60 17) (end 60 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1::coordinateFrame") (range (start 60 3) (end 60 226)))
        )
      )
    )
    (query (range (start 74 17) (end 74 32)) (probe (position 74 17))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 74 17) (end 74 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2::coordinateFrame") (range (start 74 3) (end 74 245)))
        )
      )
    )
    (query (range (start 4 16) (end 4 47)) (probe (position 4 16))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::VectorQuantityValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::VectorQuantityValue")
        (range (start 4 16) (end 4 47))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 47)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::Rotation"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::Rotation")
        (range (start 8 16) (end 8 47))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 50)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::Translation"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::Translation")
        (range (start 7 16) (end 7 50))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 16) (end 5 54)) (probe (position 5 16))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::CoordinateFrame"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::CoordinateFrame")
        (range (start 5 16) (end 5 54))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 16) (end 6 66)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "CarWithShapeAndCSG::TranslationRotationSequence"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::TranslationRotationSequence")
        (range (start 6 16) (end 6 66))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
