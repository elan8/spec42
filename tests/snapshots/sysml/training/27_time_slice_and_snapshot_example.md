# META
~~~ini
description=SysML Training 27 (Occurrences): Time Slice and Snapshot Example
type=file
~~~
# SOURCE
~~~sysml
package 'Time Slice and Snapshot Example' {
		
	attribute def Date;
	item def Person;
	
	part def Vehicle {
		timeslice assembly;
		
		first assembly then delivery;
		
		snapshot delivery {
			attribute deliveryDate : Date;
		}
		
		then timeslice ownership[0..*] ordered {
			snapshot sale = start;
			
			ref item owner : Person[1];
			
			timeslice driven[0..*] {
				ref item driver : Person[1];
			}
		}
		
		snapshot junked = done;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/27_time_slice_and_snapshot_example.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b57bc12caa5547fc29077dd18653ef97509612138b3b83668984234a9e289536") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Date"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Person"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (path (named (kind package) (name "Time Slice and Snapshot Example")) (named (kind part-def) (name "Vehicle")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "assembly")) (succession (reference "delivery")))))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::assembly"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion timeslice)))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Date")))))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::junked"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion timeslice) (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion timeslice) (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven::driver"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers reference) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::owner"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers reference) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::sale"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)) (feature-value (kind bind)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (path (named (kind package) (name "Time Slice and Snapshot Example")) (named (kind part-def) (name "Vehicle")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::assembly")))))
    (reference (id (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (path (named (kind package) (name "Time Slice and Snapshot Example")) (named (kind part-def) (name "Vehicle")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "delivery")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery")))))
    (reference (id (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (kind featureTyping) (ordinal 0))
      (authored-target "Date")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Date")))))
    (reference (id (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Person")))))
    (reference (id (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::owner"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Person")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (path (named (kind package) (name "Time Slice and Snapshot Example")) (named (kind part-def) (name "Vehicle")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::assembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (path (named (kind package) (name "Time Slice and Snapshot Example")) (named (kind part-def) (name "Vehicle")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (path (named (kind package) (name "Time Slice and Snapshot Example")) (named (kind part-def) (name "Vehicle")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (path (named (kind package) (name "Time Slice and Snapshot Example")) (named (kind part-def) (name "Vehicle")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Date"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven::driver"))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::owner"))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::owner"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (path (named (kind package) (name "Time Slice and Snapshot Example")) (named (kind part-def) (name "Vehicle")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::assembly"))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery"))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::junked"))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership"))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven"))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven::driver"))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::owner"))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::sale"))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Date")))
      (subtype (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Person")))
      (subtype (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven::driver")) (scopes any))
      (subtype (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::owner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (path (named (kind package) (name "Time Slice and Snapshot Example")) (named (kind part-def) (name "Vehicle")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::assembly")))
      (featured-by (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery")))
      (featured-by (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate")))
      (featured-by (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery")))
      (type (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Date")) (provenance authored))
      (effective-type (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Date")) (source direct))
      (supertype (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Date")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::junked")))
      (featured-by (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership")))
      (featured-by (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven")))
      (featured-by (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership")))
    )
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven::driver")))
      (featured-by (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven")))
      (type (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Person")) (source direct))
      (supertype (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::owner")))
      (featured-by (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership")))
      (type (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Person")) (source direct))
      (supertype (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::sale")))
      (featured-by (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (range (start 8 8) (end 8 16)) (probe (position 8 8))
    (reference (id (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (path (named (kind package) (name "Time Slice and Snapshot Example")) (named (kind part-def) (name "Vehicle")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::assembly")))))
    )
  )
  (query (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (range (start 8 22) (end 8 30)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (path (named (kind package) (name "Time Slice and Snapshot Example")) (named (kind part-def) (name "Vehicle")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "delivery")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery")))))
    )
  )
  (query (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (range (start 11 28) (end 11 32)) (probe (position 11 28))
    (reference (id (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (kind featureTyping) (ordinal 0) (authored-target "Date")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Date")))))
    )
  )
  (query (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (range (start 20 22) (end 20 28)) (probe (position 20 22))
    (reference (id (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Person")))))
    )
  )
  (query (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (range (start 17 20) (end 17 26)) (probe (position 17 20))
    (reference (id (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::owner"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Person")))))
    )
  )
)
~~~
