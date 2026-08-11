# META
~~~ini
description=SysML Training 07 (Parts): Parts Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Parts Example-2' {
	
	// Definitions
	
	part def Vehicle;	
	part def Engine;	
	part def Cylinder;
	
	// Usages
	
	part vehicle : Vehicle {
		part eng : Engine {
			part cyl : Cylinder[4..6];
		}
	}
	
	part smallVehicle :> vehicle {
		part redefines eng {
			part redefines cyl[4];
		}
	}
	
	part bigVehicle :> vehicle {
		part redefines eng {
			part redefines cyl[6];
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "07_parts_example_2.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6fb61cb797210f46e7279e25502702da57fc43fe198992e16f7af201e94d8698") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Parts Example-2"))) (kind "package") (name "Parts Example-2") (declared-name "Parts Example-2"))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::Cylinder"))) (kind "part def") (name "Cylinder") (declared-name "Cylinder") (parent (node (document "d0") (qualified-name "Parts Example-2"))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "Parts Example-2"))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Parts Example-2"))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::bigVehicle"))) (kind "part") (name "bigVehicle") (declared-name "bigVehicle") (parent (node (document "d0") (qualified-name "Parts Example-2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle")))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng"))) (kind "part") (name "eng") (parent (node (document "d0") (qualified-name "Parts Example-2::bigVehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "eng")))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng::cyl"))) (kind "part") (name "cyl") (parent (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl")))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::smallVehicle"))) (kind "part") (name "smallVehicle") (declared-name "smallVehicle") (parent (node (document "d0") (qualified-name "Parts Example-2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle")))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng"))) (kind "part") (name "eng") (parent (node (document "d0") (qualified-name "Parts Example-2::smallVehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "eng")))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng::cyl"))) (kind "part") (name "cyl") (parent (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl")))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Parts Example-2"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (parent (node (document "d0") (qualified-name "Parts Example-2::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng::cyl"))) (kind "part") (name "cyl") (declared-name "cyl") (parent (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cylinder")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng"))) (kind redefinition) (ordinal 0)) (authored-target "eng") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng"))) (kind redefinition) (ordinal 0)) (authored-target "eng") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng::cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "Cylinder") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::Cylinder")))))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle"))) (target (node (document "d0") (qualified-name "Parts Example-2::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng"))) (target (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng::cyl"))) (target (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng::cyl"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle"))) (target (node (document "d0") (qualified-name "Parts Example-2::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng"))) (target (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng::cyl"))) (target (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng::cyl"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts Example-2::vehicle"))) (target (node (document "d0") (qualified-name "Parts Example-2::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng"))) (target (node (document "d0") (qualified-name "Parts Example-2::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng::cyl"))) (target (node (document "d0") (qualified-name "Parts Example-2::Cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng::cyl"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 17 17) (end 17 20)) (probe (position 17 17))
      (reference
        (source (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng"))
        (kind redefinition) (ordinal 0) (authored-target "eng")
        (range (start 17 17) (end 17 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng") (range (start 17 2) (end 17 52)))
        )
      )
    )
    (query (range (start 18 18) (end 18 21)) (probe (position 18 18))
      (reference
        (source (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng::cyl"))
        (kind redefinition) (ordinal 0) (authored-target "cyl")
        (range (start 18 18) (end 18 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng::cyl") (range (start 18 3) (end 18 25)))
        )
      )
    )
    (query (range (start 23 17) (end 23 20)) (probe (position 23 17))
      (reference
        (source (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng"))
        (kind redefinition) (ordinal 0) (authored-target "eng")
        (range (start 23 17) (end 23 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng") (range (start 23 2) (end 23 52)))
        )
      )
    )
    (query (range (start 24 18) (end 24 21)) (probe (position 24 18))
      (reference
        (source (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng::cyl"))
        (kind redefinition) (ordinal 0) (authored-target "cyl")
        (range (start 24 18) (end 24 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng::cyl") (range (start 24 3) (end 24 25)))
        )
      )
    )
    (query (range (start 11 13) (end 11 19)) (probe (position 11 13))
      (reference
        (source (document "d0") (qualified-name "Parts Example-2::vehicle::eng"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 11 13) (end 11 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-2::Engine") (range (start 5 1) (end 5 17)))
        )
      )
    )
    (query (range (start 10 16) (end 10 23)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "Parts Example-2::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 10 16) (end 10 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-2::Vehicle") (range (start 4 1) (end 4 18)))
        )
      )
    )
    (query (range (start 16 22) (end 16 29)) (probe (position 16 22))
      (reference
        (source (document "d0") (qualified-name "Parts Example-2::smallVehicle"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle")
        (range (start 16 22) (end 16 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-2::vehicle") (range (start 10 1) (end 10 84)))
        )
      )
    )
    (query (range (start 22 20) (end 22 27)) (probe (position 22 20))
      (reference
        (source (document "d0") (qualified-name "Parts Example-2::bigVehicle"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle")
        (range (start 22 20) (end 22 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-2::vehicle") (range (start 10 1) (end 10 84)))
        )
      )
    )
    (query (range (start 12 14) (end 12 22)) (probe (position 12 14))
      (reference
        (source (document "d0") (qualified-name "Parts Example-2::vehicle::eng::cyl"))
        (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
        (range (start 12 14) (end 12 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-2::Cylinder") (range (start 6 1) (end 6 19)))
        )
      )
    )
  )
)
~~~
