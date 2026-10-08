# META
~~~ini
description=A calc usage is typed by every Typings target (`calc c : C1, C2;`) and lowers its whole OccurrenceUsagePrefix; the `individual`, portion-kind, `ref` and direction facts are pinned by the SMG fact lines, and the `#Tag` annotations on a part definition member by METADATA ANNOTATIONS, because neither has a typed expectation yet
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 deriveFeatureOwnedTyping
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping
type=file
~~~
# SOURCE
~~~sysml
package Calcs {
    metadata def Tag;
    calc def C1;
    calc def C2;
    individual calc def Run;

    calc top : C1;

    part def Owner {
        calc both : C1, C2;
        individual calc run : Run;
        snapshot calc frozen : Run;
        ref calc referenced : C1;
        in calc input : C1;
        #Tag calc tagged : C1;
        #Tag attribute marked;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (derived-relationship-collection (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping") (source "Calcs::top") (kind feature_typing) (target "Calcs::C1") (provenance authored) (outcome resolved))
  (derived-relationship-collection (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping") (source "Calcs::Owner::both") (kind feature_typing) (target "Calcs::C1") (provenance authored) (outcome resolved))
  (derived-relationship-collection (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping") (source "Calcs::Owner::both") (kind feature_typing) (target "Calcs::C2") (provenance authored) (outcome resolved))
  (derived-relationship-collection (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping") (source "Calcs::Owner::run") (kind feature_typing) (target "Calcs::Run") (provenance authored) (outcome resolved))
  (derived-relationship-collection (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping") (source "Calcs::Owner::frozen") (kind feature_typing) (target "Calcs::Run") (provenance authored) (outcome resolved))
  (derived-relationship-collection (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping") (source "Calcs::Owner::tagged") (kind feature_typing) (target "Calcs::C1") (provenance authored) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/calc_usage_prefix_and_typings.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ea72ebb49363bd219d766158eee0a9c0ec3badea4ef39c6b88cd5e03b50f9980"))
  (declarations
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C2"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::both"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C1")) (featureTyping (reference "C2")))))
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::frozen"))) (kind calc) (membership (kind feature) (visibility default)) (facts (portion snapshot)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Run")))))
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::input"))) (kind calc) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C1")))))
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::marked"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (path (named (kind package) (name "Calcs")) (named (kind part-def) (name "Owner")) (named (kind attribute) (name "marked")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "Tag")))))
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::referenced"))) (kind calc) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C1")))))
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::run"))) (kind calc) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Run")))))
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::tagged"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C1")))))
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (path (named (kind package) (name "Calcs")) (named (kind part-def) (name "Owner")) (named (kind calc) (name "tagged")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "Tag")))))
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Run"))) (kind calc-def) (membership (kind owning) (visibility default)) (facts (modifiers individual) (individual-multiplicity (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (path (named (kind package) (name "Calcs")) (named (kind calc-def) (name "Run")) (anonymous (kind kerml-multiplicity) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (path (named (kind package) (name "Calcs")) (named (kind calc-def) (name "Run")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)) (facts (origin individual-multiplicity)))
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Tag"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::top"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C1")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::both"))) (kind featureTyping) (ordinal 0))
      (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")))))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::both"))) (kind featureTyping) (ordinal 1))
      (authored-target "C2")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C2")))))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::frozen"))) (kind featureTyping) (ordinal 0))
      (authored-target "Run")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Run")))))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")))))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (path (named (kind package) (name "Calcs")) (named (kind part-def) (name "Owner")) (named (kind attribute) (name "marked")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Tag")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Tag")))))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::referenced"))) (kind featureTyping) (ordinal 0))
      (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")))))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::run"))) (kind featureTyping) (ordinal 0))
      (authored-target "Run")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Run")))))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::tagged"))) (kind featureTyping) (ordinal 0))
      (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")))))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (path (named (kind package) (name "Calcs")) (named (kind part-def) (name "Owner")) (named (kind calc) (name "tagged")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Tag")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Tag")))))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::top"))) (kind featureTyping) (ordinal 0))
      (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::both"))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::both"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::both"))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::both"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::frozen"))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Run"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::frozen"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::input"))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (path (named (kind package) (name "Calcs")) (named (kind part-def) (name "Owner")) (named (kind attribute) (name "marked")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Tag"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (path (named (kind package) (name "Calcs")) (named (kind part-def) (name "Owner")) (named (kind attribute) (name "marked")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::referenced"))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::referenced"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::run"))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Run"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::run"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::tagged"))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::tagged"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (path (named (kind package) (name "Calcs")) (named (kind part-def) (name "Owner")) (named (kind calc) (name "tagged")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Tag"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (path (named (kind package) (name "Calcs")) (named (kind part-def) (name "Owner")) (named (kind calc) (name "tagged")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::top"))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::top"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::both"))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::frozen"))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::input"))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::marked"))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::referenced"))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::run"))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::tagged"))) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")))
      (subtype (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::both")) (scopes any))
      (subtype (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::input")) (scopes any))
      (subtype (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::referenced")) (scopes any))
      (subtype (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::tagged")) (scopes any))
      (subtype (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::top")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C2")))
      (subtype (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::both")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::both")))
      (featured-by (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner")))
      (type (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")) (provenance authored))
      (type (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C2")) (provenance authored))
      (effective-type (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")) (source direct))
      (effective-type (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C2")) (source direct))
      (supertype (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")) (scopes any))
      (supertype (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::frozen")))
      (featured-by (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner")))
      (type (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Run")) (provenance authored))
      (effective-type (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Run")) (source direct))
      (supertype (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Run")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::input")))
      (featured-by (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner")))
      (type (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")) (source direct))
      (supertype (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::marked")))
      (featured-by (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner")))
    )
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::referenced")))
      (featured-by (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner")))
      (type (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")) (source direct))
      (supertype (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::run")))
      (featured-by (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner")))
      (type (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Run")) (provenance authored))
      (effective-type (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Run")) (source direct))
      (supertype (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Run")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::tagged")))
      (featured-by (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner")))
      (type (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")) (source direct))
      (supertype (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Run")))
      (subtype (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::frozen")) (scopes any))
      (subtype (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::run")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::top")))
      (type (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")) (source direct))
      (supertype (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")) (scopes any))
    )
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::marked"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Tag")))))
  (annotation (element (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::tagged"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Tag")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/calc_usage_prefix_and_typings.md") (range (start 9 20) (end 9 22)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::both"))) (kind featureTyping) (ordinal 0) (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")))))
    )
  )
  (query (document "memory://snapshot/calc_usage_prefix_and_typings.md") (range (start 9 24) (end 9 26)) (probe (position 9 24))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::both"))) (kind featureTyping) (ordinal 1) (authored-target "C2")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C2")))))
    )
  )
  (query (document "memory://snapshot/calc_usage_prefix_and_typings.md") (range (start 11 31) (end 11 34)) (probe (position 11 31))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::frozen"))) (kind featureTyping) (ordinal 0) (authored-target "Run")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Run")))))
    )
  )
  (query (document "memory://snapshot/calc_usage_prefix_and_typings.md") (range (start 13 24) (end 13 26)) (probe (position 13 24))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::input"))) (kind featureTyping) (ordinal 0) (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")))))
    )
  )
  (query (document "memory://snapshot/calc_usage_prefix_and_typings.md") (range (start 15 9) (end 15 12)) (probe (position 15 9))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (path (named (kind package) (name "Calcs")) (named (kind part-def) (name "Owner")) (named (kind attribute) (name "marked")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "Tag")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Tag")))))
    )
  )
  (query (document "memory://snapshot/calc_usage_prefix_and_typings.md") (range (start 12 30) (end 12 32)) (probe (position 12 30))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::referenced"))) (kind featureTyping) (ordinal 0) (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")))))
    )
  )
  (query (document "memory://snapshot/calc_usage_prefix_and_typings.md") (range (start 10 30) (end 10 33)) (probe (position 10 30))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::run"))) (kind featureTyping) (ordinal 0) (authored-target "Run")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Run")))))
    )
  )
  (query (document "memory://snapshot/calc_usage_prefix_and_typings.md") (range (start 14 27) (end 14 29)) (probe (position 14 27))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Owner::tagged"))) (kind featureTyping) (ordinal 0) (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")))))
    )
  )
  (query (document "memory://snapshot/calc_usage_prefix_and_typings.md") (range (start 14 9) (end 14 12)) (probe (position 14 9))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (path (named (kind package) (name "Calcs")) (named (kind part-def) (name "Owner")) (named (kind calc) (name "tagged")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "Tag")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::Tag")))))
    )
  )
  (query (document "memory://snapshot/calc_usage_prefix_and_typings.md") (range (start 6 15) (end 6 17)) (probe (position 6 15))
    (reference (id (source (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::top"))) (kind featureTyping) (ordinal 0) (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/calc_usage_prefix_and_typings.md") (qualified-name "Calcs::C1")))))
    )
  )
)
~~~
