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
  (document "memory://snapshot/07_parts_example_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 17) (end 17 20))
      )
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
        (range (start 23 17) (end 23 20))
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ec5764e5c5d30cc47669fbf8848299e73b1d6f9d78ba82f37659233c003c432a") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Cylinder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::bigVehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle")))))
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "eng")))))
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 6) (upper 6))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "cyl")))))
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::smallVehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle")))))
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "eng")))))
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 4) (upper 4))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "cyl")))))
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng::cyl"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 4) (upper 6))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Cylinder")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::bigVehicle"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "eng")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "cyl")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::smallVehicle"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "eng")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "cyl")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Engine")))))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng::cyl"))) (kind featureTyping) (ordinal 0))
      (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Cylinder")))))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::bigVehicle"))) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::bigVehicle"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::smallVehicle"))) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::smallVehicle"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle"))) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng"))) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng::cyl"))) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Cylinder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng::cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::bigVehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::smallVehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng"))) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng::cyl"))) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Cylinder")))
      (subtype (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng::cyl")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Engine")))
      (subtype (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Vehicle")))
      (subtype (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::bigVehicle")))
      (effective-type (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Vehicle")) (source inherited) (from (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle"))))
      (supertype (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Vehicle")) (scopes any))
      (supertype (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::bigVehicle")))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::smallVehicle")))
      (effective-type (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Vehicle")) (source inherited) (from (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle"))))
      (supertype (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Vehicle")) (scopes any))
      (supertype (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::smallVehicle")))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle")))
      (type (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Vehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::bigVehicle")) (scopes any feature))
      (subtype (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::smallVehicle")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng")))
      (featured-by (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle")))
      (type (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng::cyl")))
      (featured-by (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng")))
      (type (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Cylinder")) (provenance authored))
      (effective-type (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Cylinder")) (source direct))
      (supertype (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Cylinder")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/07_parts_example_2.md") (range (start 22 20) (end 22 27)) (probe (position 22 20))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::bigVehicle"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle")))))
    )
  )
  (query (document "memory://snapshot/07_parts_example_2.md") (range (start 23 17) (end 23 20)) (probe (position 23 17))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "eng")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/07_parts_example_2.md") (range (start 24 18) (end 24 21)) (probe (position 24 18))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "bigVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "cyl")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/07_parts_example_2.md") (range (start 16 22) (end 16 29)) (probe (position 16 22))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::smallVehicle"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle")))))
    )
  )
  (query (document "memory://snapshot/07_parts_example_2.md") (range (start 17 17) (end 17 20)) (probe (position 17 17))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "eng")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/07_parts_example_2.md") (range (start 18 18) (end 18 21)) (probe (position 18 18))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (path (named (kind package) (name "Parts Example-2")) (named (kind part) (name "smallVehicle")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "cyl")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/07_parts_example_2.md") (range (start 10 16) (end 10 23)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/07_parts_example_2.md") (range (start 11 13) (end 11 19)) (probe (position 11 13))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Engine")))))
    )
  )
  (query (document "memory://snapshot/07_parts_example_2.md") (range (start 12 14) (end 12 22)) (probe (position 12 14))
    (reference (id (source (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::vehicle::eng::cyl"))) (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/07_parts_example_2.md") (qualified-name "Parts Example-2::Cylinder")))))
    )
  )
)
~~~
