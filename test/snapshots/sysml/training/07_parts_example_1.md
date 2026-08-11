# META
~~~ini
description=SysML Training 07 (Parts): Parts Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Parts Example-1' {
	
	// Definitions
	
	part def Vehicle {
		part eng : Engine;
	}
	
	part def Engine {
		part cyl : Cylinder[4..6];
	}
	
	part def Cylinder;
	
	// Usages
	
	part smallVehicle : Vehicle {
		part redefines eng {
			part redefines cyl[4];
		}
	}
	
	part bigVehicle : Vehicle {
		part redefines eng {
			part redefines cyl[6];
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "07_parts_example_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Parts Example-1' {

    // Definitions

    part def Vehicle {
        part eng : Engine;
    }

    part def Engine {
        part cyl : Cylinder[4..6];
    }

    part def Cylinder;

    // Usages

    part smallVehicle : Vehicle {
        part redefines eng {
            part redefines cyl[4];
        }
    }

    part bigVehicle : Vehicle {
        part redefines eng {
            part redefines cyl[6];
        }
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5237fc4f7f88fbf13ee074fc04d6e22677877a85a9c824d03e611da74082f2be") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Parts Example-1"))) (kind "package") (name "Parts Example-1") (declared-name "Parts Example-1"))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::Cylinder"))) (kind "part def") (name "Cylinder") (declared-name "Cylinder") (parent (node (document "d0") (qualified-name "Parts Example-1"))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "Parts Example-1"))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::Engine::cyl"))) (kind "part") (name "cyl") (declared-name "cyl") (parent (node (document "d0") (qualified-name "Parts Example-1::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cylinder")))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Parts Example-1"))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::Vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (parent (node (document "d0") (qualified-name "Parts Example-1::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::bigVehicle"))) (kind "part") (name "bigVehicle") (declared-name "bigVehicle") (parent (node (document "d0") (qualified-name "Parts Example-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng"))) (kind "part") (name "eng") (parent (node (document "d0") (qualified-name "Parts Example-1::bigVehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "eng")))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl"))) (kind "part") (name "cyl") (parent (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl")))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::smallVehicle"))) (kind "part") (name "smallVehicle") (declared-name "smallVehicle") (parent (node (document "d0") (qualified-name "Parts Example-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng"))) (kind "part") (name "eng") (parent (node (document "d0") (qualified-name "Parts Example-1::smallVehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "eng")))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl"))) (kind "part") (name "cyl") (parent (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-1::Engine::cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "Cylinder") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-1::Cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-1::Vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-1::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-1::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng"))) (kind redefinition) (ordinal 0)) (authored-target "eng") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-1::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng"))) (kind redefinition) (ordinal 0)) (authored-target "eng") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts Example-1::Engine::cyl"))) (target (node (document "d0") (qualified-name "Parts Example-1::Cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-1::Engine::cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts Example-1::Vehicle::eng"))) (target (node (document "d0") (qualified-name "Parts Example-1::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-1::Vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle"))) (target (node (document "d0") (qualified-name "Parts Example-1::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng"))) (target (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl"))) (target (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle"))) (target (node (document "d0") (qualified-name "Parts Example-1::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng"))) (target (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl"))) (target (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl"))) (kind redefinition) (ordinal 0)))
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
        (source (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng"))
        (kind redefinition) (ordinal 0) (authored-target "eng")
        (range (start 17 17) (end 17 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng") (range (start 17 2) (end 17 52)))
        )
      )
    )
    (query (range (start 18 18) (end 18 21)) (probe (position 18 18))
      (reference
        (source (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl"))
        (kind redefinition) (ordinal 0) (authored-target "cyl")
        (range (start 18 18) (end 18 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl") (range (start 18 3) (end 18 25)))
        )
      )
    )
    (query (range (start 23 17) (end 23 20)) (probe (position 23 17))
      (reference
        (source (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng"))
        (kind redefinition) (ordinal 0) (authored-target "eng")
        (range (start 23 17) (end 23 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng") (range (start 23 2) (end 23 52)))
        )
      )
    )
    (query (range (start 24 18) (end 24 21)) (probe (position 24 18))
      (reference
        (source (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl"))
        (kind redefinition) (ordinal 0) (authored-target "cyl")
        (range (start 24 18) (end 24 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl") (range (start 24 3) (end 24 25)))
        )
      )
    )
    (query (range (start 5 13) (end 5 19)) (probe (position 5 13))
      (reference
        (source (document "d0") (qualified-name "Parts Example-1::Vehicle::eng"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 5 13) (end 5 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-1::Engine") (range (start 8 1) (end 8 50)))
        )
      )
    )
    (query (range (start 16 21) (end 16 28)) (probe (position 16 21))
      (reference
        (source (document "d0") (qualified-name "Parts Example-1::smallVehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 16 21) (end 16 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-1::Vehicle") (range (start 4 1) (end 4 43)))
        )
      )
    )
    (query (range (start 22 19) (end 22 26)) (probe (position 22 19))
      (reference
        (source (document "d0") (qualified-name "Parts Example-1::bigVehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 22 19) (end 22 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-1::Vehicle") (range (start 4 1) (end 4 43)))
        )
      )
    )
    (query (range (start 9 13) (end 9 21)) (probe (position 9 13))
      (reference
        (source (document "d0") (qualified-name "Parts Example-1::Engine::cyl"))
        (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
        (range (start 9 13) (end 9 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Parts Example-1::Cylinder") (range (start 12 1) (end 12 19)))
        )
      )
    )
  )
)
~~~
