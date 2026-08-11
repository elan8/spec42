# META
~~~ini
description=SysML Training 05 (Redefinition): Redefinition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Redefinition Example' {

	part def Vehicle {
		part eng : Engine;
	}
	part def SmallVehicle :> Vehicle {
		part smallEng : SmallEngine redefines eng;
	}
	part def BigVehicle :> Vehicle {
		part bigEng : BigEngine :>> eng;
	}

	part def Engine {
		part cyl : Cylinder[4..6];
	}
	part def SmallEngine :> Engine {
		part redefines cyl[4];
	}
	part def BigEngine :> Engine {
		part redefines cyl[6];
	}

	part def Cylinder;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "05_redefinition_example.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f009f7be8d9cceb567033c041b9ecf87d19176cecfddc9f71e9b662dcdc886e6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Redefinition Example"))) (kind "package") (name "Redefinition Example") (declared-name "Redefinition Example"))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::BigEngine"))) (kind "part def") (name "BigEngine") (declared-name "BigEngine") (parent (node (document "d0") (qualified-name "Redefinition Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl"))) (kind "part") (name "cyl") (parent (node (document "d0") (qualified-name "Redefinition Example::BigEngine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl")))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::BigVehicle"))) (kind "part def") (name "BigVehicle") (declared-name "BigVehicle") (parent (node (document "d0") (qualified-name "Redefinition Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind "part") (name "bigEng") (declared-name "bigEng") (parent (node (document "d0") (qualified-name "Redefinition Example::BigVehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "BigEngine")) (redefinition (reference "eng")))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::Cylinder"))) (kind "part def") (name "Cylinder") (declared-name "Cylinder") (parent (node (document "d0") (qualified-name "Redefinition Example"))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "Redefinition Example"))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::Engine::cyl"))) (kind "part") (name "cyl") (declared-name "cyl") (parent (node (document "d0") (qualified-name "Redefinition Example::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cylinder")))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::SmallEngine"))) (kind "part def") (name "SmallEngine") (declared-name "SmallEngine") (parent (node (document "d0") (qualified-name "Redefinition Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl"))) (kind "part") (name "cyl") (parent (node (document "d0") (qualified-name "Redefinition Example::SmallEngine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl")))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle"))) (kind "part def") (name "SmallVehicle") (declared-name "SmallVehicle") (parent (node (document "d0") (qualified-name "Redefinition Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind "part") (name "smallEng") (declared-name "smallEng") (parent (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "SmallEngine")) (redefinition (reference "eng")))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Redefinition Example"))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (parent (node (document "d0") (qualified-name "Redefinition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::BigEngine"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind featureTyping) (ordinal 0)) (authored-target "BigEngine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::BigEngine")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind redefinition) (ordinal 0)) (authored-target "eng") (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::Engine::cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "Cylinder") (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::Cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::SmallEngine"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind featureTyping) (ordinal 0)) (authored-target "SmallEngine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::SmallEngine")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind redefinition) (ordinal 0)) (authored-target "eng") (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::Engine")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Redefinition Example::BigEngine"))) (target (node (document "d0") (qualified-name "Redefinition Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::BigEngine"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl"))) (target (node (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle"))) (target (node (document "d0") (qualified-name "Redefinition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (target (node (document "d0") (qualified-name "Redefinition Example::BigEngine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (target (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Redefinition Example::Engine::cyl"))) (target (node (document "d0") (qualified-name "Redefinition Example::Cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::Engine::cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Redefinition Example::SmallEngine"))) (target (node (document "d0") (qualified-name "Redefinition Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::SmallEngine"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl"))) (target (node (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle"))) (target (node (document "d0") (qualified-name "Redefinition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (target (node (document "d0") (qualified-name "Redefinition Example::SmallEngine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (target (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (target (node (document "d0") (qualified-name "Redefinition Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 6 40) (end 6 43)) (probe (position 6 40))
      (reference
        (source (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))
        (kind redefinition) (ordinal 0) (authored-target "eng")
        (range (start 6 40) (end 6 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Redefinition Example::Vehicle::eng") (range (start 3 2) (end 3 20)))
        )
      )
    )
    (query (range (start 9 30) (end 9 33)) (probe (position 9 30))
      (reference
        (source (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))
        (kind redefinition) (ordinal 0) (authored-target "eng")
        (range (start 9 30) (end 9 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Redefinition Example::Vehicle::eng") (range (start 3 2) (end 3 20)))
        )
      )
    )
    (query (range (start 16 17) (end 16 20)) (probe (position 16 17))
      (reference
        (source (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl"))
        (kind redefinition) (ordinal 0) (authored-target "cyl")
        (range (start 16 17) (end 16 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl") (range (start 16 2) (end 16 24)))
        )
      )
    )
    (query (range (start 19 17) (end 19 20)) (probe (position 19 17))
      (reference
        (source (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl"))
        (kind redefinition) (ordinal 0) (authored-target "cyl")
        (range (start 19 17) (end 19 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl") (range (start 19 2) (end 19 24)))
        )
      )
    )
    (query (range (start 3 13) (end 3 19)) (probe (position 3 13))
      (reference
        (source (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 3 13) (end 3 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Redefinition Example::Engine") (range (start 12 1) (end 12 50)))
        )
      )
    )
    (query (range (start 15 25) (end 15 31)) (probe (position 15 25))
      (reference
        (source (document "d0") (qualified-name "Redefinition Example::SmallEngine"))
        (kind specialization) (ordinal 0) (authored-target "Engine")
        (range (start 15 25) (end 15 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Redefinition Example::Engine") (range (start 12 1) (end 12 50)))
        )
      )
    )
    (query (range (start 18 23) (end 18 29)) (probe (position 18 23))
      (reference
        (source (document "d0") (qualified-name "Redefinition Example::BigEngine"))
        (kind specialization) (ordinal 0) (authored-target "Engine")
        (range (start 18 23) (end 18 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Redefinition Example::Engine") (range (start 12 1) (end 12 50)))
        )
      )
    )
    (query (range (start 5 26) (end 5 33)) (probe (position 5 26))
      (reference
        (source (document "d0") (qualified-name "Redefinition Example::SmallVehicle"))
        (kind specialization) (ordinal 0) (authored-target "Vehicle")
        (range (start 5 26) (end 5 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Redefinition Example::Vehicle") (range (start 2 1) (end 2 43)))
        )
      )
    )
    (query (range (start 8 24) (end 8 31)) (probe (position 8 24))
      (reference
        (source (document "d0") (qualified-name "Redefinition Example::BigVehicle"))
        (kind specialization) (ordinal 0) (authored-target "Vehicle")
        (range (start 8 24) (end 8 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Redefinition Example::Vehicle") (range (start 2 1) (end 2 43)))
        )
      )
    )
    (query (range (start 13 13) (end 13 21)) (probe (position 13 13))
      (reference
        (source (document "d0") (qualified-name "Redefinition Example::Engine::cyl"))
        (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
        (range (start 13 13) (end 13 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Redefinition Example::Cylinder") (range (start 22 1) (end 22 19)))
        )
      )
    )
    (query (range (start 9 16) (end 9 25)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))
        (kind featureTyping) (ordinal 0) (authored-target "BigEngine")
        (range (start 9 16) (end 9 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Redefinition Example::BigEngine") (range (start 18 1) (end 18 59)))
        )
      )
    )
    (query (range (start 6 18) (end 6 29)) (probe (position 6 18))
      (reference
        (source (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))
        (kind featureTyping) (ordinal 0) (authored-target "SmallEngine")
        (range (start 6 18) (end 6 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Redefinition Example::SmallEngine") (range (start 15 1) (end 15 61)))
        )
      )
    )
  )
)
~~~
