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
                doc /*
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
            attribute steeringAngle : ScalarValues::Real;
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

        part vehicle1 : Vehicle {
            /*
			 * 'vehicle1' is a package-owned part of type Vehicle.
			 */

            attribute mass redefines Vehicle::mass = 1750 [kg] {
                /*
				 * This redefines the 'mass' attribute property from 'Vehicle' to 
				 * give it a fixed attribute.
				 */
            }

            part frontAxleAssembly : AxleAssembly {
                /*
				 * 'frontAxleAssembly' is a nested part of part 'vehicle1'.
				 * It is a composite part of the containing part.
				 * 
				 * (And similarly for 'rearAxleAssembly'.)
				 */

                part frontAxle : Axle;

                part frontWheel : Wheel [2] ordered {
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

            part rearAxleAssembly : AxleAssembly {
                part rearAxle : Axle;
                part rearWheel : Wheel [2] ordered;
            }
        }

        part vehicle1_c1 : Vehicle {
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

            part frontAxleAssembly : AxleAssembly {
                part frontAxle : FrontAxle {
                    /*
					 * The part 'frontAxle' has been modified to have type 'FrontAxle'.
					 */
                }

                part frontWheel : Wheel [2] ordered {
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

            part rearAxleAssembly : AxleAssembly {
                /*
				 * 'rearAxleAssembly' has also been modified to add subsetting parts
				 * for 'rearWheel'.
				 */

                part rearAxle : Axle;

                part rearWheel : Wheel [2] ordered;
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
(model
  (namespace
    (package '1a-Parts Tree'
      (membership_import private -> 'SI::kg'[unresolved])
      (package 'Definitions'
        (part_def 'Vehicle'
          (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]
            (documentation)))
        (part_def 'AxleAssembly')
        (part_def 'Axle'
          (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]))
        (part_def 'FrontAxle' :> '1a-Parts Tree::Definitions::Axle'[part_def]
          (attribute_usage composite 'steeringAngle' : 'ScalarValues::Real'[unresolved]))
        (part_def 'Wheel'))
      (package 'Usages'
        (namespace_import private -> '1a-Parts Tree::Definitions'[package])
        (part_usage 'vehicle1' : '1a-Parts Tree::Definitions::Vehicle'[part_def]
          (attribute_usage composite 'mass' :>> '1a-Parts Tree::Definitions::Vehicle::mass'[attribute_usage]
            (feature_value (=)))
          (part_usage composite 'frontAxleAssembly' : '1a-Parts Tree::Definitions::AxleAssembly'[part_def]
            (part_usage composite 'frontAxle' : '1a-Parts Tree::Definitions::Axle'[part_def])
            (part_usage composite ordered 'frontWheel' : '1a-Parts Tree::Definitions::Wheel'[part_def]
              (multiplicity_range [2])))
          (part_usage composite 'rearAxleAssembly' : '1a-Parts Tree::Definitions::AxleAssembly'[part_def]
            (part_usage composite 'rearAxle' : '1a-Parts Tree::Definitions::Axle'[part_def])
            (part_usage composite ordered 'rearWheel' : '1a-Parts Tree::Definitions::Wheel'[part_def]
              (multiplicity_range [2]))))
        (part_usage 'vehicle1_c1' : '1a-Parts Tree::Definitions::Vehicle'[part_def]
          (attribute_usage composite 'mass' :>> '1a-Parts Tree::Definitions::Vehicle::mass'[attribute_usage]
            (feature_value (=)))
          (part_usage composite 'frontAxleAssembly' : '1a-Parts Tree::Definitions::AxleAssembly'[part_def]
            (part_usage composite 'frontAxle' : '1a-Parts Tree::Definitions::FrontAxle'[part_def])
            (part_usage composite ordered 'frontWheel' : '1a-Parts Tree::Definitions::Wheel'[part_def]
              (multiplicity_range [2]))
            (part_usage composite 'frontWheel_1' :> '1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel'[part_usage]
              (feature_value (=)))
            (part_usage composite 'frontWheel_2' :> '1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel'[part_usage]
              (feature_value (=))))
          (part_usage composite 'rearAxleAssembly' : '1a-Parts Tree::Definitions::AxleAssembly'[part_def]
            (part_usage composite 'rearAxle' : '1a-Parts Tree::Definitions::Axle'[part_def])
            (part_usage composite ordered 'rearWheel' : '1a-Parts Tree::Definitions::Wheel'[part_def]
              (multiplicity_range [2]))
            (part_usage composite 'rearWheel_1' :> '1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel'[part_usage]
              (feature_value (=)))
            (part_usage composite 'rearWheel_2' :> '1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel'[part_usage]
              (feature_value (=)))))))))
~~~
