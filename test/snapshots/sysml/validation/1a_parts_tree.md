# META
~~~ini
description=SysML Validation (01-Parts Tree): 1a-Parts Tree
type=file
~~~
# SOURCE
~~~sysml
package '1a-Parts Tree' {
	private import SI::kg;
	
	package Definitions {	
		part def Vehicle {
			attribute mass :> ISQ::mass {
			doc
			/*
			 * The 'mass' attribute property is declared here to be a 
			 * specialization (subset) of the general 'mass' quantity 
			 * from the 'ISQ' (International System of Quantities) 
			 * library model.
			 */
			}
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
		private import Definitions::* {
			/*
			 * A "private" private import makes the imported names private to the
			 * imported package.
			 */
		}
	
		part vehicle1: Vehicle {
			/*
			 * 'vehicle1' is a package-owned part of type Vehicle.
			 */
			 
			attribute mass redefines Vehicle::mass = 1750 [kg] {
				/*
				 * This redefines the 'mass' attribute property from 'Vehicle' to 
				 * give it a fixed attribute.
				 */
			}
			
			part frontAxleAssembly: AxleAssembly {
				/*
				 * 'frontAxleAssembly' is a nested part of part 'vehicle1'.
				 * It is a composite part of the containing part.
				 * 
				 * (And similarly for 'rearAxleAssembly'.)
				 */
			
				part frontAxle: Axle;
				
				part frontWheel: Wheel[2] ordered {
					/*
					 * 'frontWheel' is a nested part of type 'Wheel' with
					 * multiplicity "2". This means that this axle assembly
					 * must have exactly two wheels. However, there is still
					 * only one 'frontWheel' part. The part is "ordered",
					 * so that the first wheel can be distinguished from the
					 * second.
					 */
				}
			}
			
			part rearAxleAssembly: AxleAssembly {
				part rearAxle: Axle;
				part rearWheel: Wheel[2] ordered;
			}
			
		}
	
		part vehicle1_c1: Vehicle {
			/*
			 * 'vehicle1_c1' is a modified copy of 'vehicle1'. There is no
			 * connection between this copy and the original version in the
			 * model.
			 */			
			
			attribute mass redefines Vehicle::mass = 2000 [kg] {
				/*
				 * The mass attribute has been modified.
				 */
			}
	
			part frontAxleAssembly: AxleAssembly {
				
				part frontAxle: FrontAxle {
					/*
					 * The part 'frontAxle' has been modified to have type 'FrontAxle'.
					 */
				}
				
				part frontWheel: Wheel[2] ordered {
					/*
					 * The parts 'frontWheel_1' and 'frontWheel_2' have been added
					 * as subsets of 'frontWheel'. These are separate parts from
					 * 'frontWheel', but essentially provide alternate names for
					 * each of the two wheels, as given by their defining expressions.
					 */
				}
				part frontWheel_1 subsets frontWheel = frontWheel#(1);
				part frontWheel_2 subsets frontWheel = frontWheel#(2);
			}
			
			part rearAxleAssembly: AxleAssembly {
				/*
				 * 'rearAxleAssembly' has also been modified to add subsetting parts
				 * for 'rearWheel'.
				 */
						
				part rearAxle: Axle;
				
				part rearWheel: Wheel[2] ordered;
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
  (document "1a_parts_tree.md"
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
        (range (start 17 21) (end 17 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 3) (end 20 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 28) (end 20 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 26 17) (end 26 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 17) (end 33 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 38 28) (end 38 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 45 27) (end 45 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 20) (end 53 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 21) (end 55 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 26) (end 67 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 68 19) (end 68 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 20) (end 69 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 74 20) (end 74 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 81 28) (end 81 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 87 27) (end 87 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 89 20) (end 89 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 95 21) (end 95 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 107 26) (end 107 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 19) (end 113 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 115 20) (end 115 25))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package '1a-Parts Tree' {
    private import SI::kg;

    package Definitions {
        part def Vehicle {
            attribute mass :> ISQ::mass {
                doc
                /*
			 * The 'mass' attribute property is declared here to be a 
			 * specialization (subset) of the general 'mass' quantity 
			 * from the 'ISQ' (International System of Quantities) 
			 * library model.
			 */
            }
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
        private import Definitions::* {
            /*
			 * A "private" private import makes the imported names private to the
			 * imported package.
			 */
        }

        part vehicle1: Vehicle {
            /*
			 * 'vehicle1' is a package-owned part of type Vehicle.
			 */

            attribute mass redefines Vehicle::mass = 1750 [kg] {
                /*
				 * This redefines the 'mass' attribute property from 'Vehicle' to 
				 * give it a fixed attribute.
				 */
            }

            part frontAxleAssembly: AxleAssembly {
                /*
				 * 'frontAxleAssembly' is a nested part of part 'vehicle1'.
				 * It is a composite part of the containing part.
				 * 
				 * (And similarly for 'rearAxleAssembly'.)
				 */

                part frontAxle: Axle;

                part frontWheel: Wheel[2] ordered {
                    /*
					 * 'frontWheel' is a nested part of type 'Wheel' with
					 * multiplicity "2". This means that this axle assembly
					 * must have exactly two wheels. However, there is still
					 * only one 'frontWheel' part. The part is "ordered",
					 * so that the first wheel can be distinguished from the
					 * second.
					 */
                }
            }

            part rearAxleAssembly: AxleAssembly {
                part rearAxle: Axle;
                part rearWheel: Wheel[2] ordered;
            }

        }

        part vehicle1_c1: Vehicle {
            /*
			 * 'vehicle1_c1' is a modified copy of 'vehicle1'. There is no
			 * connection between this copy and the original version in the
			 * model.
			 */			

            attribute mass redefines Vehicle::mass = 2000 [kg] {
                /*
				 * The mass attribute has been modified.
				 */
            }

            part frontAxleAssembly: AxleAssembly {

                part frontAxle: FrontAxle {
                    /*
					 * The part 'frontAxle' has been modified to have type 'FrontAxle'.
					 */
                }

                part frontWheel: Wheel[2] ordered {
                    /*
					 * The parts 'frontWheel_1' and 'frontWheel_2' have been added
					 * as subsets of 'frontWheel'. These are separate parts from
					 * 'frontWheel', but essentially provide alternate names for
					 * each of the two wheels, as given by their defining expressions.
					 */
                }
                part frontWheel_1 subsets frontWheel = frontWheel#(1);
                part frontWheel_2 subsets frontWheel = frontWheel#(2);
            }

            part rearAxleAssembly: AxleAssembly {
                /*
				 * 'rearAxleAssembly' has also been modified to add subsetting parts
				 * for 'rearWheel'.
				 */

                part rearAxle: Axle;

                part rearWheel: Wheel[2] ordered;
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "df89070b07d5fb10d9425e31b276f36215876f47099c2c46e3d3ec0cfa9bac19") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree"))) (kind "package") (name "1a-Parts Tree") (declared-name "1a-Parts Tree") (range (start (line 0) (character 0)) (end (line 0) (character 3110))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (range (start (line 3) (character 1)) (end (line 3) (character 502))) (parent (node (document "d0") (qualified-name "1a-Parts Tree"))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle"))) (kind "part def") (name "Axle") (declared-name "Axle") (range (start (line 16) (character 2)) (end (line 16) (character 54))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 17) (character 3)) (end (line 17) (character 31))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 17) (character 21)) (end (line 17) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly"))) (kind "part def") (name "AxleAssembly") (declared-name "AxleAssembly") (range (start (line 15) (character 2)) (end (line 15) (character 24))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle"))) (kind "part def") (name "FrontAxle") (declared-name "FrontAxle") (range (start (line 19) (character 2)) (end (line 19) (character 83))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Axle") (range (start (line 19) (character 24)) (end (line 19) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle::steeringAngle"))) (kind "attribute") (name "steeringAngle") (declared-name "steeringAngle") (range (start (line 20) (character 3)) (end (line 20) (character 47))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "ScalarValues::Real") (range (start (line 20) (character 28)) (end (line 20) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 4) (character 2)) (end (line 4) (character 286))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 5) (character 3)) (end (line 5) (character 261))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 5) (character 21)) (end (line 5) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (range (start (line 22) (character 2)) (end (line 22) (character 17))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (range (start (line 25) (character 1)) (end (line 25) (character 2549))) (parent (node (document "d0") (qualified-name "1a-Parts Tree"))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 26) (character 2)) (end (line 26) (character 147))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 26) (character 17)) (end (line 26) (character 28))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1"))) (kind "part") (name "vehicle1") (declared-name "vehicle1") (range (start (line 33) (character 2)) (end (line 33) (character 1060))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 33) (character 17)) (end (line 33) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly"))) (kind "part") (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (range (start (line 45) (character 3)) (end (line 45) (character 651))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly") (range (start (line 45) (character 27)) (end (line 45) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (kind "part") (name "frontAxle") (declared-name "frontAxle") (range (start (line 53) (character 4)) (end (line 53) (character 25))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Axle") (range (start (line 53) (character 20)) (end (line 53) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (kind "part") (name "frontWheel") (declared-name "frontWheel") (range (start (line 55) (character 4)) (end (line 55) (character 381))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 55) (character 21)) (end (line 55) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 38) (character 3)) (end (line 38) (character 180))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Vehicle::mass") (range (start (line 38) (character 28)) (end (line 38) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (range (start (line 67) (character 3)) (end (line 67) (character 108))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly") (range (start (line 67) (character 26)) (end (line 67) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (kind "part") (name "rearAxle") (declared-name "rearAxle") (range (start (line 68) (character 4)) (end (line 68) (character 24))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Axle") (range (start (line 68) (character 19)) (end (line 68) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (kind "part") (name "rearWheel") (declared-name "rearWheel") (range (start (line 69) (character 4)) (end (line 69) (character 37))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 69) (character 20)) (end (line 69) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (range (start (line 74) (character 2)) (end (line 74) (character 1313))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 74) (character 20)) (end (line 74) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly"))) (kind "part") (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (range (start (line 87) (character 3)) (end (line 87) (character 637))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly") (range (start (line 87) (character 27)) (end (line 87) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontAxle"))) (kind "part") (name "frontAxle") (declared-name "frontAxle") (range (start (line 89) (character 4)) (end (line 89) (character 127))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "FrontAxle") (range (start (line 89) (character 20)) (end (line 89) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))) (kind "part") (name "frontWheel") (declared-name "frontWheel") (range (start (line 95) (character 4)) (end (line 95) (character 334))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 95) (character 21)) (end (line 95) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_1"))) (kind "part") (name "frontWheel_1") (declared-name "frontWheel_1") (range (start (line 103) (character 4)) (end (line 103) (character 58))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "frontWheel") (range (start (line 103) (character 30)) (end (line 103) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_2"))) (kind "part") (name "frontWheel_2") (declared-name "frontWheel_2") (range (start (line 104) (character 4)) (end (line 104) (character 58))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "frontWheel") (range (start (line 104) (character 30)) (end (line 104) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 81) (character 3)) (end (line 81) (character 120))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Vehicle::mass") (range (start (line 81) (character 28)) (end (line 81) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (range (start (line 107) (character 3)) (end (line 107) (character 344))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly") (range (start (line 107) (character 26)) (end (line 107) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (kind "part") (name "rearAxle") (declared-name "rearAxle") (range (start (line 113) (character 4)) (end (line 113) (character 24))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Axle") (range (start (line 113) (character 19)) (end (line 113) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (kind "part") (name "rearWheel") (declared-name "rearWheel") (range (start (line 115) (character 4)) (end (line 115) (character 37))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 115) (character 20)) (end (line 115) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_1"))) (kind "part") (name "rearWheel_1") (declared-name "rearWheel_1") (range (start (line 116) (character 4)) (end (line 116) (character 55))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "rearWheel") (range (start (line 116) (character 29)) (end (line 116) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_2"))) (kind "part") (name "rearWheel_2") (declared-name "rearWheel_2") (range (start (line 117) (character 4)) (end (line 117) (character 55))) (parent (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "rearWheel") (range (start (line 117) (character 29)) (end (line 117) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "1a-Parts Tree::kg"))) (kind "import") (name "kg") (declared-name "kg") (range (start (line 1) (character 1)) (end (line 1) (character 23))) (parent (node (document "d0") (qualified-name "1a-Parts Tree"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::kg") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 22))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 17) (character 21)) (end (line 17) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle"))) (kind specialization) (ordinal 0)) (authored-target "Axle") (range (start (line 19) (character 24)) (end (line 19) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle")))))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle::steeringAngle"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle::steeringAngle"))) (kind featureTyping) (ordinal 1)) (authored-target "ScalarValues::Real") (range (start (line 20) (character 28)) (end (line 20) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 5) (character 21)) (end (line 5) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (range (start (line 26) (character 17)) (end (line 26) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 33) (character 17)) (end (line 33) (character 24))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly") (range (start (line 45) (character 27)) (end (line 45) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "Axle") (range (start (line 53) (character 20)) (end (line 53) (character 24))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 55) (character 21)) (end (line 55) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::mass"))) (kind redefinition) (ordinal 0)) (authored-target "Vehicle::mass") (range (start (line 38) (character 28)) (end (line 38) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly") (range (start (line 67) (character 26)) (end (line 67) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "Axle") (range (start (line 68) (character 19)) (end (line 68) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 69) (character 20)) (end (line 69) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 74) (character 20)) (end (line 74) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly") (range (start (line 87) (character 27)) (end (line 87) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "FrontAxle") (range (start (line 89) (character 20)) (end (line 89) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 95) (character 21)) (end (line 95) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_1"))) (kind subsetting) (ordinal 0)) (authored-target "frontWheel") (range (start (line 103) (character 30)) (end (line 103) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel")))))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_2"))) (kind subsetting) (ordinal 0)) (authored-target "frontWheel") (range (start (line 104) (character 30)) (end (line 104) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel")))))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::mass"))) (kind redefinition) (ordinal 0)) (authored-target "Vehicle::mass") (range (start (line 81) (character 28)) (end (line 81) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly") (range (start (line 107) (character 26)) (end (line 107) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "Axle") (range (start (line 113) (character 19)) (end (line 113) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 115) (character 20)) (end (line 115) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_1"))) (kind subsetting) (ordinal 0)) (authored-target "rearWheel") (range (start (line 116) (character 29)) (end (line 116) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel")))))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_2"))) (kind subsetting) (ordinal 0)) (authored-target "rearWheel") (range (start (line 117) (character 29)) (end (line 117) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel")))))
    (reference (id (source (node (document "d0") (qualified-name "1a-Parts Tree::kg"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::kg") (range (start (line 1) (character 16)) (end (line 1) (character 22))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle"))) (target (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle"))) (kind specialization) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_1"))) (target (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_1"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_2"))) (target (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_2"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_1"))) (target (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_1"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_2"))) (target (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_2"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 19 24) (end 19 28)) (probe (position 19 24))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle"))
        (kind specialization) (ordinal 0) (authored-target "Axle")
        (range (start 19 24) (end 19 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle") (range (start 16 2) (end 16 54)))
        )
      )
    )
    (query (range (start 53 20) (end 53 24)) (probe (position 53 20))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontAxle"))
        (kind featureTyping) (ordinal 0) (authored-target "Axle")
        (range (start 53 20) (end 53 24))
        (outcome (status unresolved))
      )
    )
    (query (range (start 68 19) (end 68 23)) (probe (position 68 19))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearAxle"))
        (kind featureTyping) (ordinal 0) (authored-target "Axle")
        (range (start 68 19) (end 68 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 113 19) (end 113 23)) (probe (position 113 19))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))
        (kind featureTyping) (ordinal 0) (authored-target "Axle")
        (range (start 113 19) (end 113 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 55 21) (end 55 26)) (probe (position 55 21))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 55 21) (end 55 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 69 20) (end 69 25)) (probe (position 69 20))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 69 20) (end 69 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 95 21) (end 95 26)) (probe (position 95 21))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 95 21) (end 95 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 115 20) (end 115 25)) (probe (position 115 20))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 115 20) (end 115 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 22)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::kg"))
        (kind membershipImport) (ordinal 0) (authored-target "SI::kg")
        (range (start 1 16) (end 1 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 17) (end 33 24)) (probe (position 33 17))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 33 17) (end 33 24))
        (outcome (status unresolved))
      )
    )
    (query (range (start 74 20) (end 74 27)) (probe (position 74 20))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 74 20) (end 74 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 21) (end 5 30)) (probe (position 5 21))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 5 21) (end 5 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 21) (end 17 30)) (probe (position 17 21))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 17 21) (end 17 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 89 20) (end 89 29)) (probe (position 89 20))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontAxle"))
        (kind featureTyping) (ordinal 0) (authored-target "FrontAxle")
        (range (start 89 20) (end 89 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 116 29) (end 116 38)) (probe (position 116 29))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_1"))
        (kind subsetting) (ordinal 0) (authored-target "rearWheel")
        (range (start 116 29) (end 116 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel") (range (start 115 4) (end 115 37)))
        )
      )
    )
    (query (range (start 117 29) (end 117 38)) (probe (position 117 29))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_2"))
        (kind subsetting) (ordinal 0) (authored-target "rearWheel")
        (range (start 117 29) (end 117 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel") (range (start 115 4) (end 115 37)))
        )
      )
    )
    (query (range (start 103 30) (end 103 40)) (probe (position 103 30))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_1"))
        (kind subsetting) (ordinal 0) (authored-target "frontWheel")
        (range (start 103 30) (end 103 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel") (range (start 95 4) (end 95 334)))
        )
      )
    )
    (query (range (start 104 30) (end 104 40)) (probe (position 104 30))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_2"))
        (kind subsetting) (ordinal 0) (authored-target "frontWheel")
        (range (start 104 30) (end 104 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel") (range (start 95 4) (end 95 334)))
        )
      )
    )
    (query (range (start 26 17) (end 26 28)) (probe (position 26 17))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Definitions::*")
        (range (start 26 17) (end 26 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 45 27) (end 45 39)) (probe (position 45 27))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly"))
        (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
        (range (start 45 27) (end 45 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 67 26) (end 67 38)) (probe (position 67 26))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly"))
        (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
        (range (start 67 26) (end 67 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 87 27) (end 87 39)) (probe (position 87 27))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly"))
        (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
        (range (start 87 27) (end 87 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 107 26) (end 107 38)) (probe (position 107 26))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly"))
        (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
        (range (start 107 26) (end 107 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 38 28) (end 38 41)) (probe (position 38 28))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::mass"))
        (kind redefinition) (ordinal 0) (authored-target "Vehicle::mass")
        (range (start 38 28) (end 38 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 81 28) (end 81 41)) (probe (position 81 28))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::mass"))
        (kind redefinition) (ordinal 0) (authored-target "Vehicle::mass")
        (range (start 81 28) (end 81 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 28) (end 20 46)) (probe (position 20 28))
      (reference
        (source (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle::steeringAngle"))
        (kind featureTyping) (ordinal 1) (authored-target "ScalarValues::Real")
        (range (start 20 28) (end 20 46))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
