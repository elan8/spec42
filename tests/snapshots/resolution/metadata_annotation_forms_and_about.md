# META
~~~ini
description=`inspection().metadata_annotations` publishes each authored annotation with its form (`#Tag` prefix, `@Tag` member, `metadata` usage), its resolved annotating definition, and its `about` targets - an `about` target that does not resolve stays unresolved rather than being dropped
type=file
~~~
# SOURCE
~~~sysml
package MetadataAnnotationForms {

    metadata def Risk {
        attribute severity;
    }

    part def Component;

    // `@Tag { ... }` annotating-member form: binds to its owner, carries a body.
    part criticalPump : Component {
        @Risk {
            severity = 5;
        }
    }

    // `#Tag` usage-prefix form: binds to its owner, carries no body and no `about` clause.
    #Risk
    part backupPump : Component;

    // `metadata` usage form with an `about` clause naming two targets, one unresolvable. The
    // annotation binds to each listed target rather than to its owner.
    metadata sharedRisk : Risk about criticalPump, missingPart;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/metadata_annotation_forms_and_about.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 51) (end 21 62))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:43b351c58664f0f212b03325f3023e6172d34413ba915e4501dda043ef7d2ed3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk::severity"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::backupPump"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "backupPump")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Risk")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::criticalPump"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Risk")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "severity") (short-name absent) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "severity")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::sharedRisk"))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Risk")) (metadataAnnotationAbout (reference "criticalPump")) (metadataAnnotationAbout (reference "missingPart")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::backupPump"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Component")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "backupPump")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Risk")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::criticalPump"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Component")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Risk")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "severity")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk::severity")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::sharedRisk"))) (kind featureTyping) (ordinal 0))
      (authored-target "Risk")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::sharedRisk"))) (kind metadataAnnotationAbout) (ordinal 0))
      (authored-target "criticalPump")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::criticalPump")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::sharedRisk"))) (kind metadataAnnotationAbout) (ordinal 1))
      (authored-target "missingPart")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::backupPump"))) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::backupPump"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "backupPump")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "backupPump")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::criticalPump"))) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::criticalPump"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk::severity"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::sharedRisk"))) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::sharedRisk"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotationAbout) (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::sharedRisk"))) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::criticalPump"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::sharedRisk"))) (kind metadataAnnotationAbout) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk::severity"))) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "backupPump")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::backupPump"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::criticalPump"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 5)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Component")))
      (subtype (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::backupPump")) (scopes any))
      (subtype (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::criticalPump")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk")))
      (subtype (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::sharedRisk")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk::severity")))
      (featured-by (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk")))
      (subtype (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::backupPump")))
      (type (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Component")) (source direct))
      (supertype (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "backupPump")) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::backupPump")))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::criticalPump")))
      (type (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Component")) (source direct))
      (supertype (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::criticalPump")))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk::severity")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::sharedRisk")))
      (type (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk")) (provenance authored))
      (effective-type (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk")) (source direct))
      (supertype (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk")) (scopes any))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 5))))
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::backupPump"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk")))))
  (annotation (element (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::criticalPump"))) (form annotating-member) (definition (resolved (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk")))) (value (redefines (resolved (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk::severity")))) (outcome resolved) (literal (value (kind integer) (integer 5)))))
  (annotation (element (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::criticalPump"))) (form usage) (definition (resolved (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk")))) (about (resolved (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::criticalPump")))) (about unresolved))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/metadata_annotation_forms_and_about.md") (range (start 17 22) (end 17 31)) (probe (position 17 22))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::backupPump"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Component")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_forms_and_about.md") (range (start 16 5) (end 16 9)) (probe (position 16 5))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "backupPump")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "Risk")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_forms_and_about.md") (range (start 9 24) (end 9 33)) (probe (position 9 24))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::criticalPump"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Component")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_forms_and_about.md") (range (start 10 9) (end 10 13)) (probe (position 10 9))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "Risk")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_forms_and_about.md") (range (start 11 12) (end 11 20)) (probe (position 11 12))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (path (named (kind package) (name "MetadataAnnotationForms")) (named (kind part) (name "criticalPump")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "severity")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk::severity")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_forms_and_about.md") (range (start 21 26) (end 21 30)) (probe (position 21 26))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::sharedRisk"))) (kind featureTyping) (ordinal 0) (authored-target "Risk")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::Risk")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_forms_and_about.md") (range (start 21 37) (end 21 49)) (probe (position 21 37))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::sharedRisk"))) (kind metadataAnnotationAbout) (ordinal 0) (authored-target "criticalPump")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::criticalPump")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_forms_and_about.md") (range (start 21 51) (end 21 62)) (probe (position 21 51))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_forms_and_about.md") (qualified-name "MetadataAnnotationForms::sharedRisk"))) (kind metadataAnnotationAbout) (ordinal 1) (authored-target "missingPart")
      (outcome (status unresolved)))
    )
  )
)
~~~
