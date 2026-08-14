# META
~~~ini
description=KerML KerML Spec Annex A: A-2-ModelingInstances
type=file
~~~
# SOURCE
~~~kerml
package ModelingInstances {
	doc
	/* 
	 */

	classifier Vehicle;
	classifier Bicycle specializes Vehicle;
	classifier MyBike [1] specializes Bicycle;
	classifier YourBike [1] specializes Bicycle disjoint from MyBike;
}

package ModelingInstancesWithAtoms {
	doc
	/* 
	 */

	private import Atoms::atom;

	classifier Vehicle;
	classifier Bicycle specializes Vehicle;

	#atom
	classifier MyBike specializes Bicycle;
	#atom
	classifier YourBike specializes Bicycle;

	/* Assigning feature values. */

	classifier Garage {
		feature stores : Bicycle [*];
	}
	classifier OurBicycle unions MyBike, YourBike;

	#atom
	classifier OurGarage specializes Garage {
		feature redefines stores : OurBicycle [2];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/a_2_modeling_instances.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 1) (end 5 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 6 32) (end 6 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 18 1) (end 18 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 19 32) (end 19 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 21 1) (end 22 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 23 1) (end 24 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 33 1) (end 34 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:97dd5c0e74303f65eb1d7f699eefcbf0ea890c8cdd99ab55731c0c854f9376e5") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text " \n\t "))))
    (declaration (id (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::Bicycle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::MyBike"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Bicycle"))))
    (declaration (id (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::YourBike"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Bicycle"))))
    (declaration (id (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text " \n\t "))))
    (declaration (id (node (document "memory://snapshot/a_2_modeling_instances.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Atoms::atom") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Bicycle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Garage"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Garage::stores"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bicycle"))))
    (declaration (id (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::MyBike"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Bicycle"))))
    (declaration (id (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::OurBicycle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::OurGarage"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Garage"))))
    (declaration (id (node (document "memory://snapshot/a_2_modeling_instances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "OurBicycle")) (redefinition (reference "stores"))))
    (declaration (id (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::YourBike"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Bicycle"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::Bicycle"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::MyBike"))) (kind specialization) (ordinal 0))
      (authored-target "Bicycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::Bicycle")))))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::YourBike"))) (kind specialization) (ordinal 0))
      (authored-target "Bicycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::Bicycle")))))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Atoms::atom")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Bicycle"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Garage::stores"))) (kind featureTyping) (ordinal 0))
      (authored-target "Bicycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Bicycle")))))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::MyBike"))) (kind specialization) (ordinal 0))
      (authored-target "Bicycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Bicycle")))))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::OurGarage"))) (kind specialization) (ordinal 0))
      (authored-target "Garage")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Garage")))))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "OurBicycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::OurBicycle")))))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "stores")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Garage::stores")))))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::YourBike"))) (kind specialization) (ordinal 0))
      (authored-target "Bicycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Bicycle")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::MyBike"))) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::Bicycle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::MyBike"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::YourBike"))) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::Bicycle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::YourBike"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Garage::stores"))) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Bicycle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Garage::stores"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::MyBike"))) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Bicycle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::MyBike"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::OurGarage"))) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Garage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::OurGarage"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/a_2_modeling_instances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::OurBicycle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_2_modeling_instances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/a_2_modeling_instances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Garage::stores"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_2_modeling_instances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::YourBike"))) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Bicycle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::YourBike"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/a_2_modeling_instances.md") (range (start 6 32) (end 6 39)) (probe (position 6 32))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::Bicycle"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_2_modeling_instances.md") (range (start 7 35) (end 7 42)) (probe (position 7 35))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::MyBike"))) (kind specialization) (ordinal 0) (authored-target "Bicycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::Bicycle")))))
  )
  (query (document "memory://snapshot/a_2_modeling_instances.md") (range (start 8 37) (end 8 44)) (probe (position 8 37))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::YourBike"))) (kind specialization) (ordinal 0) (authored-target "Bicycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstances::Bicycle")))))
  )
  (query (document "memory://snapshot/a_2_modeling_instances.md") (range (start 16 16) (end 16 27)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Atoms::atom")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_2_modeling_instances.md") (range (start 19 32) (end 19 39)) (probe (position 19 32))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Bicycle"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_2_modeling_instances.md") (range (start 29 19) (end 29 26)) (probe (position 29 19))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Garage::stores"))) (kind featureTyping) (ordinal 0) (authored-target "Bicycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Bicycle")))))
  )
  (query (document "memory://snapshot/a_2_modeling_instances.md") (range (start 22 31) (end 22 38)) (probe (position 22 31))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::MyBike"))) (kind specialization) (ordinal 0) (authored-target "Bicycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Bicycle")))))
  )
  (query (document "memory://snapshot/a_2_modeling_instances.md") (range (start 34 34) (end 34 40)) (probe (position 34 34))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::OurGarage"))) (kind specialization) (ordinal 0) (authored-target "Garage")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Garage")))))
  )
  (query (document "memory://snapshot/a_2_modeling_instances.md") (range (start 35 29) (end 35 39)) (probe (position 35 29))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "OurBicycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::OurBicycle")))))
  )
  (query (document "memory://snapshot/a_2_modeling_instances.md") (range (start 35 20) (end 35 26)) (probe (position 35 20))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "stores")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Garage::stores")))))
  )
  (query (document "memory://snapshot/a_2_modeling_instances.md") (range (start 24 33) (end 24 40)) (probe (position 24 33))
    (reference (id (source (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::YourBike"))) (kind specialization) (ordinal 0) (authored-target "Bicycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_modeling_instances.md") (qualified-name "ModelingInstancesWithAtoms::Bicycle")))))
  )
)
~~~
