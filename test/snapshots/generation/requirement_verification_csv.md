# META
~~~ini
description=Generate authoritative requirement verification links with explicit unsupported outcomes
type=generate
libraries=standard
plugin=requirement_verification_csv
~~~
# SOURCE
~~~sysml
package Verification {
    requirement def SafeStop;
    requirement def Availability;
    requirement safeStop : SafeStop;
    requirement availability : Availability;

    verification def SafetyCheck {
        objective { verify safeStop; }
        objective availabilityObjective { verify availability; }
    }

    verification safetyRun : SafetyCheck {
        objective runtimeObjective { verify safeStop; }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/requirement_verification_csv.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:21e54e58d4953281af02f9361a08c8787fca8b11de3e983252a33dce57d8dc3f") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::Availability"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafeStop"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafetyCheck"))) (kind verification-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafetyCheck::availabilityObjective"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification-def) (name "SafetyCheck")) (named (kind requirement) (name "availabilityObjective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verify-requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (verifyRequirementTarget (reference "availability")))))
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafetyCheck::objective"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification-def) (name "SafetyCheck")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verify-requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (verifyRequirementTarget (reference "safeStop")))))
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::availability"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Availability")))))
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safeStop"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SafeStop")))))
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safetyRun"))) (kind verification) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SafetyCheck")))))
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safetyRun::runtimeObjective"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification) (name "safetyRun")) (named (kind requirement) (name "runtimeObjective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verify-requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (verifyRequirementTarget (reference "safeStop")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification-def) (name "SafetyCheck")) (named (kind requirement) (name "availabilityObjective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0))
      (authored-target "availability")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::availability")))))
    (reference (id (source (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification-def) (name "SafetyCheck")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0))
      (authored-target "safeStop")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safeStop")))))
    (reference (id (source (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::availability"))) (kind featureTyping) (ordinal 0))
      (authored-target "Availability")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::Availability")))))
    (reference (id (source (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safeStop"))) (kind featureTyping) (ordinal 0))
      (authored-target "SafeStop")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafeStop")))))
    (reference (id (source (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safetyRun"))) (kind featureTyping) (ordinal 0))
      (authored-target "SafetyCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafetyCheck")))))
    (reference (id (source (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification) (name "safetyRun")) (named (kind requirement) (name "runtimeObjective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0))
      (authored-target "safeStop")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safeStop")))))
  )
  (relationships
    (relationship (kind verifyRequirementTarget) (source (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification-def) (name "SafetyCheck")) (named (kind requirement) (name "availabilityObjective")) (anonymous (kind verify-requirement) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::availability"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification-def) (name "SafetyCheck")) (named (kind requirement) (name "availabilityObjective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0)))
    (relationship (kind verifyRequirementTarget) (source (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification-def) (name "SafetyCheck")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safeStop"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification-def) (name "SafetyCheck")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::availability"))) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::Availability"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::availability"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safeStop"))) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafeStop"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safeStop"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safetyRun"))) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafetyCheck"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safetyRun"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind verifyRequirementTarget) (source (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification) (name "safetyRun")) (named (kind requirement) (name "runtimeObjective")) (anonymous (kind verify-requirement) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safeStop"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification) (name "safetyRun")) (named (kind requirement) (name "runtimeObjective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::Availability")))
      (subtype (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::availability")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafeStop")))
      (subtype (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safeStop")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafetyCheck")))
      (subtype (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safetyRun")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafetyCheck::availabilityObjective")))
      (featured-by (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafetyCheck")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification-def) (name "SafetyCheck")) (named (kind requirement) (name "availabilityObjective")) (anonymous (kind verify-requirement) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafetyCheck::availabilityObjective")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafetyCheck::objective")))
      (featured-by (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafetyCheck")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification-def) (name "SafetyCheck")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafetyCheck::objective")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::availability")))
      (type (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::Availability")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::Availability")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::Availability")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safeStop")))
      (type (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafeStop")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafeStop")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafeStop")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safetyRun")))
      (type (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafetyCheck")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafetyCheck")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafetyCheck")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safetyRun::runtimeObjective")))
      (featured-by (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safetyRun")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification) (name "safetyRun")) (named (kind requirement) (name "runtimeObjective")) (anonymous (kind verify-requirement) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safetyRun::runtimeObjective")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/requirement_verification_csv.md") (range (start 8 49) (end 8 61)) (probe (position 8 49))
    (reference (id (source (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification-def) (name "SafetyCheck")) (named (kind requirement) (name "availabilityObjective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0) (authored-target "availability")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::availability")))))
    )
  )
  (query (document "memory://snapshot/requirement_verification_csv.md") (range (start 7 27) (end 7 35)) (probe (position 7 27))
    (reference (id (source (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification-def) (name "SafetyCheck")) (named (kind requirement) (name "objective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0) (authored-target "safeStop")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safeStop")))))
    )
  )
  (query (document "memory://snapshot/requirement_verification_csv.md") (range (start 4 31) (end 4 43)) (probe (position 4 31))
    (reference (id (source (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::availability"))) (kind featureTyping) (ordinal 0) (authored-target "Availability")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::Availability")))))
    )
  )
  (query (document "memory://snapshot/requirement_verification_csv.md") (range (start 3 27) (end 3 35)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safeStop"))) (kind featureTyping) (ordinal 0) (authored-target "SafeStop")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafeStop")))))
    )
  )
  (query (document "memory://snapshot/requirement_verification_csv.md") (range (start 11 29) (end 11 40)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safetyRun"))) (kind featureTyping) (ordinal 0) (authored-target "SafetyCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::SafetyCheck")))))
    )
  )
  (query (document "memory://snapshot/requirement_verification_csv.md") (range (start 12 44) (end 12 52)) (probe (position 12 44))
    (reference (id (source (node (document "memory://snapshot/requirement_verification_csv.md") (path (named (kind package) (name "Verification")) (named (kind verification) (name "safetyRun")) (named (kind requirement) (name "runtimeObjective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0) (authored-target "safeStop")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_verification_csv.md") (qualified-name "Verification::safeStop")))))
    )
  )
)
~~~
# GENERATED
## requirement_verification.csv
~~~csv
requirement_qualified_name,verification_case_qualified_name,relationship_semantic_id,provenance,requirement_status,outcome,outcome_status,publication_status
Verification::safeStop,Verification::SafetyCheck,element/v149:memory://snapshot/requirement_verification_csv.md7:packagen12:Verification1:016:verification-defn11:SafetyCheck1:011:requirementn9:objective1:018:verify-requirementa1:0,authored,resolved,,unsupported,recovered
Verification::availability,Verification::SafetyCheck,element/v149:memory://snapshot/requirement_verification_csv.md7:packagen12:Verification1:016:verification-defn11:SafetyCheck1:011:requirementn21:availabilityObjective1:018:verify-requirementa1:0,authored,resolved,,unsupported,recovered
Verification::safeStop,Verification::safetyRun,element/v149:memory://snapshot/requirement_verification_csv.md7:packagen12:Verification1:012:verificationn9:safetyRun1:011:requirementn16:runtimeObjective1:018:verify-requirementa1:0,authored,resolved,,unsupported,recovered

~~~
