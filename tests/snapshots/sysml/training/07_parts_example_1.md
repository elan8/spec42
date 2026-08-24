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
  (document "memory://snapshot/07_parts_example_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 18) (end 18 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 24 18) (end 24 21))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:4946eff004bc5e86d779f136e38e38bb319ff98e69a3b6fda921b62b4c176675") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Cylinder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine::cyl"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 4) (upper 6))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Cylinder")))))
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::bigVehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "eng")))))
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 6) (upper 6))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "cyl")))))
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::smallVehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "eng")))))
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 4) (upper 4))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "cyl")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine::cyl"))) (kind featureTyping) (ordinal 0))
      (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Cylinder")))))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine")))))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::bigVehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "eng")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng")))))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "cyl")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::smallVehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "eng")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng")))))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "cyl")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine::cyl"))) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Cylinder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine::cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng"))) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::bigVehicle"))) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::bigVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::smallVehicle"))) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::smallVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine::cyl"))) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng"))) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::bigVehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::smallVehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Cylinder")))
      (subtype (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine::cyl")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine")))
      (subtype (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine::cyl")))
      (featured-by (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine")))
      (type (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Cylinder")) (provenance authored))
      (effective-type (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Cylinder")) (source direct))
      (supertype (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Cylinder")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle")))
      (subtype (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::bigVehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::smallVehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng")))
      (featured-by (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle")))
      (type (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine")) (scopes any))
      (subtype (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::bigVehicle")))
      (type (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::bigVehicle")))
      (effective-type (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine")) (source inherited) (from (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng"))))
      (supertype (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine")) (scopes any))
      (supertype (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::smallVehicle")))
      (type (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::smallVehicle")))
      (effective-type (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine")) (source inherited) (from (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng"))))
      (supertype (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine")) (scopes any))
      (supertype (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/07_parts_example_1.md") (range (start 9 13) (end 9 21)) (probe (position 9 13))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine::cyl"))) (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Cylinder")))))
    )
  )
  (query (document "memory://snapshot/07_parts_example_1.md") (range (start 5 13) (end 5 19)) (probe (position 5 13))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Engine")))))
    )
  )
  (query (document "memory://snapshot/07_parts_example_1.md") (range (start 22 19) (end 22 26)) (probe (position 22 19))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::bigVehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/07_parts_example_1.md") (range (start 23 17) (end 23 20)) (probe (position 23 17))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "eng")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng")))))
    )
  )
  (query (document "memory://snapshot/07_parts_example_1.md") (range (start 24 18) (end 24 21)) (probe (position 24 18))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "cyl")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/07_parts_example_1.md") (range (start 16 21) (end 16 28)) (probe (position 16 21))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::smallVehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/07_parts_example_1.md") (range (start 17 17) (end 17 20)) (probe (position 17 17))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "eng")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_1.md") (qualified-name "Parts Example-1::Vehicle::eng")))))
    )
  )
  (query (document "memory://snapshot/07_parts_example_1.md") (range (start 18 18) (end 18 21)) (probe (position 18 18))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_1.md") (path (named (kind package) (name "Parts Example-1")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "cyl")
      (outcome (status unresolved)))
    )
  )
)
~~~
