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
(model
  (namespace
    (package '1c-Parts Tree Redefinition'
      (membership_import private -> 'SI::kg'[unresolved])
      (package 'Definitions'
        (part_def 'Vehicle'
          (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]))
        (part_def 'AxleAssembly')
        (part_def 'Axle'
          (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]))
        (part_def 'FrontAxle' :> '1c-Parts Tree Redefinition::Definitions::Axle'[part_def]
          (attribute_usage composite 'steeringAngle' : 'ScalarValues::Real'[unresolved]))
        (part_def 'Wheel'))
      (package 'Usages'
        (namespace_import private -> '1c-Parts Tree Redefinition::Definitions'[package])
        (part_usage 'vehicle1' : '1c-Parts Tree Redefinition::Definitions::Vehicle'[part_def]
          (attribute_usage composite 'mass' :>> '1c-Parts Tree Redefinition::Definitions::Vehicle::mass'[attribute_usage]
            (feature_value (default =))
            (documentation))
          (part_usage composite 'frontAxleAssembly' : '1c-Parts Tree Redefinition::Definitions::AxleAssembly'[part_def]
            (part_usage composite 'frontAxle' : '1c-Parts Tree Redefinition::Definitions::Axle'[part_def])
            (part_usage composite ordered 'frontWheel' : '1c-Parts Tree Redefinition::Definitions::Wheel'[part_def]
              (multiplicity_range [2])))
          (part_usage composite 'rearAxleAssembly' : '1c-Parts Tree Redefinition::Definitions::AxleAssembly'[part_def]
            (part_usage composite 'rearAxle' : '1c-Parts Tree Redefinition::Definitions::Axle'[part_def])
            (part_usage composite ordered 'rearWheel' : '1c-Parts Tree Redefinition::Definitions::Wheel'[part_def]
              (multiplicity_range [2]))))
        (part_usage 'vehicle1_c1' :> '1c-Parts Tree Redefinition::Usages::vehicle1'[part_usage]
          (attribute_usage composite 'mass' :>> '1c-Parts Tree Redefinition::Usages::vehicle1::mass'[attribute_usage]
            (feature_value (=)))
          (part_usage composite 'frontAxleAssembly_c1' :>> '1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly'[part_usage]
            (part_usage composite 'frontAxle_c1' : '1c-Parts Tree Redefinition::Definitions::FrontAxle'[part_def] :>> '1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontAxle'[part_usage])
            (part_usage composite 'frontWheel_1' :> '1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontWheel'[part_usage]
              (feature_value (=)))
            (part_usage composite 'frontWheel_2' :> '1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontWheel'[part_usage]
              (feature_value (=))))
          (part_usage composite 'rearAxleAssembly_c1' :>> '1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly'[part_usage]
            (part_usage composite 'rearAxle_c1' :>> '1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearAxle'[part_usage])
            (part_usage composite 'rearWheel_1' :> '1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearWheel'[part_usage]
              (feature_value (=)))
            (part_usage composite 'rearWheel_2' :> '1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearWheel'[part_usage]
              (feature_value (=)))))))))
~~~
