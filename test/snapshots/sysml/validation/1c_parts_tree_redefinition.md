# META
~~~ini
description=SysML Validation (01-Parts Tree): 1c-Parts Tree Redefinition
type=file
~~~
# SOURCE
~~~sysml
package '1c-Parts Tree Redefinition' {
	private import SI::kg;
	
	package Definitions {	
		part def Vehicle {
			attribute mass :> ISQ::mass;
		}		
		part def AxleAssembly;		
		part def Axle { 
			attribute mass :> ISQ::mass;
		}	
		part def FrontAxle :> Axle { 
			attribute steeringAngle: ScalarValues::Real;
		}	
		part def Wheel;	
	}
		
	package Usages {
		private import Definitions::*;
		
		part vehicle1: Vehicle {
			attribute mass redefines Vehicle::mass default = 1750 [kg] {
			doc
			/*
			 * The mass attribute is redefined to give it a default value.
			 */
			}
					
			part frontAxleAssembly: AxleAssembly {
				part frontAxle: Axle;			
				part frontWheel: Wheel[2] ordered;
			}		
			part rearAxleAssembly: AxleAssembly {
				part rearAxle: Axle;
				part rearWheel: Wheel[2] ordered;
			}		
		}
	
		part vehicle1_c1 :> vehicle1 {
			/*
			 * 'vehicle1_c1' is a specialization of 'vehicle1' (technically 
			 * a subset). It inherits all the parts of 'vehicle1' and
			 * only needs to specify additional or redefined parts.
			 */
		
			attribute mass redefines vehicle1::mass = 2000 [kg] {
				/*
				 * The mass is further redefined to override the default value
				 * with a bound value for 'vehicle_c1'.
				 */
			}
					
			part frontAxleAssembly_c1 redefines frontAxleAssembly {
				part frontAxle_c1: FrontAxle redefines frontAxle {
					/*
					 * 'frontAxle_c1' redefines 'frontAxleAssembly'::'frontAxle'
					 * to give it a new name and the specialized type
					 * 'FrontAxle'.
					 */
				}
				
				/*
				 * 'frontWheel' is inherited from 'vehicle1'::'frontAxleAssembly',
				 * allowing it to be used in the following part declarations.
				 */
				
				part frontWheel_1 subsets frontWheel = frontWheel#(1);
				part frontWheel_2 subsets frontWheel = frontWheel#(2);
			}
				
			part rearAxleAssembly_c1 redefines rearAxleAssembly {
				part rearAxle_c1 redefines rearAxle {
					/*
					 * 'rearAxle_c1' redefines 'rearAxleAssembly'::'rearAxle'
					 * to give it a new name. It inherits the type 'Axle'
					 * from the redefined part.
					 */
				}
						
				part rearWheel_1 subsets rearWheel = rearWheel#(1);
				part rearWheel_2 subsets rearWheel = rearWheel#(2);
			}		
		}
		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "1c_parts_tree_redefinition.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 21) (end 5 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 21) (end 9 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 3) (end 12 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 28) (end 12 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 17) (end 18 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 17) (end 20 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 28) (end 21 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 27) (end 28 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 20) (end 29 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 21) (end 30 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 26) (end 32 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 19) (end 33 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 20) (end 34 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 52 39) (end 52 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 23) (end 53 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 53 43) (end 53 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 66 30) (end 66 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 67 30) (end 67 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 70 38) (end 70 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 71 31) (end 71 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 79 29) (end 79 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 80 29) (end 80 38))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package '1c-Parts Tree Redefinition' {
    private import SI::kg;

    package Definitions {
        part def Vehicle {
            attribute mass :> ISQ::mass;
        }
        part def AxleAssembly;
        part def Axle {
            attribute mass :> ISQ::mass;
        }
        part def FrontAxle :> Axle {
            attribute steeringAngle: ScalarValues::Real;
        }
        part def Wheel;
    }

    package Usages {
        private import Definitions::*;

        part vehicle1: Vehicle {
            attribute mass redefines Vehicle::mass default = 1750 [kg] {
                doc
                /*
			 * The mass attribute is redefined to give it a default value.
			 */
            }

            part frontAxleAssembly: AxleAssembly {
                part frontAxle: Axle;
                part frontWheel: Wheel[2] ordered;
            }
            part rearAxleAssembly: AxleAssembly {
                part rearAxle: Axle;
                part rearWheel: Wheel[2] ordered;
            }
        }

        part vehicle1_c1 :> vehicle1 {
            /*
			 * 'vehicle1_c1' is a specialization of 'vehicle1' (technically 
			 * a subset). It inherits all the parts of 'vehicle1' and
			 * only needs to specify additional or redefined parts.
			 */

            attribute mass redefines vehicle1::mass = 2000 [kg] {
                /*
				 * The mass is further redefined to override the default value
				 * with a bound value for 'vehicle_c1'.
				 */
            }

            part frontAxleAssembly_c1 redefines frontAxleAssembly {
                part frontAxle_c1: FrontAxle redefines frontAxle {
                    /*
					 * 'frontAxle_c1' redefines 'frontAxleAssembly'::'frontAxle'
					 * to give it a new name and the specialized type
					 * 'FrontAxle'.
					 */
                }

                /*
				 * 'frontWheel' is inherited from 'vehicle1'::'frontAxleAssembly',
				 * allowing it to be used in the following part declarations.
				 */

                part frontWheel_1 subsets frontWheel = frontWheel#(1);
                part frontWheel_2 subsets frontWheel = frontWheel#(2);
            }

            part rearAxleAssembly_c1 redefines rearAxleAssembly {
                part rearAxle_c1 redefines rearAxle {
                    /*
					 * 'rearAxle_c1' redefines 'rearAxleAssembly'::'rearAxle'
					 * to give it a new name. It inherits the type 'Axle'
					 * from the redefined part.
					 */
                }

                part rearWheel_1 subsets rearWheel = rearWheel#(1);
                part rearWheel_2 subsets rearWheel = rearWheel#(2);
            }
        }

    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "9c07d89aede5983be29fe44a43994b63897a488d73330f5877851a8902e486df") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition"))) (kind "package") (name "1c-Parts Tree Redefinition") (declared-name "1c-Parts Tree Redefinition"))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition"))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle"))) (kind "part def") (name "Axle") (declared-name "Axle") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::AxleAssembly"))) (kind "part def") (name "AxleAssembly") (declared-name "AxleAssembly") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))) (kind "part def") (name "FrontAxle") (declared-name "FrontAxle") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Axle")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle::steeringAngle"))) (kind "attribute") (name "steeringAngle") (declared-name "steeringAngle") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (typing (reference "ScalarValues::Real")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition"))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (kind "part") (name "vehicle1") (declared-name "vehicle1") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly"))) (kind "part") (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (kind "part") (name "frontAxle") (declared-name "frontAxle") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Axle")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (kind "part") (name "frontWheel") (declared-name "frontWheel") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Vehicle::mass")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (kind "part") (name "rearAxle") (declared-name "rearAxle") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Axle")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (kind "part") (name "rearWheel") (declared-name "rearWheel") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle1")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1"))) (kind "part") (name "frontAxleAssembly_c1") (declared-name "frontAxleAssembly_c1") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "frontAxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))) (kind "part") (name "frontAxle_c1") (declared-name "frontAxle_c1") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "FrontAxle")) (redefinition (reference "frontAxle")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_1"))) (kind "part") (name "frontWheel_1") (declared-name "frontWheel_1") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "frontWheel")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_2"))) (kind "part") (name "frontWheel_2") (declared-name "frontWheel_2") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "frontWheel")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "vehicle1::mass")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1"))) (kind "part") (name "rearAxleAssembly_c1") (declared-name "rearAxleAssembly_c1") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rearAxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearAxle_c1"))) (kind "part") (name "rearAxle_c1") (declared-name "rearAxle_c1") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rearAxle")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_1"))) (kind "part") (name "rearWheel_1") (declared-name "rearWheel_1") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "rearWheel")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_2"))) (kind "part") (name "rearWheel_2") (declared-name "rearWheel_2") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "rearWheel")))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::kg"))) (kind "import") (name "kg") (declared-name "kg") (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::kg") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))) (kind specialization) (ordinal 0)) (authored-target "Axle") (outcome (status resolved) (target (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle")))))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle::steeringAngle"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle::steeringAngle"))) (kind featureTyping) (ordinal 1)) (authored-target "ScalarValues::Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "Axle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass"))) (kind redefinition) (ordinal 0)) (authored-target "Vehicle::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "Axle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle1") (outcome (status resolved) (target (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1")))))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1"))) (kind redefinition) (ordinal 0)) (authored-target "frontAxleAssembly") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "FrontAxle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))) (kind redefinition) (ordinal 0)) (authored-target "frontAxle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_1"))) (kind subsetting) (ordinal 0)) (authored-target "frontWheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_2"))) (kind subsetting) (ordinal 0)) (authored-target "frontWheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::mass"))) (kind redefinition) (ordinal 0)) (authored-target "vehicle1::mass") (outcome (status resolved) (target (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1"))) (kind redefinition) (ordinal 0)) (authored-target "rearAxleAssembly") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearAxle_c1"))) (kind redefinition) (ordinal 0)) (authored-target "rearAxle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_1"))) (kind subsetting) (ordinal 0)) (authored-target "rearWheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_2"))) (kind subsetting) (ordinal 0)) (authored-target "rearWheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::kg"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::kg") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))) (target (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))) (kind specialization) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (target (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::mass"))) (target (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::mass"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 11 24) (end 11 28)) (probe (position 11 24))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))
        (kind specialization) (ordinal 0) (authored-target "Axle")
        (range (start 11 24) (end 11 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle") (range (start 8 2) (end 8 54)))
        )
      )
    )
    (query (range (start 29 20) (end 29 24)) (probe (position 29 20))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontAxle"))
        (kind featureTyping) (ordinal 0) (authored-target "Axle")
        (range (start 29 20) (end 29 24))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 19) (end 33 23)) (probe (position 33 19))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearAxle"))
        (kind featureTyping) (ordinal 0) (authored-target "Axle")
        (range (start 33 19) (end 33 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 30 21) (end 30 26)) (probe (position 30 21))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 30 21) (end 30 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 34 20) (end 34 25)) (probe (position 34 20))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 34 20) (end 34 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 22)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::kg"))
        (kind membershipImport) (ordinal 0) (authored-target "SI::kg")
        (range (start 1 16) (end 1 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 17) (end 20 24)) (probe (position 20 17))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 20 17) (end 20 24))
        (outcome (status unresolved))
      )
    )
    (query (range (start 38 22) (end 38 30)) (probe (position 38 22))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle1")
        (range (start 38 22) (end 38 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1") (range (start 20 2) (end 20 419)))
        )
      )
    )
    (query (range (start 71 31) (end 71 39)) (probe (position 71 31))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearAxle_c1"))
        (kind redefinition) (ordinal 0) (authored-target "rearAxle")
        (range (start 71 31) (end 71 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 21) (end 5 30)) (probe (position 5 21))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 5 21) (end 5 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 21) (end 9 30)) (probe (position 9 21))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 9 21) (end 9 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 53 23) (end 53 32)) (probe (position 53 23))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))
        (kind featureTyping) (ordinal 0) (authored-target "FrontAxle")
        (range (start 53 23) (end 53 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 53 43) (end 53 52)) (probe (position 53 43))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))
        (kind redefinition) (ordinal 0) (authored-target "frontAxle")
        (range (start 53 43) (end 53 52))
        (outcome (status unresolved))
      )
    )
    (query (range (start 79 29) (end 79 38)) (probe (position 79 29))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_1"))
        (kind subsetting) (ordinal 0) (authored-target "rearWheel")
        (range (start 79 29) (end 79 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 80 29) (end 80 38)) (probe (position 80 29))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_2"))
        (kind subsetting) (ordinal 0) (authored-target "rearWheel")
        (range (start 80 29) (end 80 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 66 30) (end 66 40)) (probe (position 66 30))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_1"))
        (kind subsetting) (ordinal 0) (authored-target "frontWheel")
        (range (start 66 30) (end 66 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 67 30) (end 67 40)) (probe (position 67 30))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_2"))
        (kind subsetting) (ordinal 0) (authored-target "frontWheel")
        (range (start 67 30) (end 67 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 17) (end 18 28)) (probe (position 18 17))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Definitions::*")
        (range (start 18 17) (end 18 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 28 27) (end 28 39)) (probe (position 28 27))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly"))
        (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
        (range (start 28 27) (end 28 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 32 26) (end 32 38)) (probe (position 32 26))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly"))
        (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
        (range (start 32 26) (end 32 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 28) (end 21 41)) (probe (position 21 28))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass"))
        (kind redefinition) (ordinal 0) (authored-target "Vehicle::mass")
        (range (start 21 28) (end 21 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 45 28) (end 45 42)) (probe (position 45 28))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::mass"))
        (kind redefinition) (ordinal 0) (authored-target "vehicle1::mass")
        (range (start 45 28) (end 45 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass") (range (start 21 3) (end 21 154)))
        )
      )
    )
    (query (range (start 70 38) (end 70 54)) (probe (position 70 38))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1"))
        (kind redefinition) (ordinal 0) (authored-target "rearAxleAssembly")
        (range (start 70 38) (end 70 54))
        (outcome (status unresolved))
      )
    )
    (query (range (start 52 39) (end 52 56)) (probe (position 52 39))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1"))
        (kind redefinition) (ordinal 0) (authored-target "frontAxleAssembly")
        (range (start 52 39) (end 52 56))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 28) (end 12 46)) (probe (position 12 28))
      (reference
        (source (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle::steeringAngle"))
        (kind featureTyping) (ordinal 1) (authored-target "ScalarValues::Real")
        (range (start 12 28) (end 12 46))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
