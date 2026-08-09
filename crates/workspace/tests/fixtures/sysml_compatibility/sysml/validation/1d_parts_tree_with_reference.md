# META
~~~ini
description=SysML Validation (01-Parts Tree): 1d-Parts Tree with Reference
type=file
~~~
# SOURCE
~~~sysml
package '1d-Parts Tree with Reference' {
	
	package Definitions {
		part def Vehicle;
		part def Trailer;
		part def TrailerHitch;
		part def HitchBall;
		part def TrailerCoupler;
	}
	
	package Usages {
		private import Definitions::*;
		
		part vehicle_trailer_system {
			
			part vehicle1_c1: Vehicle {
				ref hitchBall : HitchBall {
					/*
					 * 'vehicle1_c1'::'hitchBall' is a reference property that
					 * references a hitch ball that is not part of this vehicle. 
					 * If 'vehicle1_c1' is removed or destroyed, this does not
					 * effect the hitchBall referenced here.
					 */
				}
			}
			
			bind vehicle1_c1.hitchBall = trailerHitch.hitchBall {
				/*
				 * This is a binding connector between the 'hitchBall' in 'vehicle1_c1'
				 * and the 'hitchBall' in 'trailerHitch'.
				 */			
			}
			
			part trailerHitch: TrailerHitch {				
				part hitchBall: HitchBall;
				part trailerCoupler: TrailerCoupler;
			}
			
			part trailer1: Trailer {
				ref trailerCoupler : TrailerCoupler = trailerHitch.trailerCoupler {
					/*
					 * This is a shorthand for a binding connector between the
					 * 'trailerCoupler' here and the 'trailerCoupler' in 'trailerHitch'.
					 * The binding connector is now contained within the 'trailer1'
					 * part, though, rather than being at the system level. 
					 */
				}
			}
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwRef,Ident,Colon,Ident,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,
KwBind,Ident,Dot,Ident,Eq,Ident,Dot,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwRef,Ident,Colon,Ident,Eq,Ident,Dot,Ident,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''1d-Parts Tree with Reference''
    (package_def 'Definitions'
      (part_def 'Vehicle')
      (part_def 'Trailer')
      (part_def 'TrailerHitch')
      (part_def 'HitchBall')
      (part_def 'TrailerCoupler'))
    (package_def 'Usages'
      (import_decl private 'Definitions::*')
      (part_usage 'vehicle_trailer_system'
        (part_usage 'vehicle1_c1' : 'Vehicle'
          (ref_usage ref 'hitchBall' : 'HitchBall'
            (comment)))
        (binding_as_usage
          (connector_end)
          (connector_end)
          (comment))
        (part_usage 'trailerHitch' : 'TrailerHitch'
          (part_usage 'hitchBall' : 'HitchBall')
          (part_usage 'trailerCoupler' : 'TrailerCoupler'))
        (part_usage 'trailer1' : 'Trailer'
          (ref_usage ref 'trailerCoupler' : 'TrailerCoupler' value
            (comment)))))))
~~~
# FORMAT
~~~sysml
package '1d-Parts Tree with Reference' {
    package Definitions {
        part def Vehicle;
        part def Trailer;
        part def TrailerHitch;
        part def HitchBall;
        part def TrailerCoupler;
    }

    package Usages {
        private import Definitions::*;

        part vehicle_trailer_system {
            part vehicle1_c1 : Vehicle {
                ref hitchBall : HitchBall {
                    /*
					 * 'vehicle1_c1'::'hitchBall' is a reference property that
					 * references a hitch ball that is not part of this vehicle. 
					 * If 'vehicle1_c1' is removed or destroyed, this does not
					 * effect the hitchBall referenced here.
					 */
                }
            }

            bind vehicle1_c1.hitchBall = trailerHitch.hitchBall {
                /*
				 * This is a binding connector between the 'hitchBall' in 'vehicle1_c1'
				 * and the 'hitchBall' in 'trailerHitch'.
				 */
            }

            part trailerHitch : TrailerHitch {
                part hitchBall : HitchBall;
                part trailerCoupler : TrailerCoupler;
            }

            part trailer1 : Trailer {
                ref trailerCoupler : TrailerCoupler = trailerHitch.trailerCoupler {
                    /*
					 * This is a shorthand for a binding connector between the
					 * 'trailerCoupler' here and the 'trailerCoupler' in 'trailerHitch'.
					 * The binding connector is now contained within the 'trailer1'
					 * part, though, rather than being at the system level. 
					 */
                }
            }
        }
    }
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(model
  (namespace
    (package '1d-Parts Tree with Reference'
      (package 'Definitions'
        (part_def 'Vehicle')
        (part_def 'Trailer')
        (part_def 'TrailerHitch')
        (part_def 'HitchBall')
        (part_def 'TrailerCoupler'))
      (package 'Usages'
        (namespace_import private -> '1d-Parts Tree with Reference::Definitions'[package])
        (part_usage 'vehicle_trailer_system'
          (part_usage composite 'vehicle1_c1' : '1d-Parts Tree with Reference::Definitions::Vehicle'[part_def]
            (reference_usage reference 'hitchBall' : '1d-Parts Tree with Reference::Definitions::HitchBall'[part_def]))
          (binding_connector_def
            (connector_end 'vehicle1_c1.hitchBall')
            (connector_end 'trailerHitch.hitchBall'))
          (part_usage composite 'trailerHitch' : '1d-Parts Tree with Reference::Definitions::TrailerHitch'[part_def]
            (part_usage composite 'hitchBall' : '1d-Parts Tree with Reference::Definitions::HitchBall'[part_def])
            (part_usage composite 'trailerCoupler' : '1d-Parts Tree with Reference::Definitions::TrailerCoupler'[part_def]))
          (part_usage composite 'trailer1' : '1d-Parts Tree with Reference::Definitions::Trailer'[part_def]
            (reference_usage reference 'trailerCoupler' : '1d-Parts Tree with Reference::Definitions::TrailerCoupler'[part_def]
              (feature_value (=)))))))))
~~~
