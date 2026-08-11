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
    (element (id (node (document "d0") (qualified-name "Generalization Example"))) (kind "package") (name "Generalization Example") (declared-name "Generalization Example") (range (start (line 0) (character 0)) (end (line 0) (character 334))))
    (element (id (node (document "d0") (qualified-name "Generalization Example::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 15) (character 1)) (end (line 15) (character 17))) (parent (node (document "d0") (qualified-name "Generalization Example"))))
    (element (id (node (document "d0") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (kind "part def") (name "HumanDrivenPoweredVehicle") (declared-name "HumanDrivenPoweredVehicle") (range (start (line 12) (character 1)) (end (line 12) (character 77))) (parent (node (document "d0") (qualified-name "Generalization Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "HumanDrivenVehicle") (range (start (line 13) (character 2)) (end (line 13) (character 20)))) (specializes (reference "PoweredVehicle") (range (start (line 13) (character 22)) (end (line 13) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (kind "part def") (name "HumanDrivenVehicle") (declared-name "HumanDrivenVehicle") (range (start (line 4) (character 1)) (end (line 4) (character 81))) (parent (node (document "d0") (qualified-name "Generalization Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle") (range (start (line 4) (character 41)) (end (line 4) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle::driver"))) (kind "ref") (name "driver") (declared-name "driver") (range (start (line 5) (character 2)) (end (line 5) (character 27))) (parent (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Person") (range (start (line 5) (character 20)) (end (line 5) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "Generalization Example::Person"))) (kind "part def") (name "Person") (declared-name "Person") (range (start (line 16) (character 1)) (end (line 16) (character 17))) (parent (node (document "d0") (qualified-name "Generalization Example"))))
    (element (id (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle"))) (kind "part def") (name "PoweredVehicle") (declared-name "PoweredVehicle") (range (start (line 8) (character 1)) (end (line 8) (character 61))) (parent (node (document "d0") (qualified-name "Generalization Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle") (range (start (line 8) (character 28)) (end (line 8) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 9) (character 2)) (end (line 9) (character 20))) (parent (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 9) (character 13)) (end (line 9) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "Generalization Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 2) (character 1)) (end (line 2) (character 27))) (parent (node (document "d0") (qualified-name "Generalization Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (kind specialization) (ordinal 0)) (authored-target "HumanDrivenVehicle") (range (start (line 13) (character 2)) (end (line 13) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (kind specialization) (ordinal 1)) (authored-target "PoweredVehicle") (range (start (line 13) (character 22)) (end (line 13) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (range (start (line 4) (character 41)) (end (line 4) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Generalization Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range (start (line 5) (character 20)) (end (line 5) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Generalization Example::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (range (start (line 8) (character 28)) (end (line 8) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Generalization Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 9) (character 13)) (end (line 9) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Generalization Example::Engine")))))
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
