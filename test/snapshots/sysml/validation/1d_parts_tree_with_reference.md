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
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference"))) (kind "package") (name "1d-Parts Tree with Reference") (declared-name "1d-Parts Tree with Reference"))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference"))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall"))) (kind "part def") (name "HitchBall") (declared-name "HitchBall") (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::Trailer"))) (kind "part def") (name "Trailer") (declared-name "Trailer") (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler"))) (kind "part def") (name "TrailerCoupler") (declared-name "TrailerCoupler") (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerHitch"))) (kind "part def") (name "TrailerHitch") (declared-name "TrailerHitch") (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Definitions"))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference"))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (kind "part") (name "vehicle_trailer_system") (declared-name "vehicle_trailer_system") (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages"))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1"))) (kind "part") (name "trailer1") (declared-name "trailer1") (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (authored (membership (kind Feature)) (relationships (typing (reference "Trailer")))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (kind "ref") (name "trailerCoupler") (declared-name "trailerCoupler") (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerCoupler")) (reference (reference "trailerHitch.trailerCoupler")))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (kind "part") (name "trailerHitch") (declared-name "trailerHitch") (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerHitch")))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall"))) (kind "part") (name "hitchBall") (declared-name "hitchBall") (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (authored (membership (kind Feature)) (relationships (typing (reference "HitchBall")))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler"))) (kind "part") (name "trailerCoupler") (declared-name "trailerCoupler") (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerCoupler")))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall"))) (kind "ref") (name "hitchBall") (declared-name "hitchBall") (parent (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "HitchBall")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (kind bindSource) (ordinal 0)) (authored-target "vehicle1_c1::hitchBall") (outcome (status resolved) (target (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall")))))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (kind bindTarget) (ordinal 0)) (authored-target "trailerHitch::hitchBall") (outcome (status resolved) (target (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall")))))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1"))) (kind featureTyping) (ordinal 0)) (authored-target "Trailer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerCoupler") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (kind referenceSource) (ordinal 0)) (authored-target "trailerHitch.trailerCoupler") (outcome (status resolved) (target (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler")))))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerHitch") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall"))) (kind featureTyping) (ordinal 0)) (authored-target "HitchBall") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerCoupler") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall"))) (kind featureTyping) (ordinal 0)) (authored-target "HitchBall") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind reference) (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (target (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (kind referenceSource) (ordinal 0)))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall"))) (target (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (kind bindSource) (ordinal 0)) (expression (kind bind) (source "vehicle1_c1::hitchBall") (target "trailerHitch::hitchBall")))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 15 21) (end 15 28)) (probe (position 15 21))
      (reference
        (source (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 15 21) (end 15 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 38 18) (end 38 25)) (probe (position 38 18))
      (reference
        (source (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1"))
        (kind featureTyping) (ordinal 0) (authored-target "Trailer")
        (range (start 38 18) (end 38 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 20) (end 16 29)) (probe (position 16 20))
      (reference
        (source (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall"))
        (kind featureTyping) (ordinal 0) (authored-target "HitchBall")
        (range (start 16 20) (end 16 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 34 20) (end 34 29)) (probe (position 34 20))
      (reference
        (source (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall"))
        (kind featureTyping) (ordinal 0) (authored-target "HitchBall")
        (range (start 34 20) (end 34 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 17) (end 11 28)) (probe (position 11 17))
      (reference
        (source (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Definitions::*")
        (range (start 11 17) (end 11 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 22) (end 33 34)) (probe (position 33 22))
      (reference
        (source (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))
        (kind featureTyping) (ordinal 0) (authored-target "TrailerHitch")
        (range (start 33 22) (end 33 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 35 25) (end 35 39)) (probe (position 35 25))
      (reference
        (source (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler"))
        (kind featureTyping) (ordinal 0) (authored-target "TrailerCoupler")
        (range (start 35 25) (end 35 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 39 25) (end 39 39)) (probe (position 39 25))
      (reference
        (source (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))
        (kind featureTyping) (ordinal 0) (authored-target "TrailerCoupler")
        (range (start 39 25) (end 39 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 26 8) (end 26 29)) (probe (position 26 8))
      (reference
        (source (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))
        (kind bindSource) (ordinal 0) (authored-target "vehicle1_c1::hitchBall")
        (range (start 26 8) (end 26 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall") (range (start 16 4) (end 16 295)))
        )
      )
    )
    (query (range (start 26 32) (end 26 54)) (probe (position 26 32))
      (reference
        (source (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))
        (kind bindTarget) (ordinal 0) (authored-target "trailerHitch::hitchBall")
        (range (start 26 32) (end 26 54))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall") (range (start 34 4) (end 34 30)))
        )
      )
    )
  )
)
~~~
