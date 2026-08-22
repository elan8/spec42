# META
~~~ini
description=Subject roles, satisfy endpoints, verify membership and include targets
type=file
~~~
# SOURCE
~~~sysml
package RequirementsAndCases {
    part def Vehicle;
    part vehicle : Vehicle;
    requirement def Limit;
    requirement limit : Limit;
    use case def Drive;
    use case drive : Drive;

    part def ConformingSatisfy {
        satisfy limit;
    }

    part def SatisfiesSomethingThatIsNotARequirement {
        satisfy vehicle;
    }

    requirement def TwoSubjects {
        subject first : Vehicle;
        subject second : Vehicle;
    }

    requirement def SubjectAfterActor {
        actor operator : Vehicle;
        subject item : Vehicle;
    }

    use case def ConformingInclude {
        include drive;
    }

    use case def IncludesSomethingThatIsNotAUseCase {
        include vehicle;
    }

    verification def VerifiesARequirement {
        objective {
            verify limit;
        }
    }

    verification def VerifiesSomethingThatIsNotARequirement {
        objective {
            verify vehicle;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/requirement_case_conformance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "satisfy_invalid_endpoint_kind")
        (source "semantic")
        (range (start 13 16) (end 13 23))
        (related-information
          (related
            (uri "memory://snapshot/requirement_case_conformance.md")
            (range (start 2 4) (end 2 27))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "duplicate_role_member")
        (source "semantic")
        (range (start 18 8) (end 18 33))
        (related-information
          (related
            (uri "memory://snapshot/requirement_case_conformance.md")
            (range (start 17 8) (end 17 32))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "subject_member_not_first")
        (source "semantic")
        (range (start 23 8) (end 23 31))
        (related-information
          (related
            (uri "memory://snapshot/requirement_case_conformance.md")
            (range (start 22 8) (end 22 33))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "use_case_include_invalid_target")
        (source "semantic")
        (range (start 31 16) (end 31 23))
        (related-information
          (related
            (uri "memory://snapshot/requirement_case_conformance.md")
            (range (start 2 4) (end 2 27))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "verified_requirement_invalid_target")
        (source "semantic")
        (range (start 42 19) (end 42 26))
        (related-information
          (related
            (uri "memory://snapshot/requirement_case_conformance.md")
            (range (start 2 4) (end 2 27))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:74bfdfd058c9fe8f199d83bd3c095fb9c01bf34b5bc53cb977632e5a9c2e9486") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::ConformingInclude"))) (kind use-case-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (includeUseCase (reference "drive")))))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::ConformingSatisfy"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind part-def) (name "ConformingSatisfy")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "limit")))))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Drive"))) (kind use-case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::IncludesSomethingThatIsNotAUseCase"))) (kind use-case-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (includeUseCase (reference "vehicle")))))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Limit"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SatisfiesSomethingThatIsNotARequirement"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind part-def) (name "SatisfiesSomethingThatIsNotARequirement")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "vehicle")))))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor::item"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor::operator"))) (kind requirement-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects::first"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects::second"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::VerifiesARequirement"))) (kind verification-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::VerifiesARequirement::objective"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind verification-def) (name "VerifiesARequirement")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verify-requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (verifyRequirementTarget (reference "limit")))))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::VerifiesSomethingThatIsNotARequirement"))) (kind verification-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::VerifiesSomethingThatIsNotARequirement::objective"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind verification-def) (name "VerifiesSomethingThatIsNotARequirement")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verify-requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (verifyRequirementTarget (reference "vehicle")))))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::drive"))) (kind use-case) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Drive")))))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::limit"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Limit")))))
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::ConformingInclude"))) (kind includeUseCase) (ordinal 0))
      (authored-target "drive")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::drive")))))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind part-def) (name "ConformingSatisfy")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::limit")))))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::IncludesSomethingThatIsNotAUseCase"))) (kind includeUseCase) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind part-def) (name "SatisfiesSomethingThatIsNotARequirement")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor::item"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor::operator"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects::first"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects::second"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind verification-def) (name "VerifiesARequirement")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0))
      (authored-target "limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::limit")))))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind verification-def) (name "VerifiesSomethingThatIsNotARequirement")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::drive"))) (kind featureTyping) (ordinal 0))
      (authored-target "Drive")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Drive")))))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::limit"))) (kind featureTyping) (ordinal 0))
      (authored-target "Limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Limit")))))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")))))
  )
  (relationships
    (relationship (kind includeUseCase) (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::ConformingInclude"))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::drive"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::ConformingInclude"))) (kind includeUseCase) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind part-def) (name "ConformingSatisfy")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::limit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind part-def) (name "ConformingSatisfy")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind includeUseCase) (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::IncludesSomethingThatIsNotAUseCase"))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::IncludesSomethingThatIsNotAUseCase"))) (kind includeUseCase) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind part-def) (name "SatisfiesSomethingThatIsNotARequirement")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind part-def) (name "SatisfiesSomethingThatIsNotARequirement")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor::item"))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor::item"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor::operator"))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor::operator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects::first"))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects::first"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects::second"))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects::second"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind verifyRequirementTarget) (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind verification-def) (name "VerifiesARequirement")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::limit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind verification-def) (name "VerifiesARequirement")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0)))
    (relationship (kind verifyRequirementTarget) (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind verification-def) (name "VerifiesSomethingThatIsNotARequirement")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind verification-def) (name "VerifiesSomethingThatIsNotARequirement")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::drive"))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Drive"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::drive"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::limit"))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Limit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::limit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::vehicle"))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind part-def) (name "ConformingSatisfy")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::ConformingSatisfy"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind part-def) (name "SatisfiesSomethingThatIsNotARequirement")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SatisfiesSomethingThatIsNotARequirement"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor::item"))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor::operator"))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects::first"))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects::second"))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::VerifiesARequirement::objective"))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::VerifiesARequirement"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind verification-def) (name "VerifiesARequirement")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::VerifiesARequirement::objective"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::VerifiesSomethingThatIsNotARequirement::objective"))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::VerifiesSomethingThatIsNotARequirement"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind verification-def) (name "VerifiesSomethingThatIsNotARequirement")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::VerifiesSomethingThatIsNotARequirement::objective"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind part-def) (name "ConformingSatisfy")) (anonymous (kind satisfy) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::ConformingSatisfy")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Drive")))
      (subtype (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::drive")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Limit")))
      (subtype (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::limit")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind part-def) (name "SatisfiesSomethingThatIsNotARequirement")) (anonymous (kind satisfy) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SatisfiesSomethingThatIsNotARequirement")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor::item")))
      (featured-by (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor")))
      (type (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor::operator")))
      (featured-by (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor")))
      (type (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects::first")))
      (featured-by (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects")))
      (type (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects::second")))
      (featured-by (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects")))
      (type (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")))
      (subtype (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor::item")) (scopes any))
      (subtype (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor::operator")) (scopes any))
      (subtype (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects::first")) (scopes any))
      (subtype (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects::second")) (scopes any))
      (subtype (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::VerifiesARequirement::objective")))
      (featured-by (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::VerifiesARequirement")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind verification-def) (name "VerifiesARequirement")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::VerifiesARequirement::objective")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::VerifiesSomethingThatIsNotARequirement::objective")))
      (featured-by (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::VerifiesSomethingThatIsNotARequirement")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind verification-def) (name "VerifiesSomethingThatIsNotARequirement")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::VerifiesSomethingThatIsNotARequirement::objective")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::drive")))
      (type (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Drive")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Drive")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Drive")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::limit")))
      (type (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Limit")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Limit")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Limit")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::vehicle")))
      (type (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/requirement_case_conformance.md") (range (start 27 16) (end 27 21)) (probe (position 27 16))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::ConformingInclude"))) (kind includeUseCase) (ordinal 0) (authored-target "drive")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::drive")))))
    )
  )
  (query (document "memory://snapshot/requirement_case_conformance.md") (range (start 9 16) (end 9 21)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind part-def) (name "ConformingSatisfy")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::limit")))))
    )
  )
  (query (document "memory://snapshot/requirement_case_conformance.md") (range (start 31 16) (end 31 23)) (probe (position 31 16))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::IncludesSomethingThatIsNotAUseCase"))) (kind includeUseCase) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::vehicle")))))
    )
  )
  (query (document "memory://snapshot/requirement_case_conformance.md") (range (start 13 16) (end 13 23)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind part-def) (name "SatisfiesSomethingThatIsNotARequirement")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::vehicle")))))
    )
  )
  (query (document "memory://snapshot/requirement_case_conformance.md") (range (start 23 23) (end 23 30)) (probe (position 23 23))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor::item"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/requirement_case_conformance.md") (range (start 22 25) (end 22 32)) (probe (position 22 25))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::SubjectAfterActor::operator"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/requirement_case_conformance.md") (range (start 17 24) (end 17 31)) (probe (position 17 24))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects::first"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/requirement_case_conformance.md") (range (start 18 25) (end 18 32)) (probe (position 18 25))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::TwoSubjects::second"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/requirement_case_conformance.md") (range (start 36 19) (end 36 24)) (probe (position 36 19))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind verification-def) (name "VerifiesARequirement")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0) (authored-target "limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::limit")))))
    )
  )
  (query (document "memory://snapshot/requirement_case_conformance.md") (range (start 42 19) (end 42 26)) (probe (position 42 19))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (path (named (kind package) (name "RequirementsAndCases")) (named (kind verification-def) (name "VerifiesSomethingThatIsNotARequirement")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::vehicle")))))
    )
  )
  (query (document "memory://snapshot/requirement_case_conformance.md") (range (start 6 21) (end 6 26)) (probe (position 6 21))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::drive"))) (kind featureTyping) (ordinal 0) (authored-target "Drive")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Drive")))))
    )
  )
  (query (document "memory://snapshot/requirement_case_conformance.md") (range (start 4 24) (end 4 29)) (probe (position 4 24))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::limit"))) (kind featureTyping) (ordinal 0) (authored-target "Limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Limit")))))
    )
  )
  (query (document "memory://snapshot/requirement_case_conformance.md") (range (start 2 19) (end 2 26)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_case_conformance.md") (qualified-name "RequirementsAndCases::Vehicle")))))
    )
  )
)
~~~
