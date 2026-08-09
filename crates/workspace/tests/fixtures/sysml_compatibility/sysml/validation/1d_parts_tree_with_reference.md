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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference"))) (name "1d-Parts Tree with Reference") (declared-name "1d-Parts Tree with Reference")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions"))) (name "Definitions") (declared-name "Definitions")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall"))) (name "HitchBall") (declared-name "HitchBall") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::Trailer"))) (name "Trailer") (declared-name "Trailer") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler"))) (name "TrailerCoupler") (declared-name "TrailerCoupler") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerHitch"))) (name "TrailerHitch") (declared-name "TrailerHitch") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages"))) (name "Usages") (declared-name "Usages")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::*"))) (name "*") (declared-name "*"))
            (element (kind "part") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (name "vehicle_trailer_system") (declared-name "vehicle_trailer_system") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1"))) (name "trailer1") (declared-name "trailer1") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "ref") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (name "trailerCoupler") (declared-name "trailerCoupler") (declared (properties (composite false) (reference true)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "trailerCoupler") (children (expression (kind "featureReference") (reference "trailerHitch")))))) (effective (featuring-type (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::Trailer"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (role feature-value))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (name "trailerHitch") (declared-name "trailerHitch") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall"))) (name "hitchBall") (declared-name "hitchBall") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerHitch")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler"))) (name "trailerCoupler") (declared-name "trailerCoupler") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerHitch")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1"))) (name "vehicle1_c1") (declared-name "vehicle1_c1") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "ref") (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall"))) (name "hitchBall") (declared-name "hitchBall") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::Vehicle")))))
                  )
                )
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (bind (status resolved) (from (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall"))) (to (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall"))) (connect (source-expression "vehicle1_c1::hitchBall") (target-expression "trailerHitch::hitchBall") (container-prefix "1d-Parts Tree with Reference::Usages::vehicle_trailer_system")))
    (reference (status resolved) (from (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (to (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1"))) (to (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::Trailer"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (to (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (to (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerHitch"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall"))) (to (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler"))) (to (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1"))) (to (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall"))) (to (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/1d_parts_tree_with_reference.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 2) (end 11 32))
      )
    )
  )
)
~~~
