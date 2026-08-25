# META
~~~ini
description=Contextual hover distinguishes declarations usages references resolution states sources and punctuation
type=file
~~~
# SOURCE
## domain.sysml
~~~sysml
package DomainTypes {
    part def Vehicle {
        doc /* Moves *people* through [the domain]. */
    }
    part def 'Véhicule';
    enum def RecordAvailabilityState {
        enum availableControlled;
    }
    item def ExternalRecordReference {
        attribute availability : RecordAvailabilityState;
    }
}
~~~
## road.sysml
~~~sysml
package Road {
    part def Vehicle;
}
~~~
## rail.sysml
~~~sysml
package Rail {
    part def Vehicle;
}
~~~
## model.sysml
~~~sysml
package Model {
    private import DomainTypes::*;
    private import Road::*;
    private import Rail::*;
    part car : DomainTypes::Vehicle;
    part local : Missing;
    part ambiguous : Vehicle;
    item maintenanceReviewRecord : DomainTypes::ExternalRecordReference {
        attribute :>> availability = DomainTypes::RecordAvailabilityState::availableControlled;
    }
    requirement def MaxAltitudeAGLReq {
        subject drone : DomainTypes::Vehicle;
    }
}
~~~
# EDITOR QUERIES
~~~text
probe domain.sysml 1 13 hover
probe domain.sysml 4 14 hover
probe model.sysml 4 9 hover
probe model.sysml 4 28 hover
probe model.sysml 5 17 hover
probe model.sysml 6 21 hover
probe model.sysml 4 35 hover
probe model.sysml 8 22 hover
probe model.sysml 11 16 hover
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/domain.sysml"
    (diagnostics
    )
  )
  (document "memory://snapshot/model.sysml"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 19) (end 1 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 17) (end 5 24))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 6 21) (end 6 28))
        (related-information
          (related
            (uri "memory://snapshot/domain.sysml")
            (range (start 1 4) (end 3 5))
          )
          (related
            (uri "memory://snapshot/rail.sysml")
            (range (start 1 4) (end 1 21))
          )
          (related
            (uri "memory://snapshot/road.sysml")
            (range (start 1 4) (end 1 21))
          )
        )
      )
    )
  )
  (document "memory://snapshot/rail.sysml"
    (diagnostics
    )
  )
  (document "memory://snapshot/road.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:a0a2d11b3f364a1879a233ec9e0dc14a9e38b05d3dd6c87003325adc49bde561") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference::availability"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RecordAvailabilityState")))))
    (declaration (id (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState::availableControlled"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Moves *people* through [the domain]. "))))
    (declaration (id (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Véhicule"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "DomainTypes") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Road") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Rail") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::MaxAltitudeAGLReq"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::MaxAltitudeAGLReq::drone"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DomainTypes::Vehicle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::ambiguous"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::car"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DomainTypes::Vehicle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::local"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Missing")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::maintenanceReviewRecord"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DomainTypes::ExternalRecordReference")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind item) (name "maintenanceReviewRecord")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "availability")) (expressionOperand (reference "DomainTypes::RecordAvailabilityState::availableControlled")))))
    (declaration (id (node (document "memory://snapshot/rail.sysml") (qualified-name "Rail"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/rail.sysml") (qualified-name "Rail::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/road.sysml") (qualified-name "Road"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/road.sysml") (qualified-name "Road::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference::availability"))) (kind featureTyping) (ordinal 0))
      (authored-target "RecordAvailabilityState")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "DomainTypes")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Road")
      (outcome (status resolved) (target (node (document "memory://snapshot/road.sysml") (qualified-name "Road")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Rail")
      (outcome (status resolved) (target (node (document "memory://snapshot/rail.sysml") (qualified-name "Rail")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::MaxAltitudeAGLReq::drone"))) (kind featureTyping) (ordinal 0))
      (authored-target "DomainTypes::Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::ambiguous"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")) (node (document "memory://snapshot/rail.sysml") (qualified-name "Rail::Vehicle")) (node (document "memory://snapshot/road.sysml") (qualified-name "Road::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::car"))) (kind featureTyping) (ordinal 0))
      (authored-target "DomainTypes::Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::local"))) (kind featureTyping) (ordinal 0))
      (authored-target "Missing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::maintenanceReviewRecord"))) (kind featureTyping) (ordinal 0))
      (authored-target "DomainTypes::ExternalRecordReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind item) (name "maintenanceReviewRecord")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "availability")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference::availability")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind item) (name "maintenanceReviewRecord")) (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "DomainTypes::RecordAvailabilityState::availableControlled")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState::availableControlled")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference::availability"))) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference::availability"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::MaxAltitudeAGLReq::drone"))) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::MaxAltitudeAGLReq::drone"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::car"))) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::car"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::maintenanceReviewRecord"))) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::maintenanceReviewRecord"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind item) (name "maintenanceReviewRecord")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference::availability"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind item) (name "maintenanceReviewRecord")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind item) (name "maintenanceReviewRecord")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState::availableControlled"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind item) (name "maintenanceReviewRecord")) (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference::availability"))) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState::availableControlled"))) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::MaxAltitudeAGLReq::drone"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::MaxAltitudeAGLReq"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind item) (name "maintenanceReviewRecord")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::maintenanceReviewRecord"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind item) (name "maintenanceReviewRecord")) (anonymous (kind attribute) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference")))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Model::maintenanceReviewRecord")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference::availability")))
      (featured-by (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference")))
      (type (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState")) (provenance authored))
      (effective-type (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState")) (source direct))
      (supertype (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind item) (name "maintenanceReviewRecord")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState")))
      (subtype (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference::availability")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState::availableControlled")))
      (featured-by (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState")))
    )
    (declaration (id (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Model::MaxAltitudeAGLReq::drone")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Model::car")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::MaxAltitudeAGLReq::drone")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Model::MaxAltitudeAGLReq")))
      (type (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::car")))
      (type (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::maintenanceReviewRecord")))
      (type (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference")) (provenance authored))
      (effective-type (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference")) (source direct))
      (supertype (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind item) (name "maintenanceReviewRecord")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Model::maintenanceReviewRecord")))
      (effective-type (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState")) (source inherited) (from (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference::availability"))))
      (supertype (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference::availability")) (scopes any feature))
      (supertype (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/domain.sysml") (range (start 9 33) (end 9 56)) (probe (position 9 33))
    (reference (id (source (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference::availability"))) (kind featureTyping) (ordinal 0) (authored-target "RecordAvailabilityState")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 33)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "DomainTypes")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 2 19) (end 2 26)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Road")
      (outcome (status resolved) (target (node (document "memory://snapshot/road.sysml") (qualified-name "Road")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 3 19) (end 3 26)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "Rail")
      (outcome (status resolved) (target (node (document "memory://snapshot/rail.sysml") (qualified-name "Rail")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 24) (end 11 44)) (probe (position 11 24))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::MaxAltitudeAGLReq::drone"))) (kind featureTyping) (ordinal 0) (authored-target "DomainTypes::Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 21) (end 6 28)) (probe (position 6 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::ambiguous"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")) (node (document "memory://snapshot/rail.sysml") (qualified-name "Rail::Vehicle")) (node (document "memory://snapshot/road.sysml") (qualified-name "Road::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 15) (end 4 35)) (probe (position 4 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::car"))) (kind featureTyping) (ordinal 0) (authored-target "DomainTypes::Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 17) (end 5 24)) (probe (position 5 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::local"))) (kind featureTyping) (ordinal 0) (authored-target "Missing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 35) (end 7 71)) (probe (position 7 35))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::maintenanceReviewRecord"))) (kind featureTyping) (ordinal 0) (authored-target "DomainTypes::ExternalRecordReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 22) (end 8 34)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind item) (name "maintenanceReviewRecord")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "availability")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::ExternalRecordReference::availability")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 37) (end 8 94)) (probe (position 8 37))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind item) (name "maintenanceReviewRecord")) (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "DomainTypes::RecordAvailabilityState::availableControlled")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::RecordAvailabilityState::availableControlled")))))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/domain.sysml") (position 1 13)
    (target (status resolved) (candidate (name "Vehicle") (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration)) (location (document "memory://snapshot/model.sysml") (range (start 4 28) (end 4 35)) (role Reference)) (location (document "memory://snapshot/model.sysml") (range (start 11 37) (end 11 44)) (role Reference))))
    (rename (status ready) (name "Vehicle") (range (start 1 13) (end 1 20)) (occurrences 3))
    (visible-members (candidates (member (name "DomainTypes") (qualified-name "DomainTypes") (kind "Package")) (member (name "ExternalRecordReference") (qualified-name "DomainTypes::ExternalRecordReference") (kind "ItemDefinition")) (member (name "Model") (qualified-name "Model") (kind "Package")) (member (name "Rail") (qualified-name "Rail") (kind "Package")) (member (name "RecordAvailabilityState") (qualified-name "DomainTypes::RecordAvailabilityState") (kind "EnumerationDefinition")) (member (name "Road") (qualified-name "Road") (kind "Package")) (member (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (kind "PartDefinition")) (member (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (kind "PartDefinition"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartDefinition")
          (name "Vehicle")
          (qualified-name "DomainTypes::Vehicle")
          (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration))
          (declaration (range (start 1 4) (end 3 5)))
          (membership (kind owning) (visibility public) (provenance default))
          (documentation (form doc) (text TextId(3)))
          (incoming (kind "typing") (peer "Model::MaxAltitudeAGLReq::drone") (provenance authored))
          (incoming (kind "typing") (peer "Model::car") (provenance authored))
        )
      )
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/domain.sysml") (position 4 14)
    (target (status resolved) (candidate (name "Véhicule") (location (document "memory://snapshot/domain.sysml") (range (start 4 14) (end 4 23)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/domain.sysml") (range (start 4 14) (end 4 23)) (role Declaration))))
    (rename (status ready) (name "Véhicule") (range (start 4 14) (end 4 23)) (occurrences 1))
    (visible-members (candidates (member (name "DomainTypes") (qualified-name "DomainTypes") (kind "Package")) (member (name "ExternalRecordReference") (qualified-name "DomainTypes::ExternalRecordReference") (kind "ItemDefinition")) (member (name "Model") (qualified-name "Model") (kind "Package")) (member (name "Rail") (qualified-name "Rail") (kind "Package")) (member (name "RecordAvailabilityState") (qualified-name "DomainTypes::RecordAvailabilityState") (kind "EnumerationDefinition")) (member (name "Road") (qualified-name "Road") (kind "Package")) (member (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (kind "PartDefinition")) (member (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (kind "PartDefinition"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartDefinition")
          (name "Véhicule")
          (qualified-name "DomainTypes::Véhicule")
          (location (document "memory://snapshot/domain.sysml") (range (start 4 14) (end 4 23)) (role Declaration))
          (declaration (range (start 4 4) (end 4 25)))
          (membership (kind owning) (visibility public) (provenance default))
        )
      )
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 4 9)
    (target (status resolved) (candidate (name "car") (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 12)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 12)) (role Declaration))))
    (rename (status ready) (name "car") (range (start 4 9) (end 4 12)) (occurrences 1))
    (visible-members (candidates (member (name "DomainTypes") (qualified-name "DomainTypes") (kind "Package")) (member (name "ExternalRecordReference") (qualified-name "DomainTypes::ExternalRecordReference") (kind "ItemDefinition")) (member (name "MaxAltitudeAGLReq") (qualified-name "Model::MaxAltitudeAGLReq") (kind "RequirementDefinition")) (member (name "Model") (qualified-name "Model") (kind "Package")) (member (name "Rail") (qualified-name "Rail") (kind "Package")) (member (name "RecordAvailabilityState") (qualified-name "DomainTypes::RecordAvailabilityState") (kind "EnumerationDefinition")) (member (name "Road") (qualified-name "Road") (kind "Package")) (member (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Rail::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Road::Vehicle") (kind "PartDefinition")) (member (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (kind "PartDefinition")) (member (name "ambiguous") (qualified-name "Model::ambiguous") (kind "PartUsage")) (member (name "car") (qualified-name "Model::car") (kind "PartUsage")) (member (name "local") (qualified-name "Model::local") (kind "PartUsage")) (member (name "maintenanceReviewRecord") (qualified-name "Model::maintenanceReviewRecord") (kind "ItemUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "car")
          (qualified-name "Model::car")
          (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 12)) (role Declaration))
          (declaration (range (start 4 4) (end 4 36)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "DomainTypes::Vehicle") (target resolved))
          (typing (outcome resolved) (target "DomainTypes::Vehicle"))
          (effective-typing (outcome resolved) (type (qualified-name "DomainTypes::Vehicle") (origin direct)))
          (outgoing (kind "typing") (peer "DomainTypes::Vehicle") (provenance authored))
        )
      )
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 4 28)
    (target (status resolved) (candidate (name "Vehicle") (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration)) (location (document "memory://snapshot/model.sysml") (range (start 4 28) (end 4 35)) (role Reference)) (location (document "memory://snapshot/model.sysml") (range (start 11 37) (end 11 44)) (role Reference))))
    (rename (status ready) (name "Vehicle") (range (start 4 28) (end 4 35)) (occurrences 3))
    (visible-members (candidates (member (name "DomainTypes") (qualified-name "DomainTypes") (kind "Package")) (member (name "ExternalRecordReference") (qualified-name "DomainTypes::ExternalRecordReference") (kind "ItemDefinition")) (member (name "MaxAltitudeAGLReq") (qualified-name "Model::MaxAltitudeAGLReq") (kind "RequirementDefinition")) (member (name "Model") (qualified-name "Model") (kind "Package")) (member (name "Rail") (qualified-name "Rail") (kind "Package")) (member (name "RecordAvailabilityState") (qualified-name "DomainTypes::RecordAvailabilityState") (kind "EnumerationDefinition")) (member (name "Road") (qualified-name "Road") (kind "Package")) (member (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Rail::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Road::Vehicle") (kind "PartDefinition")) (member (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (kind "PartDefinition")) (member (name "ambiguous") (qualified-name "Model::ambiguous") (kind "PartUsage")) (member (name "car") (qualified-name "Model::car") (kind "PartUsage")) (member (name "local") (qualified-name "Model::local") (kind "PartUsage")) (member (name "maintenanceReviewRecord") (qualified-name "Model::maintenanceReviewRecord") (kind "ItemUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "car")
          (qualified-name "Model::car")
          (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 12)) (role Declaration))
          (declaration (range (start 4 4) (end 4 36)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "DomainTypes::Vehicle") (target resolved))
          (typing (outcome resolved) (target "DomainTypes::Vehicle"))
          (effective-typing (outcome resolved) (type (qualified-name "DomainTypes::Vehicle") (origin direct)))
          (outgoing (kind "typing") (peer "DomainTypes::Vehicle") (provenance authored))
        )
      )
      (reference-kind featureTyping)
      (referenced (status resolved)
        (element (kind "PartDefinition")
          (name "Vehicle")
          (qualified-name "DomainTypes::Vehicle")
          (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration))
          (declaration (range (start 1 4) (end 3 5)))
          (membership (kind owning) (visibility public) (provenance default))
          (documentation (form doc) (text TextId(3)))
          (incoming (kind "typing") (peer "Model::MaxAltitudeAGLReq::drone") (provenance authored))
          (incoming (kind "typing") (peer "Model::car") (provenance authored))
        )
      )
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 5 17)
    (target (status unresolved))
    (rename (status unresolved))
    (visible-members (candidates (member (name "DomainTypes") (qualified-name "DomainTypes") (kind "Package")) (member (name "ExternalRecordReference") (qualified-name "DomainTypes::ExternalRecordReference") (kind "ItemDefinition")) (member (name "MaxAltitudeAGLReq") (qualified-name "Model::MaxAltitudeAGLReq") (kind "RequirementDefinition")) (member (name "Model") (qualified-name "Model") (kind "Package")) (member (name "Rail") (qualified-name "Rail") (kind "Package")) (member (name "RecordAvailabilityState") (qualified-name "DomainTypes::RecordAvailabilityState") (kind "EnumerationDefinition")) (member (name "Road") (qualified-name "Road") (kind "Package")) (member (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Rail::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Road::Vehicle") (kind "PartDefinition")) (member (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (kind "PartDefinition")) (member (name "ambiguous") (qualified-name "Model::ambiguous") (kind "PartUsage")) (member (name "car") (qualified-name "Model::car") (kind "PartUsage")) (member (name "local") (qualified-name "Model::local") (kind "PartUsage")) (member (name "maintenanceReviewRecord") (qualified-name "Model::maintenanceReviewRecord") (kind "ItemUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "local")
          (qualified-name "Model::local")
          (location (document "memory://snapshot/model.sysml") (range (start 5 9) (end 5 14)) (role Declaration))
          (declaration (range (start 5 4) (end 5 25)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Missing") (target unresolved))
          (typing (outcome unresolved))
          (effective-typing (outcome unresolved))
        )
      )
      (reference-kind featureTyping)
      (referenced (status unresolved))
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 6 21)
    (target (status ambiguous) (candidates (candidate (name "Vehicle") (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration))) (candidate (name "Vehicle") (location (document "memory://snapshot/rail.sysml") (range (start 1 13) (end 1 20)) (role Declaration))) (candidate (name "Vehicle") (location (document "memory://snapshot/road.sysml") (range (start 1 13) (end 1 20)) (role Declaration)))))
    (rename (status ambiguous) (candidates 3))
    (visible-members (candidates (member (name "DomainTypes") (qualified-name "DomainTypes") (kind "Package")) (member (name "ExternalRecordReference") (qualified-name "DomainTypes::ExternalRecordReference") (kind "ItemDefinition")) (member (name "MaxAltitudeAGLReq") (qualified-name "Model::MaxAltitudeAGLReq") (kind "RequirementDefinition")) (member (name "Model") (qualified-name "Model") (kind "Package")) (member (name "Rail") (qualified-name "Rail") (kind "Package")) (member (name "RecordAvailabilityState") (qualified-name "DomainTypes::RecordAvailabilityState") (kind "EnumerationDefinition")) (member (name "Road") (qualified-name "Road") (kind "Package")) (member (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Rail::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Road::Vehicle") (kind "PartDefinition")) (member (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (kind "PartDefinition")) (member (name "ambiguous") (qualified-name "Model::ambiguous") (kind "PartUsage")) (member (name "car") (qualified-name "Model::car") (kind "PartUsage")) (member (name "local") (qualified-name "Model::local") (kind "PartUsage")) (member (name "maintenanceReviewRecord") (qualified-name "Model::maintenanceReviewRecord") (kind "ItemUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "ambiguous")
          (qualified-name "Model::ambiguous")
          (location (document "memory://snapshot/model.sysml") (range (start 6 9) (end 6 18)) (role Declaration))
          (declaration (range (start 6 4) (end 6 29)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Vehicle") (target ambiguous 3))
          (typing (outcome ambiguous) (candidate "Rail::Vehicle") (candidate "Road::Vehicle") (candidate "DomainTypes::Vehicle"))
          (effective-typing (outcome ambiguous))
        )
      )
      (reference-kind featureTyping)
      (referenced (status ambiguous)
        (element (kind "PartDefinition")
          (name "Vehicle")
          (qualified-name "DomainTypes::Vehicle")
          (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration))
          (declaration (range (start 1 4) (end 3 5)))
          (membership (kind owning) (visibility public) (provenance default))
          (documentation (form doc) (text TextId(3)))
          (incoming (kind "typing") (peer "Model::MaxAltitudeAGLReq::drone") (provenance authored))
          (incoming (kind "typing") (peer "Model::car") (provenance authored))
        )
        (element (kind "PartDefinition")
          (name "Vehicle")
          (qualified-name "Rail::Vehicle")
          (location (document "memory://snapshot/rail.sysml") (range (start 1 13) (end 1 20)) (role Declaration))
          (declaration (range (start 1 4) (end 1 21)))
          (membership (kind owning) (visibility public) (provenance default))
        )
        (element (kind "PartDefinition")
          (name "Vehicle")
          (qualified-name "Road::Vehicle")
          (location (document "memory://snapshot/road.sysml") (range (start 1 13) (end 1 20)) (role Declaration))
          (declaration (range (start 1 4) (end 1 21)))
          (membership (kind owning) (visibility public) (provenance default))
        )
      )
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 4 35)
    (target (status resolved) (candidate (name "Vehicle") (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration)) (location (document "memory://snapshot/model.sysml") (range (start 4 28) (end 4 35)) (role Reference)) (location (document "memory://snapshot/model.sysml") (range (start 11 37) (end 11 44)) (role Reference))))
    (rename (status ready) (name "Vehicle") (range (start 4 28) (end 4 35)) (occurrences 3))
    (visible-members (candidates (member (name "DomainTypes") (qualified-name "DomainTypes") (kind "Package")) (member (name "ExternalRecordReference") (qualified-name "DomainTypes::ExternalRecordReference") (kind "ItemDefinition")) (member (name "MaxAltitudeAGLReq") (qualified-name "Model::MaxAltitudeAGLReq") (kind "RequirementDefinition")) (member (name "Model") (qualified-name "Model") (kind "Package")) (member (name "Rail") (qualified-name "Rail") (kind "Package")) (member (name "RecordAvailabilityState") (qualified-name "DomainTypes::RecordAvailabilityState") (kind "EnumerationDefinition")) (member (name "Road") (qualified-name "Road") (kind "Package")) (member (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Rail::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Road::Vehicle") (kind "PartDefinition")) (member (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (kind "PartDefinition")) (member (name "ambiguous") (qualified-name "Model::ambiguous") (kind "PartUsage")) (member (name "car") (qualified-name "Model::car") (kind "PartUsage")) (member (name "local") (qualified-name "Model::local") (kind "PartUsage")) (member (name "maintenanceReviewRecord") (qualified-name "Model::maintenanceReviewRecord") (kind "ItemUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "car")
          (qualified-name "Model::car")
          (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 12)) (role Declaration))
          (declaration (range (start 4 4) (end 4 36)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "DomainTypes::Vehicle") (target resolved))
          (typing (outcome resolved) (target "DomainTypes::Vehicle"))
          (effective-typing (outcome resolved) (type (qualified-name "DomainTypes::Vehicle") (origin direct)))
          (outgoing (kind "typing") (peer "DomainTypes::Vehicle") (provenance authored))
        )
      )
      (reference-kind featureTyping)
      (referenced (status resolved)
        (element (kind "PartDefinition")
          (name "Vehicle")
          (qualified-name "DomainTypes::Vehicle")
          (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration))
          (declaration (range (start 1 4) (end 3 5)))
          (membership (kind owning) (visibility public) (provenance default))
          (documentation (form doc) (text TextId(3)))
          (incoming (kind "typing") (peer "Model::MaxAltitudeAGLReq::drone") (provenance authored))
          (incoming (kind "typing") (peer "Model::car") (provenance authored))
        )
      )
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 8 22)
    (target (status resolved) (candidate (name "availability") (location (document "memory://snapshot/domain.sysml") (range (start 9 18) (end 9 30)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/domain.sysml") (range (start 9 18) (end 9 30)) (role Declaration)) (location (document "memory://snapshot/model.sysml") (range (start 8 22) (end 8 34)) (role Reference))))
    (rename (status ready) (name "availability") (range (start 8 22) (end 8 34)) (occurrences 2))
    (visible-members (candidates (member (name "DomainTypes") (qualified-name "DomainTypes") (kind "Package")) (member (name "ExternalRecordReference") (qualified-name "DomainTypes::ExternalRecordReference") (kind "ItemDefinition")) (member (name "MaxAltitudeAGLReq") (qualified-name "Model::MaxAltitudeAGLReq") (kind "RequirementDefinition")) (member (name "Model") (qualified-name "Model") (kind "Package")) (member (name "Rail") (qualified-name "Rail") (kind "Package")) (member (name "RecordAvailabilityState") (qualified-name "DomainTypes::RecordAvailabilityState") (kind "EnumerationDefinition")) (member (name "Road") (qualified-name "Road") (kind "Package")) (member (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Rail::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Road::Vehicle") (kind "PartDefinition")) (member (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (kind "PartDefinition")) (member (name "ambiguous") (qualified-name "Model::ambiguous") (kind "PartUsage")) (member (name "availability") (qualified-name "DomainTypes::ExternalRecordReference::availability") (kind "AttributeUsage")) (member (name "car") (qualified-name "Model::car") (kind "PartUsage")) (member (name "local") (qualified-name "Model::local") (kind "PartUsage")) (member (name "maintenanceReviewRecord") (qualified-name "Model::maintenanceReviewRecord") (kind "ItemUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "AttributeUsage")
          (qualified-name "Model::maintenanceReviewRecord::")
          (location (document "memory://snapshot/model.sysml") (range (start 8 8) (end 8 95)) (role Declaration))
          (declaration (range (start 8 8) (end 8 95)))
          (membership (kind feature) (visibility private) (provenance default))
          (value (kind bind) (default false) (operator true))
          (evaluation non-constant)
          (relationship (kind "redefinition") (provenance authored) (authored "availability") (target resolved))
          (relationship (kind "expressionOperand") (provenance authored) (authored "DomainTypes::RecordAvailabilityState::availableControlled") (target resolved))
          (relationship (kind "typeFeaturing") (provenance implied) (target resolved))
          (redefinition (outcome resolved) (target "DomainTypes::ExternalRecordReference::availability"))
          (effective-typing (outcome resolved) (type (qualified-name "DomainTypes::RecordAvailabilityState") (origin inherited)))
          (outgoing (kind "expressionOperand") (peer "DomainTypes::RecordAvailabilityState::availableControlled") (provenance authored))
          (outgoing (kind "redefinition") (peer "DomainTypes::ExternalRecordReference::availability") (provenance authored))
          (outgoing (kind "typeFeaturing") (peer "Model::maintenanceReviewRecord") (provenance implied))
        )
      )
      (reference-kind redefinition)
      (referenced (status resolved)
        (element (kind "AttributeUsage")
          (name "availability")
          (qualified-name "DomainTypes::ExternalRecordReference::availability")
          (location (document "memory://snapshot/domain.sysml") (range (start 9 18) (end 9 30)) (role Declaration))
          (declaration (range (start 9 8) (end 9 57)))
          (membership (kind feature) (visibility private) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "RecordAvailabilityState") (target resolved))
          (relationship (kind "typeFeaturing") (provenance implied) (target resolved))
          (typing (outcome resolved) (target "DomainTypes::RecordAvailabilityState"))
          (effective-typing (outcome resolved) (type (qualified-name "DomainTypes::RecordAvailabilityState") (origin direct)))
          (inherited-feature (qualified-name "DomainTypes::RecordAvailabilityState::availableControlled") (declared-in "DomainTypes::RecordAvailabilityState"))
          (incoming (kind "redefinition") (peer "Model::maintenanceReviewRecord::") (provenance authored))
          (outgoing (kind "typeFeaturing") (peer "DomainTypes::ExternalRecordReference") (provenance implied))
          (outgoing (kind "typing") (peer "DomainTypes::RecordAvailabilityState") (provenance authored))
        )
      )
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 11 16)
    (target (status resolved) (candidate (name "drone") (location (document "memory://snapshot/model.sysml") (range (start 11 16) (end 11 21)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/model.sysml") (range (start 11 16) (end 11 21)) (role Declaration))))
    (rename (status ready) (name "drone") (range (start 11 16) (end 11 21)) (occurrences 1))
    (visible-members (candidates (member (name "DomainTypes") (qualified-name "DomainTypes") (kind "Package")) (member (name "ExternalRecordReference") (qualified-name "DomainTypes::ExternalRecordReference") (kind "ItemDefinition")) (member (name "MaxAltitudeAGLReq") (qualified-name "Model::MaxAltitudeAGLReq") (kind "RequirementDefinition")) (member (name "Model") (qualified-name "Model") (kind "Package")) (member (name "Rail") (qualified-name "Rail") (kind "Package")) (member (name "RecordAvailabilityState") (qualified-name "DomainTypes::RecordAvailabilityState") (kind "EnumerationDefinition")) (member (name "Road") (qualified-name "Road") (kind "Package")) (member (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Rail::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Road::Vehicle") (kind "PartDefinition")) (member (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (kind "PartDefinition")) (member (name "ambiguous") (qualified-name "Model::ambiguous") (kind "PartUsage")) (member (name "car") (qualified-name "Model::car") (kind "PartUsage")) (member (name "drone") (qualified-name "Model::MaxAltitudeAGLReq::drone") (kind "ReferenceUsage") (role "subject")) (member (name "local") (qualified-name "Model::local") (kind "PartUsage")) (member (name "maintenanceReviewRecord") (qualified-name "Model::maintenanceReviewRecord") (kind "ItemUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "ReferenceUsage")
          (role "subject")
          (name "drone")
          (qualified-name "Model::MaxAltitudeAGLReq::drone")
          (location (document "memory://snapshot/model.sysml") (range (start 11 16) (end 11 21)) (role Declaration))
          (declaration (range (start 11 8) (end 11 45)))
          (membership (kind feature) (visibility private) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "DomainTypes::Vehicle") (target resolved))
          (relationship (kind "typeFeaturing") (provenance implied) (target resolved))
          (typing (outcome resolved) (target "DomainTypes::Vehicle"))
          (effective-typing (outcome resolved) (type (qualified-name "DomainTypes::Vehicle") (origin direct)))
          (outgoing (kind "typeFeaturing") (peer "Model::MaxAltitudeAGLReq") (provenance implied))
          (outgoing (kind "typing") (peer "DomainTypes::Vehicle") (provenance authored))
        )
      )
      (referenced (status none))
    )
  )
  (document-symbols (document "memory://snapshot/domain.sysml")
    (status resolved)
    (symbol (kind "Package") (name "DomainTypes") (qualified-name "DomainTypes") (location (document "memory://snapshot/domain.sysml") (range (start 0 8) (end 0 19)) (role Declaration)) (declaration (range (start 0 0) (end 11 1))))
    (symbol (kind "PartDefinition") (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration)) (declaration (range (start 1 4) (end 3 5))))
    (symbol (kind "PartDefinition") (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (location (document "memory://snapshot/domain.sysml") (range (start 4 14) (end 4 23)) (role Declaration)) (declaration (range (start 4 4) (end 4 25))))
    (symbol (kind "EnumerationDefinition") (name "RecordAvailabilityState") (qualified-name "DomainTypes::RecordAvailabilityState") (location (document "memory://snapshot/domain.sysml") (range (start 5 13) (end 5 36)) (role Declaration)) (declaration (range (start 5 4) (end 7 5))))
    (symbol (kind "EnumerationUsage") (name "availableControlled") (qualified-name "DomainTypes::RecordAvailabilityState::availableControlled") (location (document "memory://snapshot/domain.sysml") (range (start 6 13) (end 6 32)) (role Declaration)) (declaration (range (start 6 8) (end 6 33))))
    (symbol (kind "ItemDefinition") (name "ExternalRecordReference") (qualified-name "DomainTypes::ExternalRecordReference") (location (document "memory://snapshot/domain.sysml") (range (start 8 13) (end 8 36)) (role Declaration)) (declaration (range (start 8 4) (end 10 5))))
    (symbol (kind "AttributeUsage") (name "availability") (qualified-name "DomainTypes::ExternalRecordReference::availability") (location (document "memory://snapshot/domain.sysml") (range (start 9 18) (end 9 30)) (role Declaration)) (declaration (range (start 9 8) (end 9 57))))
  )
  (document-symbols (document "memory://snapshot/model.sysml")
    (status resolved)
    (symbol (kind "Package") (name "Model") (qualified-name "Model") (location (document "memory://snapshot/model.sysml") (range (start 0 8) (end 0 13)) (role Declaration)) (declaration (range (start 0 0) (end 13 1))))
    (symbol (kind "Import") (qualified-name "Model::") (location (document "memory://snapshot/model.sysml") (range (start 1 4) (end 1 34)) (role Declaration)) (declaration (range (start 1 4) (end 1 34))))
    (symbol (kind "Import") (qualified-name "Model::") (location (document "memory://snapshot/model.sysml") (range (start 2 4) (end 2 27)) (role Declaration)) (declaration (range (start 2 4) (end 2 27))))
    (symbol (kind "Import") (qualified-name "Model::") (location (document "memory://snapshot/model.sysml") (range (start 3 4) (end 3 27)) (role Declaration)) (declaration (range (start 3 4) (end 3 27))))
    (symbol (kind "PartUsage") (name "car") (qualified-name "Model::car") (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 12)) (role Declaration)) (declaration (range (start 4 4) (end 4 36))))
    (symbol (kind "PartUsage") (name "local") (qualified-name "Model::local") (location (document "memory://snapshot/model.sysml") (range (start 5 9) (end 5 14)) (role Declaration)) (declaration (range (start 5 4) (end 5 25))))
    (symbol (kind "PartUsage") (name "ambiguous") (qualified-name "Model::ambiguous") (location (document "memory://snapshot/model.sysml") (range (start 6 9) (end 6 18)) (role Declaration)) (declaration (range (start 6 4) (end 6 29))))
    (symbol (kind "ItemUsage") (name "maintenanceReviewRecord") (qualified-name "Model::maintenanceReviewRecord") (location (document "memory://snapshot/model.sysml") (range (start 7 9) (end 7 32)) (role Declaration)) (declaration (range (start 7 4) (end 9 5))))
    (symbol (kind "AttributeUsage") (qualified-name "Model::maintenanceReviewRecord::") (location (document "memory://snapshot/model.sysml") (range (start 8 8) (end 8 95)) (role Declaration)) (declaration (range (start 8 8) (end 8 95))))
    (symbol (kind "RequirementDefinition") (name "MaxAltitudeAGLReq") (qualified-name "Model::MaxAltitudeAGLReq") (location (document "memory://snapshot/model.sysml") (range (start 10 20) (end 10 37)) (role Declaration)) (declaration (range (start 10 4) (end 12 5))))
    (symbol (kind "ReferenceUsage") (name "drone") (qualified-name "Model::MaxAltitudeAGLReq::drone") (location (document "memory://snapshot/model.sysml") (range (start 11 16) (end 11 21)) (role Declaration)) (declaration (range (start 11 8) (end 11 45))))
  )
)
~~~
# HOVER RESULTS
~~~sexpr
(hover-reports
  (probe (document "memory://snapshot/domain.sysml") (position 1 13) (status available)
    (hover
      (identity (kind "part def") (name "Vehicle") (direct-types))
      (qualified-name "DomainTypes::Vehicle")
      (documentation "Moves *people* through [the domain].")
      (link (label "Vehicle") (uri "memory://snapshot/domain.sysml") (position 1 13))
      (link (label "DomainTypes::Vehicle") (uri "memory://snapshot/domain.sysml") (position 1 13))
    )
  )
  (probe (document "memory://snapshot/domain.sysml") (position 4 14) (status available)
    (hover
      (identity (kind "part def") (name "Véhicule") (direct-types))
      (qualified-name "DomainTypes::Véhicule")
      (link (label "Véhicule") (uri "memory://snapshot/domain.sysml") (position 4 14))
      (link (label "DomainTypes::Véhicule") (uri "memory://snapshot/domain.sysml") (position 4 14))
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 4 9) (status available)
    (hover
      (identity (kind "part") (name "car") (direct-types "DomainTypes::Vehicle"))
      (qualified-name "Model::car")
      (link (label "car") (uri "memory://snapshot/model.sysml") (position 4 9))
      (link (label "DomainTypes::Vehicle") (uri "memory://snapshot/domain.sysml") (position 1 13))
      (link (label "Model::car") (uri "memory://snapshot/model.sysml") (position 4 9))
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 4 28) (status available)
    (hover
      (context (relation "Type of") (subject "Model::car"))
      (identity (kind "part def") (name "Vehicle") (direct-types))
      (qualified-name "DomainTypes::Vehicle")
      (documentation "Moves *people* through [the domain].")
      (source (identity "memory://snapshot/domain.sysml") (line 2))
      (link (label "Model::car") (uri "memory://snapshot/model.sysml") (position 4 9))
      (link (label "Vehicle") (uri "memory://snapshot/domain.sysml") (position 1 13))
      (link (label "DomainTypes::Vehicle") (uri "memory://snapshot/domain.sysml") (position 1 13))
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 5 17) (status available)
    (hover
      (resolution (state "unresolved") (subject "type reference") (token "Missing") (explanation "Spec42 could not resolve this name in the current scope and admitted imports."))
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 6 21) (status available)
    (hover
      (resolution (state "ambiguous") (subject "type reference") (token "Vehicle"))
      (candidates "DomainTypes::Vehicle" "Rail::Vehicle" "Road::Vehicle")
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 4 35) (status none))
  (probe (document "memory://snapshot/model.sysml") (position 8 22) (status available)
    (hover
      (context (relation "Redefines"))
      (identity (kind "attribute") (name "availability") (direct-types "DomainTypes::RecordAvailabilityState"))
      (qualified-name "DomainTypes::ExternalRecordReference::availability")
      (source (identity "memory://snapshot/domain.sysml") (line 10))
      (link (label "availability") (uri "memory://snapshot/domain.sysml") (position 9 18))
      (link (label "DomainTypes::RecordAvailabilityState") (uri "memory://snapshot/domain.sysml") (position 5 13))
      (link (label "DomainTypes::ExternalRecordReference::availability") (uri "memory://snapshot/domain.sysml") (position 9 18))
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 11 16) (status available)
    (hover
      (identity (kind "ref") (role "subject") (name "drone") (direct-types "DomainTypes::Vehicle"))
      (qualified-name "Model::MaxAltitudeAGLReq::drone")
      (link (label "drone") (uri "memory://snapshot/model.sysml") (position 11 16))
      (link (label "DomainTypes::Vehicle") (uri "memory://snapshot/domain.sysml") (position 1 13))
      (link (label "Model::MaxAltitudeAGLReq::drone") (uri "memory://snapshot/model.sysml") (position 11 16))
    )
  )
)
~~~
# HOVER MARKDOWN
## domain.sysml:1:13
~~~markdown
`part def` **[Vehicle](memory://snapshot/domain.sysml#L2)**

[`DomainTypes::Vehicle`](memory://snapshot/domain.sysml#L2)

Moves \*people\* through \[the domain\]\.
~~~
## domain.sysml:4:14
~~~markdown
`part def` **[Véhicule](memory://snapshot/domain.sysml#L5)**

[`DomainTypes::Véhicule`](memory://snapshot/domain.sysml#L5)
~~~
## model.sysml:4:9
~~~markdown
`part` **[car](memory://snapshot/model.sysml#L5)**: [`DomainTypes::Vehicle`](memory://snapshot/domain.sysml#L2)

[`Model::car`](memory://snapshot/model.sysml#L5)
~~~
## model.sysml:4:28
~~~markdown
**Type of** [`Model::car`](memory://snapshot/model.sysml#L5)

`part def` **[Vehicle](memory://snapshot/domain.sysml#L2)**

[`DomainTypes::Vehicle`](memory://snapshot/domain.sysml#L2)

Moves \*people\* through \[the domain\]\.

Defined in [memory://snapshot/domain.sysml:2](memory://snapshot/domain.sysml#L2)
~~~
## model.sysml:5:17
~~~markdown
**Unresolved type reference** `Missing`

Spec42 could not resolve this name in the current scope and admitted imports.
~~~
## model.sysml:6:21
~~~markdown
**Ambiguous type reference** `Vehicle`

Candidates:
- `DomainTypes::Vehicle`
- `Rail::Vehicle`
- `Road::Vehicle`
~~~
## model.sysml:4:35
~~~markdown
~~~
## model.sysml:8:22
~~~markdown
**Redefines**

`attribute` **[availability](memory://snapshot/domain.sysml#L10)**: [`DomainTypes::RecordAvailabilityState`](memory://snapshot/domain.sysml#L6)

[`DomainTypes::ExternalRecordReference::availability`](memory://snapshot/domain.sysml#L10)

Defined in [memory://snapshot/domain.sysml:10](memory://snapshot/domain.sysml#L10)
~~~
## model.sysml:11:16
~~~markdown
`subject` **[drone](memory://snapshot/model.sysml#L12)**: [`DomainTypes::Vehicle`](memory://snapshot/domain.sysml#L2)

[`Model::MaxAltitudeAGLReq::drone`](memory://snapshot/model.sysml#L12)
~~~
