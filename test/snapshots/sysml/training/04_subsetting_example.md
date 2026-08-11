# META
~~~ini
description=SysML Training 04 (Subsetting): Subsetting Example
type=file
~~~
# SOURCE
~~~sysml
package 'Subsetting Example' {
	
	part def Vehicle {
		part parts : VehiclePart[*];
		
		part eng : Engine subsets parts;
		part trans : Transmission subsets parts;
		part wheels : Wheel[4] :> parts;
	}
	
	abstract part def VehiclePart;
	part def Engine :> VehiclePart;
	part def Transmission :> VehiclePart;
	part def Wheel :> VehiclePart;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "04_subsetting_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Subsetting Example' {

    part def Vehicle {
        part parts : VehiclePart[*];

        part eng : Engine subsets parts;
        part trans : Transmission subsets parts;
        part wheels : Wheel[4] :> parts;
    }

    abstract part def VehiclePart;
    part def Engine :> VehiclePart;
    part def Transmission :> VehiclePart;
    part def Wheel :> VehiclePart;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "631877bfc3a795b00a6e5671d3d35bbda12c90535165b777a6feedffb32244d2") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Subsetting Example"))) (kind "package") (name "Subsetting Example") (declared-name "Subsetting Example") (range (start (line 0) (character 0)) (end (line 0) (character 342))))
    (element (id (node (document "d0") (qualified-name "Subsetting Example::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 11) (character 1)) (end (line 11) (character 32))) (parent (node (document "d0") (qualified-name "Subsetting Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehiclePart") (range (start (line 11) (character 20)) (end (line 11) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "Subsetting Example::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (range (start (line 12) (character 1)) (end (line 12) (character 38))) (parent (node (document "d0") (qualified-name "Subsetting Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehiclePart") (range (start (line 12) (character 26)) (end (line 12) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "Subsetting Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 2) (character 1)) (end (line 2) (character 169))) (parent (node (document "d0") (qualified-name "Subsetting Example"))))
    (element (id (node (document "d0") (qualified-name "Subsetting Example::Vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 5) (character 2)) (end (line 5) (character 34))) (parent (node (document "d0") (qualified-name "Subsetting Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 5) (character 13)) (end (line 5) (character 19)))) (subsetting (reference "parts") (range (start (line 5) (character 28)) (end (line 5) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Subsetting Example::Vehicle::parts"))) (kind "part") (name "parts") (declared-name "parts") (range (start (line 3) (character 2)) (end (line 3) (character 30))) (parent (node (document "d0") (qualified-name "Subsetting Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehiclePart") (range (start (line 3) (character 15)) (end (line 3) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "Subsetting Example::Vehicle::trans"))) (kind "part") (name "trans") (declared-name "trans") (range (start (line 6) (character 2)) (end (line 6) (character 42))) (parent (node (document "d0") (qualified-name "Subsetting Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission") (range (start (line 6) (character 15)) (end (line 6) (character 27)))) (subsetting (reference "parts") (range (start (line 6) (character 36)) (end (line 6) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "Subsetting Example::Vehicle::wheels"))) (kind "part") (name "wheels") (declared-name "wheels") (range (start (line 7) (character 2)) (end (line 7) (character 34))) (parent (node (document "d0") (qualified-name "Subsetting Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 7) (character 16)) (end (line 7) (character 21)))) (subsetting (reference "parts") (range (start (line 7) (character 28)) (end (line 7) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Subsetting Example::VehiclePart"))) (kind "part def") (name "VehiclePart") (declared-name "VehiclePart") (range (start (line 10) (character 1)) (end (line 10) (character 31))) (parent (node (document "d0") (qualified-name "Subsetting Example"))))
    (element (id (node (document "d0") (qualified-name "Subsetting Example::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (range (start (line 13) (character 1)) (end (line 13) (character 31))) (parent (node (document "d0") (qualified-name "Subsetting Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehiclePart") (range (start (line 13) (character 19)) (end (line 13) (character 30)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Subsetting Example::Engine"))) (kind specialization) (ordinal 0)) (authored-target "VehiclePart") (range (start (line 11) (character 20)) (end (line 11) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Subsetting Example::VehiclePart")))))
    (reference (id (source (node (document "d0") (qualified-name "Subsetting Example::Transmission"))) (kind specialization) (ordinal 0)) (authored-target "VehiclePart") (range (start (line 12) (character 26)) (end (line 12) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Subsetting Example::VehiclePart")))))
    (reference (id (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 5) (character 13)) (end (line 5) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Subsetting Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::eng"))) (kind subsetting) (ordinal 0)) (authored-target "parts") (range (start (line 5) (character 28)) (end (line 5) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Subsetting Example::Vehicle::parts")))))
    (reference (id (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::parts"))) (kind featureTyping) (ordinal 0)) (authored-target "VehiclePart") (range (start (line 3) (character 15)) (end (line 3) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Subsetting Example::VehiclePart")))))
    (reference (id (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::trans"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (range (start (line 6) (character 15)) (end (line 6) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Subsetting Example::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::trans"))) (kind subsetting) (ordinal 0)) (authored-target "parts") (range (start (line 6) (character 36)) (end (line 6) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Subsetting Example::Vehicle::parts")))))
    (reference (id (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::wheels"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 7) (character 16)) (end (line 7) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Subsetting Example::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::wheels"))) (kind subsetting) (ordinal 0)) (authored-target "parts") (range (start (line 7) (character 28)) (end (line 7) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Subsetting Example::Vehicle::parts")))))
    (reference (id (source (node (document "d0") (qualified-name "Subsetting Example::Wheel"))) (kind specialization) (ordinal 0)) (authored-target "VehiclePart") (range (start (line 13) (character 19)) (end (line 13) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Subsetting Example::VehiclePart")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Subsetting Example::Engine"))) (target (node (document "d0") (qualified-name "Subsetting Example::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Subsetting Example::Engine"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Subsetting Example::Transmission"))) (target (node (document "d0") (qualified-name "Subsetting Example::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Subsetting Example::Transmission"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::eng"))) (target (node (document "d0") (qualified-name "Subsetting Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::eng"))) (target (node (document "d0") (qualified-name "Subsetting Example::Vehicle::parts"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::eng"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::parts"))) (target (node (document "d0") (qualified-name "Subsetting Example::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::parts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::trans"))) (target (node (document "d0") (qualified-name "Subsetting Example::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::trans"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::trans"))) (target (node (document "d0") (qualified-name "Subsetting Example::Vehicle::parts"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::trans"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::wheels"))) (target (node (document "d0") (qualified-name "Subsetting Example::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::wheels"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::wheels"))) (target (node (document "d0") (qualified-name "Subsetting Example::Vehicle::parts"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Subsetting Example::Vehicle::wheels"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Subsetting Example::Wheel"))) (target (node (document "d0") (qualified-name "Subsetting Example::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Subsetting Example::Wheel"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 5 28) (end 5 33)) (probe (position 5 28))
      (reference
        (source (document "d0") (qualified-name "Subsetting Example::Vehicle::eng"))
        (kind subsetting) (ordinal 0) (authored-target "parts")
        (range (start 5 28) (end 5 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Subsetting Example::Vehicle::parts") (range (start 3 2) (end 3 30)))
        )
      )
    )
    (query (range (start 6 36) (end 6 41)) (probe (position 6 36))
      (reference
        (source (document "d0") (qualified-name "Subsetting Example::Vehicle::trans"))
        (kind subsetting) (ordinal 0) (authored-target "parts")
        (range (start 6 36) (end 6 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Subsetting Example::Vehicle::parts") (range (start 3 2) (end 3 30)))
        )
      )
    )
    (query (range (start 7 16) (end 7 21)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Subsetting Example::Vehicle::wheels"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 7 16) (end 7 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Subsetting Example::Wheel") (range (start 13 1) (end 13 31)))
        )
      )
    )
    (query (range (start 7 28) (end 7 33)) (probe (position 7 28))
      (reference
        (source (document "d0") (qualified-name "Subsetting Example::Vehicle::wheels"))
        (kind subsetting) (ordinal 0) (authored-target "parts")
        (range (start 7 28) (end 7 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Subsetting Example::Vehicle::parts") (range (start 3 2) (end 3 30)))
        )
      )
    )
    (query (range (start 5 13) (end 5 19)) (probe (position 5 13))
      (reference
        (source (document "d0") (qualified-name "Subsetting Example::Vehicle::eng"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 5 13) (end 5 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Subsetting Example::Engine") (range (start 11 1) (end 11 32)))
        )
      )
    )
    (query (range (start 3 15) (end 3 26)) (probe (position 3 15))
      (reference
        (source (document "d0") (qualified-name "Subsetting Example::Vehicle::parts"))
        (kind featureTyping) (ordinal 0) (authored-target "VehiclePart")
        (range (start 3 15) (end 3 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Subsetting Example::VehiclePart") (range (start 10 1) (end 10 31)))
        )
      )
    )
    (query (range (start 11 20) (end 11 31)) (probe (position 11 20))
      (reference
        (source (document "d0") (qualified-name "Subsetting Example::Engine"))
        (kind specialization) (ordinal 0) (authored-target "VehiclePart")
        (range (start 11 20) (end 11 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Subsetting Example::VehiclePart") (range (start 10 1) (end 10 31)))
        )
      )
    )
    (query (range (start 12 26) (end 12 37)) (probe (position 12 26))
      (reference
        (source (document "d0") (qualified-name "Subsetting Example::Transmission"))
        (kind specialization) (ordinal 0) (authored-target "VehiclePart")
        (range (start 12 26) (end 12 37))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Subsetting Example::VehiclePart") (range (start 10 1) (end 10 31)))
        )
      )
    )
    (query (range (start 13 19) (end 13 30)) (probe (position 13 19))
      (reference
        (source (document "d0") (qualified-name "Subsetting Example::Wheel"))
        (kind specialization) (ordinal 0) (authored-target "VehiclePart")
        (range (start 13 19) (end 13 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Subsetting Example::VehiclePart") (range (start 10 1) (end 10 31)))
        )
      )
    )
    (query (range (start 6 15) (end 6 27)) (probe (position 6 15))
      (reference
        (source (document "d0") (qualified-name "Subsetting Example::Vehicle::trans"))
        (kind featureTyping) (ordinal 0) (authored-target "Transmission")
        (range (start 6 15) (end 6 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Subsetting Example::Transmission") (range (start 12 1) (end 12 38)))
        )
      )
    )
  )
)
~~~
