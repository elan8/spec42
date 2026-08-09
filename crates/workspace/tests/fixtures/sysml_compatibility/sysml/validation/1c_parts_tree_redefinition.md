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
            attribute steeringAngle : ScalarValues::Real;
        }
        part def Wheel;
    }

    package Usages {
        private import Definitions::*;

        part vehicle1 : Vehicle {
            attribute mass redefines Vehicle::mass default = 1750 [kg] {
                doc /*
			 * The mass attribute is redefined to give it a default value.
			 */
            }

            part frontAxleAssembly : AxleAssembly {
                part frontAxle : Axle;
                part frontWheel : Wheel [2] ordered;
            }
            part rearAxleAssembly : AxleAssembly {
                part rearAxle : Axle;
                part rearWheel : Wheel [2] ordered;
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
                part frontAxle_c1 : FrontAxle redefines frontAxle {
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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition"))) (name "1c-Parts Tree Redefinition") (declared-name "1c-Parts Tree Redefinition")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions"))) (name "Definitions") (declared-name "Definitions")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle"))) (name "Axle") (declared-name "Axle") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::AxleAssembly"))) (name "AxleAssembly") (declared-name "AxleAssembly") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))) (name "FrontAxle") (declared-name "FrontAxle") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle::steeringAngle"))) (name "steeringAngle") (declared-name "steeringAngle") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages"))) (name "Usages") (declared-name "Usages")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::*"))) (name "*") (declared-name "*"))
            (element (kind "part") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (name "vehicle1") (declared-name "vehicle1") (declared (properties (composite true) (reference false) (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly"))) (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (name "frontAxle") (declared-name "frontAxle") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::AxleAssembly")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (name "frontWheel") (declared-name "frontWheel") (declared (properties (composite true) (reference false) (ordered true)) (multiplicity (lower 2) (upper 2) (ordered true) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::AxleAssembly")))))
                  )
                )
                (element (kind "attribute") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind default) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 1750)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly"))) (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (name "rearAxle") (declared-name "rearAxle") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::AxleAssembly")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (name "rearWheel") (declared-name "rearWheel") (declared (properties (composite true) (reference false) (ordered true)) (multiplicity (lower 2) (upper 2) (ordered true) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::AxleAssembly")))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (name "vehicle1_c1") (declared-name "vehicle1_c1") (declared (properties (composite true) (reference false) (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1"))) (name "frontAxleAssembly_c1") (declared-name "frontAxleAssembly_c1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))) (name "frontAxle_c1") (declared-name "frontAxle_c1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_1"))) (name "frontWheel_1") (declared-name "frontWheel_1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_2"))) (name "frontWheel_2") (declared-name "frontWheel_2") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
                (element (kind "attribute") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 2000)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::mass"))) (role feature-value))))
                (element (kind "part") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1"))) (name "rearAxleAssembly_c1") (declared-name "rearAxleAssembly_c1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearAxle_c1"))) (name "rearAxle_c1") (declared-name "rearAxle_c1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_1"))) (name "rearWheel_1") (declared-name "rearWheel_1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_2"))) (name "rearWheel_2") (declared-name "rearWheel_2") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::kg"))) (name "kg") (declared-name "kg"))
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass"))) (to (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle::mass"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))) (to (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (to (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (to (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly"))) (to (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::AxleAssembly"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (to (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (to (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly"))) (to (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::AxleAssembly"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (to (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (to (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))) (to (node (document "d0") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
