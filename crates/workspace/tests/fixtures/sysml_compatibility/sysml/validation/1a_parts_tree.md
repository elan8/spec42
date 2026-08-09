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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
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
KwPrivate,KwImport,Ident,ColonColon,Star,OpenCurly,
RegularComment,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
RegularComment,
KwAttribute,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,OpenCurly,
RegularComment,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
RegularComment,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
RegularComment,
KwAttribute,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,OpenCurly,
RegularComment,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,OpenCurly,
RegularComment,
CloseCurly,
KwPart,Ident,KwSubsets,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwPart,Ident,KwSubsets,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
RegularComment,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,Semicolon,
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
  (package_def ''1a-Parts Tree''
    (import_decl private 'SI::kg')
    (package_def 'Definitions'
      (part_def 'Vehicle'
        (attribute_usage 'mass' :> 'ISQ::mass'
          (documentation)))
      (part_def 'AxleAssembly')
      (part_def 'Axle'
        (attribute_usage 'mass' :> 'ISQ::mass'))
      (part_def 'FrontAxle' :> 'Axle'
        (attribute_usage 'steeringAngle' : 'ScalarValues::Real'))
      (part_def 'Wheel'))
    (package_def 'Usages'
      (import_decl private 'Definitions::*'
        (comment))
      (part_usage 'vehicle1' : 'Vehicle'
        (comment)
        (attribute_usage 'mass' :>> 'Vehicle::mass' value
          (comment))
        (part_usage 'frontAxleAssembly' : 'AxleAssembly'
          (comment)
          (part_usage 'frontAxle' : 'Axle')
          (part_usage 'frontWheel' : 'Wheel' multiplicity ordered
            (comment)))
        (part_usage 'rearAxleAssembly' : 'AxleAssembly'
          (part_usage 'rearAxle' : 'Axle')
          (part_usage 'rearWheel' : 'Wheel' multiplicity ordered)))
      (part_usage 'vehicle1_c1' : 'Vehicle'
        (comment)
        (attribute_usage 'mass' :>> 'Vehicle::mass' value
          (comment))
        (part_usage 'frontAxleAssembly' : 'AxleAssembly'
          (part_usage 'frontAxle' : 'FrontAxle'
            (comment))
          (part_usage 'frontWheel' : 'Wheel' multiplicity ordered
            (comment))
          (part_usage 'frontWheel_1' :> 'frontWheel' value)
          (part_usage 'frontWheel_2' :> 'frontWheel' value))
        (part_usage 'rearAxleAssembly' : 'AxleAssembly'
          (comment)
          (part_usage 'rearAxle' : 'Axle')
          (part_usage 'rearWheel' : 'Wheel' multiplicity ordered)
          (part_usage 'rearWheel_1' :> 'rearWheel' value)
          (part_usage 'rearWheel_2' :> 'rearWheel' value))))))
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
# EXPECTED
~~~
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ScalarValues::Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ScalarValues::Real'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "1a-Parts Tree"))) (name "1a-Parts Tree") (declared-name "1a-Parts Tree")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions"))) (name "Definitions") (declared-name "Definitions")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle"))) (name "Axle") (declared-name "Axle") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly"))) (name "AxleAssembly") (declared-name "AxleAssembly") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle"))) (name "FrontAxle") (declared-name "FrontAxle") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle::steeringAngle"))) (name "steeringAngle") (declared-name "steeringAngle") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages"))) (name "Usages") (declared-name "Usages")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::*"))) (name "*") (declared-name "*"))
            (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1"))) (name "vehicle1") (declared-name "vehicle1") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly"))) (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (name "frontAxle") (declared-name "frontAxle") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (name "frontWheel") (declared-name "frontWheel") (declared (properties (ordered true)) (multiplicity (lower 2) (upper 2) (ordered true) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
                  )
                )
                (element (kind "attribute") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 1750)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::mass"))) (role feature-value))))
                (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly"))) (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (name "rearAxle") (declared-name "rearAxle") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (name "rearWheel") (declared-name "rearWheel") (declared (properties (ordered true)) (multiplicity (lower 2) (upper 2) (ordered true) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1"))) (name "vehicle1_c1") (declared-name "vehicle1_c1") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly"))) (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontAxle"))) (name "frontAxle") (declared-name "frontAxle") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))) (name "frontWheel") (declared-name "frontWheel") (declared (properties (ordered true)) (multiplicity (lower 2) (upper 2) (ordered true) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_1"))) (name "frontWheel_1") (declared-name "frontWheel_1") (declared (properties (ordered false))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_2"))) (name "frontWheel_2") (declared-name "frontWheel_2") (declared (properties (ordered false))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
                  )
                )
                (element (kind "attribute") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 2000)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::mass"))) (role feature-value))))
                (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly"))) (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (name "rearAxle") (declared-name "rearAxle") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (name "rearWheel") (declared-name "rearWheel") (declared (properties (ordered true)) (multiplicity (lower 2) (upper 2) (ordered true) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_1"))) (name "rearWheel_1") (declared-name "rearWheel_1") (declared (properties (ordered false))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_2"))) (name "rearWheel_2") (declared-name "rearWheel_2") (declared (properties (ordered false))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
                  )
                )
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "1a-Parts Tree::kg"))) (name "kg") (declared-name "kg"))
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::mass"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle::mass"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::mass"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle::mass"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_1"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_2"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_1"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_2"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Wheel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Wheel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontAxle"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Wheel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (to (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Wheel"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Axle::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::FrontAxle::steeringAngle"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Vehicle::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Definitions::Wheel"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontAxle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_2"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_2"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/1a_parts_tree.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 3) (end 20 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 26 17) (end 26 28))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 38 3) (end 38 180))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 38 3) (end 38 180))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 81 3) (end 81 120))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 81 3) (end 81 120))
      )
    )
  )
)
~~~
