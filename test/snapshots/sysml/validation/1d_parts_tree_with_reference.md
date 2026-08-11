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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "1d_parts_tree_with_reference.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 17) (end 11 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 21) (end 15 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 20) (end 16 29))
      )
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 26 8) (end 26 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 22) (end 33 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 20) (end 34 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 25) (end 35 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 18) (end 38 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 25) (end 39 39))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "51e948294d3ffe86c6e0888e735403d13f0094d51c87a0b43d59198f7d0936c8") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference"))) (kind "package") (name "1d-Parts Tree with Reference") (declared-name "1d-Parts Tree with Reference") (range (start (line 0) (character 0)) (end (line 0) (character 1344))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (range (start (line 2) (character 1)) (end (line 2) (character 139))) (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference"))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall"))) (kind "part def") (name "HitchBall") (declared-name "HitchBall") (range (start (line 6) (character 2)) (end (line 6) (character 21))) (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::Trailer"))) (kind "part def") (name "Trailer") (declared-name "Trailer") (range (start (line 4) (character 2)) (end (line 4) (character 19))) (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler"))) (kind "part def") (name "TrailerCoupler") (declared-name "TrailerCoupler") (range (start (line 7) (character 2)) (end (line 7) (character 26))) (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerHitch"))) (kind "part def") (name "TrailerHitch") (declared-name "TrailerHitch") (range (start (line 5) (character 2)) (end (line 5) (character 24))) (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 3) (character 2)) (end (line 3) (character 19))) (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (range (start (line 10) (character 1)) (end (line 10) (character 1157))) (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference"))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 11) (character 2)) (end (line 11) (character 32))) (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 11) (character 17)) (end (line 11) (character 28))))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (kind "part") (name "vehicle_trailer_system") (declared-name "vehicle_trailer_system") (range (start (line 13) (character 2)) (end (line 13) (character 1100))) (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages"))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1"))) (kind "part") (name "trailer1") (declared-name "trailer1") (range (start (line 38) (character 3)) (end (line 38) (character 396))) (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (authored (membership (kind Feature)) (relationships (typing (reference "Trailer") (range (start (line 38) (character 18)) (end (line 38) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (kind "ref") (name "trailerCoupler") (declared-name "trailerCoupler") (range (start (line 39) (character 4)) (end (line 39) (character 363))) (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerCoupler") (range (start (line 39) (character 25)) (end (line 39) (character 39)))) (reference (reference "trailerHitch.trailerCoupler") (range none)))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (kind "part") (name "trailerHitch") (declared-name "trailerHitch") (range (start (line 33) (character 3)) (end (line 33) (character 117))) (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerHitch") (range (start (line 33) (character 22)) (end (line 33) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall"))) (kind "part") (name "hitchBall") (declared-name "hitchBall") (range (start (line 34) (character 4)) (end (line 34) (character 30))) (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (authored (membership (kind Feature)) (relationships (typing (reference "HitchBall") (range (start (line 34) (character 20)) (end (line 34) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler"))) (kind "part") (name "trailerCoupler") (declared-name "trailerCoupler") (range (start (line 35) (character 4)) (end (line 35) (character 40))) (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerCoupler") (range (start (line 35) (character 25)) (end (line 35) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (range (start (line 15) (character 3)) (end (line 15) (character 331))) (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 15) (character 21)) (end (line 15) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall"))) (kind "ref") (name "hitchBall") (declared-name "hitchBall") (range (start (line 16) (character 4)) (end (line 16) (character 295))) (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "HitchBall") (range (start (line 16) (character 20)) (end (line 16) (character 29)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (range (start (line 11) (character 17)) (end (line 11) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (kind bindSource) (ordinal 0)) (authored-target "vehicle1_c1::hitchBall") (range (start (line 26) (character 8)) (end (line 26) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall")))))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (kind bindTarget) (ordinal 0)) (authored-target "trailerHitch::hitchBall") (range (start (line 26) (character 32)) (end (line 26) (character 54))) (outcome (status resolved) (target (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall")))))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1"))) (kind featureTyping) (ordinal 0)) (authored-target "Trailer") (range (start (line 38) (character 18)) (end (line 38) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerCoupler") (range (start (line 39) (character 25)) (end (line 39) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (kind referenceSource) (ordinal 0)) (authored-target "trailerHitch.trailerCoupler") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler")))))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerHitch") (range (start (line 33) (character 22)) (end (line 33) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall"))) (kind featureTyping) (ordinal 0)) (authored-target "HitchBall") (range (start (line 34) (character 20)) (end (line 34) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerCoupler") (range (start (line 35) (character 25)) (end (line 35) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 15) (character 21)) (end (line 15) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall"))) (kind featureTyping) (ordinal 0)) (authored-target "HitchBall") (range (start (line 16) (character 20)) (end (line 16) (character 29))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind reference) (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (target (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (kind referenceSource) (ordinal 0)))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall"))) (target (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (kind bindSource) (ordinal 0)) (expression (kind bind) (source "vehicle1_c1::hitchBall") (target "trailerHitch::hitchBall") (source-range (start (line 26) (character 8)) (end (line 26) (character 29))) (target-range (start (line 26) (character 32)) (end (line 26) (character 54)))))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
