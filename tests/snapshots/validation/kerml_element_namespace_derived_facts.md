# META
~~~ini
description=KerML Root elements, relationships, annotations, namespaces and memberships derive their names, owners, members and annotations from their pinned bodies
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.2.1.2:deriveElementIsLibraryElement
rule_id=kerml-1.0:8.3.2.1.2:deriveElementName
rule_id=kerml-1.0:8.3.2.1.2:deriveElementShortName
rule_id=kerml-1.0:8.3.2.1.2:deriveElementQualifiedName
rule_id=kerml-1.0:8.3.2.1.2:deriveElementOwnedElement
rule_id=kerml-1.0:8.3.2.1.2:deriveElementOwnedAnnotation
rule_id=kerml-1.0:8.3.2.1.2:deriveOwningNamespace
rule_id=kerml-1.0:8.3.2.1.3:deriveRelationshipRelatedElement
rule_id=kerml-1.0:8.3.2.3.2:deriveAnnotatingElementAnnotatedElement
rule_id=kerml-1.0:8.3.2.3.2:deriveAnnotatingElementAnnotation
rule_id=kerml-1.0:8.3.2.3.2:deriveAnnotatingElementOwnedAnnotatingRelationship
rule_id=kerml-1.0:8.3.2.3.3:deriveAnnotationOwnedAnnotatingElement
rule_id=kerml-1.0:8.3.2.4.4:deriveMembershipImportImportedElement
rule_id=kerml-1.0:8.3.2.4.5:deriveNamespaceImportedMembership
rule_id=kerml-1.0:8.3.2.4.5:deriveNamespaceMembers
rule_id=kerml-1.0:8.3.2.4.5:deriveNamespaceOwnedMembership
rule_id=kerml-1.0:8.3.2.4.8:deriveOwningMembershipOwnedMemberName
rule_id=kerml-1.0:8.3.2.4.8:deriveOwningMembershipOwnedMemberShortName
rule_id=kerml-1.0:8.3.4.13.4:derivePackageFilterCondition
libraries=none
type=file
~~~
# SOURCE
~~~kerml
package Root {
    public import Lib::Engine;
    filter true;
    classifier <veh> Vehicle {
        doc /* A vehicle. */
        feature 'max speed';
        feature mass;
    }
    classifier Car :> Vehicle {
        feature :>> mass;
    }
    alias Auto for Car;
    comment note about Vehicle /* Note on vehicles. */
    dependency trace from Car to Vehicle {
        doc /* Why cars trace to vehicles. */
    }
}
library package Lib {
    classifier Engine;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.1.2:deriveElementIsLibraryElement") (source "Lib::Engine") (value true) (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.1.2:deriveElementIsLibraryElement") (source "Root::Vehicle") (value false) (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.1.2:deriveElementName") (source (anonymous (owner "Root::Car") (kind Feature) (ordinal 0))) (value "mass") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.1.2:deriveElementShortName") (source "Root::Vehicle") (value "veh") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.1.2:deriveElementQualifiedName") (source "Root::Vehicle::max speed") (value "Root::Vehicle::'max speed'") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.1.2:deriveElementQualifiedName") (source (anonymous (owner "Root::Car") (kind Feature) (ordinal 0))) (value "Root::Car::mass") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.1.2:deriveElementOwnedElement") (source "Root::Vehicle") (target "Root::Vehicle::mass") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.1.2:deriveElementOwnedAnnotation") (source "Root::trace") (target (anonymous (owner "Root::trace") (kind Documentation) (ordinal 0))) (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.1.2:deriveElementOwnedAnnotation") (source "Root::Vehicle") (outcome absent))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.1.2:deriveOwningNamespace") (source "Root::Vehicle::mass") (target "Root::Vehicle") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.1.2:deriveOwningNamespace") (source (anonymous (owner "Root::trace") (kind Documentation) (ordinal 0))) (outcome absent))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.1.3:deriveRelationshipRelatedElement") (source "Root::trace") (target "Root::Car") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.1.3:deriveRelationshipRelatedElement") (source "Root::trace") (target "Root::Vehicle") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.3.2:deriveAnnotatingElementAnnotatedElement") (source "Root::note") (target "Root::Vehicle") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.3.2:deriveAnnotatingElementAnnotatedElement") (source (anonymous (owner "Root::Vehicle") (kind Documentation) (ordinal 0))) (target "Root::Vehicle") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.3.2:deriveAnnotatingElementAnnotation") (source "Root::note") (target "Root::Vehicle") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.3.2:deriveAnnotatingElementAnnotation") (source (anonymous (owner "Root::trace") (kind Documentation) (ordinal 0))) (target "Root::trace") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.3.2:deriveAnnotatingElementOwnedAnnotatingRelationship") (source "Root::note") (target "Root::Vehicle") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.3.2:deriveAnnotatingElementOwnedAnnotatingRelationship") (source (anonymous (owner "Root::trace") (kind Documentation) (ordinal 0))) (outcome absent))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.3.3:deriveAnnotationOwnedAnnotatingElement") (source (anonymous (owner "Root::trace") (kind Documentation) (ordinal 0))) (target (anonymous (owner "Root::trace") (kind Documentation) (ordinal 0))) (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.3.3:deriveAnnotationOwnedAnnotatingElement") (source "Root::note") (outcome absent))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.4.4:deriveMembershipImportImportedElement") (source (anonymous (owner "Root") (kind Import) (ordinal 0))) (target "Lib::Engine") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.4.5:deriveNamespaceImportedMembership") (source "Root") (target "Lib::Engine") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.4.5:deriveNamespaceOwnedMembership") (source "Root") (target "Root::Car") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.4.5:deriveNamespaceMembers") (source "Root::Car") (target "Root::Vehicle::max speed") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.4.5:deriveNamespaceMembers") (source "Root") (target "Lib::Engine") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.4.8:deriveOwningMembershipOwnedMemberName") (source "Root::Vehicle::mass") (value "mass") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.2.4.8:deriveOwningMembershipOwnedMemberShortName") (source "Root::Vehicle") (value "veh") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.4.13.4:derivePackageFilterCondition") (source "Root") (position 1) (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.4.13.4:derivePackageFilterCondition") (source "Lib") (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_element_namespace_derived_facts.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:84919704ac87623f718033dee131c25c2949319f79aeba7eea04bc35746f9d5f"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Lib"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Lib::Engine"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (path (named (kind package) (name "Root")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "Lib::Engine") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (path (named (kind package) (name "Root")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (kind kerml-literal-boolean) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Auto"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "Car")))))
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Car"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (path (named (kind package) (name "Root")) (named (kind kerml-classifier) (name "Car")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (effective-identification (name "mass") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass")))))
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (short-name "veh")))
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (path (named (kind package) (name "Root")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "A vehicle. "))))
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle::mass"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle::max speed"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::note"))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "Note on vehicles. "))) (authored (membership (kind owning) (visibility default)) (relationships (annotation (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::trace"))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "Car")) (dependencySupplier (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (path (named (kind package) (name "Root")) (named (kind dependency) (name "trace")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "Why cars trace to vehicles. "))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (path (named (kind package) (name "Root")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Lib::Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Lib::Engine")))))
    (reference (id (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Auto"))) (kind aliasBinding) (ordinal 0))
      (authored-target "Car")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Car")))))
    (reference (id (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Car"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (path (named (kind package) (name "Root")) (named (kind kerml-classifier) (name "Car")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle::mass")))))
    (reference (id (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::note"))) (kind annotation) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::trace"))) (kind dependencyClient) (ordinal 0))
      (authored-target "Car")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Car")))))
    (reference (id (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::trace"))) (kind dependencySupplier) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle")))))
  )
  (relationships
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Auto"))) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Car"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Auto"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Car"))) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Car"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (path (named (kind package) (name "Root")) (named (kind kerml-classifier) (name "Car")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle::mass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (path (named (kind package) (name "Root")) (named (kind kerml-classifier) (name "Car")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind annotation) (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::note"))) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::note"))) (kind annotation) (ordinal 0)))
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::trace"))) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Car"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::trace"))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::trace"))) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::trace"))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (path (named (kind package) (name "Root")) (named (kind kerml-classifier) (name "Car")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Car"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle::mass"))) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle::max speed"))) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle"))) (provenance implied))
  )
  (evaluation
    (filter (owner (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root"))) (form package-import) (state literal) (start 2 11) (end 2 15) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Car")))
      (supertype (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (path (named (kind package) (name "Root")) (named (kind kerml-classifier) (name "Car")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Car")))
      (supertype (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle::mass")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle")))
      (subtype (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Car")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle")))
      (subtype (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (path (named (kind package) (name "Root")) (named (kind kerml-classifier) (name "Car")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle::max speed")))
      (featured-by (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (range (start 1 18) (end 1 29)) (probe (position 1 18))
    (reference (id (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (path (named (kind package) (name "Root")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Lib::Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Lib::Engine")))))
    )
  )
  (query (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (range (start 11 19) (end 11 22)) (probe (position 11 19))
    (reference (id (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Auto"))) (kind aliasBinding) (ordinal 0) (authored-target "Car")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Car")))))
    )
  )
  (query (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (range (start 8 22) (end 8 29)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Car"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (range (start 9 20) (end 9 24)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (path (named (kind package) (name "Root")) (named (kind kerml-classifier) (name "Car")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle::mass")))))
    )
  )
  (query (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (range (start 12 23) (end 12 30)) (probe (position 12 23))
    (reference (id (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::note"))) (kind annotation) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (range (start 13 26) (end 13 29)) (probe (position 13 26))
    (reference (id (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::trace"))) (kind dependencyClient) (ordinal 0) (authored-target "Car")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Car")))))
    )
  )
  (query (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (range (start 13 33) (end 13 40)) (probe (position 13 33))
    (reference (id (source (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::trace"))) (kind dependencySupplier) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_namespace_derived_facts.md") (qualified-name "Root::Vehicle")))))
    )
  )
)
~~~
