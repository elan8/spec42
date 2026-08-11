# META
~~~ini
description=SysML Training 03 (Generalization): Generalization Example
type=file
~~~
# SOURCE
~~~sysml
package 'Generalization Example' {

	abstract part def Vehicle;
	
	part def HumanDrivenVehicle specializes Vehicle {
		ref part driver : Person;
	}
	
	part def PoweredVehicle :> Vehicle {
		part eng : Engine;
	}
	
	part def HumanDrivenPoweredVehicle :> 
		HumanDrivenVehicle, PoweredVehicle;
	
	part def Engine;	
	part def Person;
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "03_generalization_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Generalization Example' {

    abstract part def Vehicle;

    part def HumanDrivenVehicle specializes Vehicle {
        ref part driver : Person;
    }

    part def PoweredVehicle :> Vehicle {
        part eng : Engine;
    }

    part def HumanDrivenPoweredVehicle :>
    HumanDrivenVehicle, PoweredVehicle;

    part def Engine;
    part def Person;

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b6ada02dc0db6fe253a4094cc7365e36681b71a023d0dd7e72850e65dc1a51e2") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Generalization Example"))) (kind "package") (name "Generalization Example") (declared-name "Generalization Example"))
    (element (id (node (document "d0") (qualified-name "Generalization Example::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "Generalization Example"))))
    (element (id (node (document "d0") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (kind "part def") (name "HumanDrivenPoweredVehicle") (declared-name "HumanDrivenPoweredVehicle") (parent (node (document "d0") (qualified-name "Generalization Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "HumanDrivenVehicle")) (specializes (reference "PoweredVehicle")))))
    (element (id (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (kind "part def") (name "HumanDrivenVehicle") (declared-name "HumanDrivenVehicle") (parent (node (document "d0") (qualified-name "Generalization Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle::driver"))) (kind "ref") (name "driver") (declared-name "driver") (parent (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Person")))))
    (element (id (node (document "d0") (qualified-name "Generalization Example::Person"))) (kind "part def") (name "Person") (declared-name "Person") (parent (node (document "d0") (qualified-name "Generalization Example"))))
    (element (id (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle"))) (kind "part def") (name "PoweredVehicle") (declared-name "PoweredVehicle") (parent (node (document "d0") (qualified-name "Generalization Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (parent (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Generalization Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Generalization Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (kind specialization) (ordinal 0)) (authored-target "HumanDrivenVehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (kind specialization) (ordinal 1)) (authored-target "PoweredVehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Generalization Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (outcome (status resolved) (target (node (document "d0") (qualified-name "Generalization Example::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Generalization Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Generalization Example::Engine")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (target (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (target (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (target (node (document "d0") (qualified-name "Generalization Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle::driver"))) (target (node (document "d0") (qualified-name "Generalization Example::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle"))) (target (node (document "d0") (qualified-name "Generalization Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle::eng"))) (target (node (document "d0") (qualified-name "Generalization Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle::eng"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 5 20) (end 5 26)) (probe (position 5 20))
      (reference
        (source (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle::driver"))
        (kind featureTyping) (ordinal 0) (authored-target "Person")
        (range (start 5 20) (end 5 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Generalization Example::Person") (range (start 16 1) (end 16 17)))
        )
      )
    )
    (query (range (start 9 13) (end 9 19)) (probe (position 9 13))
      (reference
        (source (document "d0") (qualified-name "Generalization Example::PoweredVehicle::eng"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 9 13) (end 9 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Generalization Example::Engine") (range (start 15 1) (end 15 17)))
        )
      )
    )
    (query (range (start 4 41) (end 4 48)) (probe (position 4 41))
      (reference
        (source (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle"))
        (kind specialization) (ordinal 0) (authored-target "Vehicle")
        (range (start 4 41) (end 4 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Generalization Example::Vehicle") (range (start 2 1) (end 2 27)))
        )
      )
    )
    (query (range (start 8 28) (end 8 35)) (probe (position 8 28))
      (reference
        (source (document "d0") (qualified-name "Generalization Example::PoweredVehicle"))
        (kind specialization) (ordinal 0) (authored-target "Vehicle")
        (range (start 8 28) (end 8 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Generalization Example::Vehicle") (range (start 2 1) (end 2 27)))
        )
      )
    )
    (query (range (start 13 22) (end 13 36)) (probe (position 13 22))
      (reference
        (source (document "d0") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))
        (kind specialization) (ordinal 1) (authored-target "PoweredVehicle")
        (range (start 13 22) (end 13 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Generalization Example::PoweredVehicle") (range (start 8 1) (end 8 61)))
        )
      )
    )
    (query (range (start 13 2) (end 13 20)) (probe (position 13 2))
      (reference
        (source (document "d0") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))
        (kind specialization) (ordinal 0) (authored-target "HumanDrivenVehicle")
        (range (start 13 2) (end 13 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle") (range (start 4 1) (end 4 81)))
        )
      )
    )
  )
)
~~~
