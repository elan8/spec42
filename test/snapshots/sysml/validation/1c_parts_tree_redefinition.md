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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,KwRedefines,Ident,ColonColon,Ident,KwDefault,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
RegularComment,
KwAttribute,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,OpenCurly,
RegularComment,
CloseCurly,
KwPart,Ident,KwRedefines,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,KwRedefines,Ident,OpenCurly,
RegularComment,
CloseCurly,
RegularComment,
KwPart,Ident,KwSubsets,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwPart,Ident,KwSubsets,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
CloseCurly,
KwPart,Ident,KwRedefines,Ident,OpenCurly,
KwPart,Ident,KwRedefines,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwPart,Ident,KwSubsets,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwPart,Ident,KwSubsets,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''1c-Parts Tree Redefinition''
    (import_decl private 'SI::kg')
    (package_def 'Definitions'
      (part_def 'Vehicle'
        (attribute_usage 'mass' :> 'ISQ::mass'))
      (part_def 'AxleAssembly')
      (part_def 'Axle'
        (attribute_usage 'mass' :> 'ISQ::mass'))
      (part_def 'FrontAxle' :> 'Axle'
        (attribute_usage 'steeringAngle' : 'ScalarValues::Real'))
      (part_def 'Wheel'))
    (package_def 'Usages'
      (import_decl private 'Definitions::*')
      (part_usage 'vehicle1' : 'Vehicle'
        (attribute_usage 'mass' :>> 'Vehicle::mass' value
          (documentation))
        (part_usage 'frontAxleAssembly' : 'AxleAssembly'
          (part_usage 'frontAxle' : 'Axle')
          (part_usage 'frontWheel' : 'Wheel' multiplicity ordered))
        (part_usage 'rearAxleAssembly' : 'AxleAssembly'
          (part_usage 'rearAxle' : 'Axle')
          (part_usage 'rearWheel' : 'Wheel' multiplicity ordered)))
      (part_usage 'vehicle1_c1' :> 'vehicle1'
        (comment)
        (attribute_usage 'mass' :>> 'vehicle1::mass' value
          (comment))
        (part_usage 'frontAxleAssembly_c1' :>> 'frontAxleAssembly'
          (part_usage 'frontAxle_c1' : 'FrontAxle' :>> 'frontAxle'
            (comment))
          (comment)
          (part_usage 'frontWheel_1' :> 'frontWheel' value)
          (part_usage 'frontWheel_2' :> 'frontWheel' value))
        (part_usage 'rearAxleAssembly_c1' :>> 'rearAxleAssembly'
          (part_usage 'rearAxle_c1' :>> 'rearAxle'
            (comment))
          (part_usage 'rearWheel_1' :> 'rearWheel' value)
          (part_usage 'rearWheel_2' :> 'rearWheel' value))))))
~~~
# EXPECTED
~~~
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ScalarValues::Real'
~~~
# PROBLEMS
~~~
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ScalarValues::Real'
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
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition"))) (kind "package") (name "1c-Parts Tree Redefinition") (declared-name "1c-Parts Tree Redefinition") (range (start (line 0) (character 0)) (end (line 0) (character 2231))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (range (start (line 3) (character 1)) (end (line 3) (character 272))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition"))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle"))) (kind "part def") (name "Axle") (declared-name "Axle") (range (start (line 8) (character 2)) (end (line 8) (character 54))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 9) (character 3)) (end (line 9) (character 31))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 9) (character 21)) (end (line 9) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::AxleAssembly"))) (kind "part def") (name "AxleAssembly") (declared-name "AxleAssembly") (range (start (line 7) (character 2)) (end (line 7) (character 24))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))) (kind "part def") (name "FrontAxle") (declared-name "FrontAxle") (range (start (line 11) (character 2)) (end (line 11) (character 83))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Axle") (range (start (line 11) (character 24)) (end (line 11) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle::steeringAngle"))) (kind "attribute") (name "steeringAngle") (declared-name "steeringAngle") (range (start (line 12) (character 3)) (end (line 12) (character 47))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "ScalarValues::Real") (range (start (line 12) (character 28)) (end (line 12) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 4) (character 2)) (end (line 4) (character 56))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 5) (character 3)) (end (line 5) (character 31))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 5) (character 21)) (end (line 5) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (range (start (line 14) (character 2)) (end (line 14) (character 17))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (range (start (line 17) (character 1)) (end (line 17) (character 1888))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition"))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 18) (character 2)) (end (line 18) (character 32))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 18) (character 17)) (end (line 18) (character 28))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (kind "part") (name "vehicle1") (declared-name "vehicle1") (range (start (line 20) (character 2)) (end (line 20) (character 419))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 20) (character 17)) (end (line 20) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly"))) (kind "part") (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (range (start (line 28) (character 3)) (end (line 28) (character 114))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly") (range (start (line 28) (character 27)) (end (line 28) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (kind "part") (name "frontAxle") (declared-name "frontAxle") (range (start (line 29) (character 4)) (end (line 29) (character 25))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Axle") (range (start (line 29) (character 20)) (end (line 29) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (kind "part") (name "frontWheel") (declared-name "frontWheel") (range (start (line 30) (character 4)) (end (line 30) (character 38))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 30) (character 21)) (end (line 30) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 21) (character 3)) (end (line 21) (character 154))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Vehicle::mass") (range (start (line 21) (character 28)) (end (line 21) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (range (start (line 32) (character 3)) (end (line 32) (character 108))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly") (range (start (line 32) (character 26)) (end (line 32) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (kind "part") (name "rearAxle") (declared-name "rearAxle") (range (start (line 33) (character 4)) (end (line 33) (character 24))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Axle") (range (start (line 33) (character 19)) (end (line 33) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (kind "part") (name "rearWheel") (declared-name "rearWheel") (range (start (line 34) (character 4)) (end (line 34) (character 37))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 34) (character 20)) (end (line 34) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (range (start (line 38) (character 2)) (end (line 38) (character 1406))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle1") (range (start (line 38) (character 22)) (end (line 38) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1"))) (kind "part") (name "frontAxleAssembly_c1") (declared-name "frontAxleAssembly_c1") (range (start (line 52) (character 3)) (end (line 52) (character 563))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "frontAxleAssembly") (range (start (line 52) (character 39)) (end (line 52) (character 56)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))) (kind "part") (name "frontAxle_c1") (declared-name "frontAxle_c1") (range (start (line 53) (character 4)) (end (line 53) (character 219))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "FrontAxle") (range (start (line 53) (character 23)) (end (line 53) (character 32)))) (redefinition (reference "frontAxle") (range (start (line 53) (character 43)) (end (line 53) (character 52)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_1"))) (kind "part") (name "frontWheel_1") (declared-name "frontWheel_1") (range (start (line 66) (character 4)) (end (line 66) (character 58))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "frontWheel") (range (start (line 66) (character 30)) (end (line 66) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_2"))) (kind "part") (name "frontWheel_2") (declared-name "frontWheel_2") (range (start (line 67) (character 4)) (end (line 67) (character 58))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "frontWheel") (range (start (line 67) (character 30)) (end (line 67) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 45) (character 3)) (end (line 45) (character 187))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "vehicle1::mass") (range (start (line 45) (character 28)) (end (line 45) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1"))) (kind "part") (name "rearAxleAssembly_c1") (declared-name "rearAxleAssembly_c1") (range (start (line 70) (character 3)) (end (line 70) (character 400))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rearAxleAssembly") (range (start (line 70) (character 38)) (end (line 70) (character 54)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearAxle_c1"))) (kind "part") (name "rearAxle_c1") (declared-name "rearAxle_c1") (range (start (line 71) (character 4)) (end (line 71) (character 219))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rearAxle") (range (start (line 71) (character 31)) (end (line 71) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_1"))) (kind "part") (name "rearWheel_1") (declared-name "rearWheel_1") (range (start (line 79) (character 4)) (end (line 79) (character 55))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "rearWheel") (range (start (line 79) (character 29)) (end (line 79) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_2"))) (kind "part") (name "rearWheel_2") (declared-name "rearWheel_2") (range (start (line 80) (character 4)) (end (line 80) (character 55))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "rearWheel") (range (start (line 80) (character 29)) (end (line 80) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::kg"))) (kind "import") (name "kg") (declared-name "kg") (range (start (line 1) (character 1)) (end (line 1) (character 23))) (parent (node (document "d0") (qualified-name "1c-Parts Tree Redefinition"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::kg") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 22))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 9) (character 21)) (end (line 9) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))) (kind specialization) (ordinal 0)) (authored-target "Axle") (range (start (line 11) (character 24)) (end (line 11) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle")))))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle::steeringAngle"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle::steeringAngle"))) (kind featureTyping) (ordinal 1)) (authored-target "ScalarValues::Real") (range (start (line 12) (character 28)) (end (line 12) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 5) (character 21)) (end (line 5) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (range (start (line 18) (character 17)) (end (line 18) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 20) (character 17)) (end (line 20) (character 24))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly") (range (start (line 28) (character 27)) (end (line 28) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "Axle") (range (start (line 29) (character 20)) (end (line 29) (character 24))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 30) (character 21)) (end (line 30) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass"))) (kind redefinition) (ordinal 0)) (authored-target "Vehicle::mass") (range (start (line 21) (character 28)) (end (line 21) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly") (range (start (line 32) (character 26)) (end (line 32) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "Axle") (range (start (line 33) (character 19)) (end (line 33) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 34) (character 20)) (end (line 34) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle1") (range (start (line 38) (character 22)) (end (line 38) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1")))))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1"))) (kind redefinition) (ordinal 0)) (authored-target "frontAxleAssembly") (range (start (line 52) (character 39)) (end (line 52) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "FrontAxle") (range (start (line 53) (character 23)) (end (line 53) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))) (kind redefinition) (ordinal 0)) (authored-target "frontAxle") (range (start (line 53) (character 43)) (end (line 53) (character 52))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_1"))) (kind subsetting) (ordinal 0)) (authored-target "frontWheel") (range (start (line 66) (character 30)) (end (line 66) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_2"))) (kind subsetting) (ordinal 0)) (authored-target "frontWheel") (range (start (line 67) (character 30)) (end (line 67) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::mass"))) (kind redefinition) (ordinal 0)) (authored-target "vehicle1::mass") (range (start (line 45) (character 28)) (end (line 45) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1"))) (kind redefinition) (ordinal 0)) (authored-target "rearAxleAssembly") (range (start (line 70) (character 38)) (end (line 70) (character 54))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearAxle_c1"))) (kind redefinition) (ordinal 0)) (authored-target "rearAxle") (range (start (line 71) (character 31)) (end (line 71) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_1"))) (kind subsetting) (ordinal 0)) (authored-target "rearWheel") (range (start (line 79) (character 29)) (end (line 79) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_2"))) (kind subsetting) (ordinal 0)) (authored-target "rearWheel") (range (start (line 80) (character 29)) (end (line 80) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::kg"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::kg") (range (start (line 1) (character 16)) (end (line 1) (character 22))) (outcome (status unresolved)))
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
