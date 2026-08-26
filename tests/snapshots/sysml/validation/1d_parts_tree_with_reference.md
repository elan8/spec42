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
  (document "memory://snapshot/1d_parts_tree_with_reference.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 11 17) (end 11 31))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 13 2) (end 48 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 8) (end 26 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 32) (end 26 54))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:0f8762df4050efb46434e714370d6b42149f7ac35d5cad124ec1173c2d8c42b3") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::Trailer"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerHitch"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle_trailer_system")) (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (documentation (comment (text "\n\t\t\t\t * This is a binding connector between the 'hitchBall' in 'vehicle1_c1'\n\t\t\t\t * and the 'hitchBall' in 'trailerHitch'.\n\t\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle1_c1::hitchBall")) (memberAccessOperand (reference "trailerHitch::hitchBall")))))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Trailer")))))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (kind ref) (membership (kind feature) (visibility default)) (documentation (comment (text "\n\t\t\t\t\t * This is a shorthand for a binding connector between the\n\t\t\t\t\t * 'trailerCoupler' here and the 'trailerCoupler' in 'trailerHitch'.\n\t\t\t\t\t * The binding connector is now contained within the 'trailer1'\n\t\t\t\t\t * part, though, rather than being at the system level. \n\t\t\t\t\t "))) (feature-value (kind bind) (value (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle_trailer_system")) (named (kind part) (name "trailer1")) (named (kind ref) (name "trailerCoupler")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle_trailer_system")) (named (kind part) (name "trailer1")) (named (kind ref) (name "trailerCoupler")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TrailerCoupler")))))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle_trailer_system")) (named (kind part) (name "trailer1")) (named (kind ref) (name "trailerCoupler")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle_trailer_system")) (named (kind part) (name "trailer1")) (named (kind ref) (name "trailerCoupler")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle_trailer_system")) (named (kind part) (name "trailer1")) (named (kind ref) (name "trailerCoupler")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TrailerHitch")))))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HitchBall")))))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TrailerCoupler")))))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall"))) (kind ref) (membership (kind feature) (visibility default)) (documentation (comment (text "\n\t\t\t\t\t * 'vehicle1_c1'::'hitchBall' is a reference property that\n\t\t\t\t\t * references a hitch ball that is not part of this vehicle. \n\t\t\t\t\t * If 'vehicle1_c1' is removed or destroyed, this does not\n\t\t\t\t\t * effect the hitchBall referenced here.\n\t\t\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HitchBall")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle_trailer_system")) (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle1_c1::hitchBall")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle_trailer_system")) (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "trailerHitch::hitchBall")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Trailer")
      (outcome (status resolved) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::Trailer")))))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (kind featureTyping) (ordinal 0))
      (authored-target "TrailerCoupler")
      (outcome (status resolved) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler")))))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (kind featureTyping) (ordinal 0))
      (authored-target "TrailerHitch")
      (outcome (status resolved) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerHitch")))))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall"))) (kind featureTyping) (ordinal 0))
      (authored-target "HitchBall")
      (outcome (status resolved) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall")))))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler"))) (kind featureTyping) (ordinal 0))
      (authored-target "TrailerCoupler")
      (outcome (status resolved) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler")))))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall"))) (kind featureTyping) (ordinal 0))
      (authored-target "HitchBall")
      (outcome (status resolved) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1"))) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::Trailer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerHitch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall"))) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler"))) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1"))) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall"))) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle_trailer_system")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1"))) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle_trailer_system")) (named (kind part) (name "trailer1")) (named (kind ref) (name "trailerCoupler")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle_trailer_system")) (named (kind part) (name "trailer1")) (named (kind ref) (name "trailerCoupler")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall"))) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler"))) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1"))) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall"))) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall")))
      (subtype (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall")) (scopes any))
      (subtype (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::Trailer")))
      (subtype (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler")))
      (subtype (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler")) (scopes any))
      (subtype (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerHitch")))
      (subtype (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::Vehicle")))
      (subtype (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle_trailer_system")) (anonymous (kind bind) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system")))
    )
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1")))
      (featured-by (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system")))
      (type (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::Trailer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::Trailer")) (source direct))
      (supertype (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::Trailer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler")))
      (featured-by (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1")))
      (type (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler")) (provenance authored))
      (effective-type (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler")) (source direct))
      (supertype (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle_trailer_system")) (named (kind part) (name "trailer1")) (named (kind ref) (name "trailerCoupler")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle_trailer_system")) (named (kind part) (name "trailer1")) (named (kind ref) (name "trailerCoupler")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch")))
      (featured-by (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system")))
      (type (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerHitch")) (provenance authored))
      (effective-type (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerHitch")) (source direct))
      (supertype (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerHitch")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall")))
      (featured-by (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch")))
      (type (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall")) (provenance authored))
      (effective-type (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall")) (source direct))
      (supertype (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler")))
      (featured-by (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch")))
      (type (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler")) (provenance authored))
      (effective-type (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler")) (source direct))
      (supertype (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1")))
      (featured-by (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system")))
      (type (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall")))
      (featured-by (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1")))
      (type (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall")) (provenance authored))
      (effective-type (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall")) (source direct))
      (supertype (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/1d_parts_tree_with_reference.md") (range (start 11 17) (end 11 31)) (probe (position 11 17))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions")))))
    )
  )
  (query (document "memory://snapshot/1d_parts_tree_with_reference.md") (range (start 26 8) (end 26 29)) (probe (position 26 8))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle_trailer_system")) (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle1_c1::hitchBall")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/1d_parts_tree_with_reference.md") (range (start 26 32) (end 26 54)) (probe (position 26 32))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (path (named (kind package) (name "1d-Parts Tree with Reference")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle_trailer_system")) (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "trailerHitch::hitchBall")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/1d_parts_tree_with_reference.md") (range (start 38 18) (end 38 25)) (probe (position 38 18))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1"))) (kind featureTyping) (ordinal 0) (authored-target "Trailer")
      (outcome (status resolved) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::Trailer")))))
    )
  )
  (query (document "memory://snapshot/1d_parts_tree_with_reference.md") (range (start 39 25) (end 39 39)) (probe (position 39 25))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailer1::trailerCoupler"))) (kind featureTyping) (ordinal 0) (authored-target "TrailerCoupler")
      (outcome (status resolved) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler")))))
    )
  )
  (query (document "memory://snapshot/1d_parts_tree_with_reference.md") (range (start 33 22) (end 33 34)) (probe (position 33 22))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch"))) (kind featureTyping) (ordinal 0) (authored-target "TrailerHitch")
      (outcome (status resolved) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerHitch")))))
    )
  )
  (query (document "memory://snapshot/1d_parts_tree_with_reference.md") (range (start 34 20) (end 34 29)) (probe (position 34 20))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::hitchBall"))) (kind featureTyping) (ordinal 0) (authored-target "HitchBall")
      (outcome (status resolved) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall")))))
    )
  )
  (query (document "memory://snapshot/1d_parts_tree_with_reference.md") (range (start 35 25) (end 35 39)) (probe (position 35 25))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::trailerHitch::trailerCoupler"))) (kind featureTyping) (ordinal 0) (authored-target "TrailerCoupler")
      (outcome (status resolved) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::TrailerCoupler")))))
    )
  )
  (query (document "memory://snapshot/1d_parts_tree_with_reference.md") (range (start 15 21) (end 15 28)) (probe (position 15 21))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/1d_parts_tree_with_reference.md") (range (start 16 20) (end 16 29)) (probe (position 16 20))
    (reference (id (source (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Usages::vehicle_trailer_system::vehicle1_c1::hitchBall"))) (kind featureTyping) (ordinal 0) (authored-target "HitchBall")
      (outcome (status resolved) (target (node (document "memory://snapshot/1d_parts_tree_with_reference.md") (qualified-name "1d-Parts Tree with Reference::Definitions::HitchBall")))))
    )
  )
)
~~~
