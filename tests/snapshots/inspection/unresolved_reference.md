# META
~~~ini
description=A reference at a position that resolution could not settle stays distinct from no reference at all
type=file
~~~
# SOURCE
~~~sysml
package Broken {
    part def Known;

    part missing : Absent;
    part known : Known;
    attribute size = undefinedName + 1;
}
~~~
# EDITOR QUERIES
~~~ini
probe unresolved_reference.md 3 22
probe unresolved_reference.md 3 9
probe unresolved_reference.md 4 17
probe unresolved_reference.md 5 25
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/unresolved_reference.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 19) (end 3 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 21) (end 5 34))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:38022cec3cfb7d58c5189fdcdabd099c6cdb469ebccd33e1fd8ae2448d0d4571"))
  (declarations
    (declaration (id (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::Known"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::known"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Known")))))
    (declaration (id (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::missing"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Absent")))))
    (declaration (id (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::size"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/unresolved_reference.md") (path (named (kind package) (name "Broken")) (named (kind attribute) (name "size")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/unresolved_reference.md") (path (named (kind package) (name "Broken")) (named (kind attribute) (name "size")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/unresolved_reference.md") (path (named (kind package) (name "Broken")) (named (kind attribute) (name "size")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/unresolved_reference.md") (path (named (kind package) (name "Broken")) (named (kind attribute) (name "size")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "undefinedName")))))
    (declaration (id (node (document "memory://snapshot/unresolved_reference.md") (path (named (kind package) (name "Broken")) (named (kind attribute) (name "size")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::known"))) (kind featureTyping) (ordinal 0))
      (authored-target "Known")
      (outcome (status resolved) (target (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::Known")))))
    (reference (id (source (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::missing"))) (kind featureTyping) (ordinal 0))
      (authored-target "Absent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/unresolved_reference.md") (path (named (kind package) (name "Broken")) (named (kind attribute) (name "size")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "undefinedName")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::known"))) (target (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::Known"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::known"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::size"))) (target (node (document "memory://snapshot/unresolved_reference.md") (path (named (kind package) (name "Broken")) (named (kind attribute) (name "size")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/unresolved_reference.md") (path (named (kind package) (name "Broken")) (named (kind attribute) (name "size")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/unresolved_reference.md") (path (named (kind package) (name "Broken")) (named (kind attribute) (name "size")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/unresolved_reference.md") (path (named (kind package) (name "Broken")) (named (kind attribute) (name "size")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::Known")))
      (subtype (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::known")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::known")))
      (type (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::Known")) (provenance authored))
      (effective-type (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::Known")) (source direct))
      (supertype (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::Known")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::size")))
      (supertype (node (document "memory://snapshot/unresolved_reference.md") (path (named (kind package) (name "Broken")) (named (kind attribute) (name "size")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/unresolved_reference.md") (path (named (kind package) (name "Broken")) (named (kind attribute) (name "size")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/unresolved_reference.md") (path (named (kind package) (name "Broken")) (named (kind attribute) (name "size")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::size")) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/unresolved_reference.md") (path (named (kind package) (name "Broken")) (named (kind attribute) (name "size")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (operator "+" (feature-reference "undefinedName" (target unresolved)) (literal (value (kind integer) (integer 1)))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/unresolved_reference.md") (range (start 4 17) (end 4 22)) (probe (position 4 17))
    (reference (id (source (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::known"))) (kind featureTyping) (ordinal 0) (authored-target "Known")
      (outcome (status resolved) (target (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::Known")))))
    )
  )
  (query (document "memory://snapshot/unresolved_reference.md") (range (start 3 19) (end 3 25)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/unresolved_reference.md") (qualified-name "Broken::missing"))) (kind featureTyping) (ordinal 0) (authored-target "Absent")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/unresolved_reference.md") (range (start 5 21) (end 5 34)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/unresolved_reference.md") (path (named (kind package) (name "Broken")) (named (kind attribute) (name "size")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "undefinedName")
      (outcome (status unresolved)))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/unresolved_reference.md") (position 3 22)
    (target (status unresolved))
    (rename (status unresolved))
    (visible-members (candidates (member (name "Broken") (qualified-name "Broken") (kind "Package")) (member (name "Known") (qualified-name "Broken::Known") (kind "PartDefinition")) (member (name "known") (qualified-name "Broken::known") (kind "PartUsage")) (member (name "missing") (qualified-name "Broken::missing") (kind "PartUsage")) (member (name "size") (qualified-name "Broken::size") (kind "AttributeUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "missing")
          (qualified-name "Broken::missing")
          (location (document "memory://snapshot/unresolved_reference.md") (range (start 3 9) (end 3 16)) (role Declaration))
          (declaration (range (start 3 4) (end 3 26)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Absent") (target unresolved))
          (typing (outcome unresolved))
          (effective-typing (outcome unresolved))
        )
      )
      (reference-kind featureTyping)
      (referenced (status unresolved))
    )
  )
  (probe (document "memory://snapshot/unresolved_reference.md") (position 3 9)
    (target (status resolved) (candidate (name "missing") (location (document "memory://snapshot/unresolved_reference.md") (range (start 3 9) (end 3 16)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/unresolved_reference.md") (range (start 3 9) (end 3 16)) (role Declaration))))
    (rename (status ready) (name "missing") (range (start 3 9) (end 3 16)) (occurrences 1))
    (visible-members (candidates (member (name "Broken") (qualified-name "Broken") (kind "Package")) (member (name "Known") (qualified-name "Broken::Known") (kind "PartDefinition")) (member (name "known") (qualified-name "Broken::known") (kind "PartUsage")) (member (name "missing") (qualified-name "Broken::missing") (kind "PartUsage")) (member (name "size") (qualified-name "Broken::size") (kind "AttributeUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "missing")
          (qualified-name "Broken::missing")
          (location (document "memory://snapshot/unresolved_reference.md") (range (start 3 9) (end 3 16)) (role Declaration))
          (declaration (range (start 3 4) (end 3 26)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Absent") (target unresolved))
          (typing (outcome unresolved))
          (effective-typing (outcome unresolved))
        )
      )
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/unresolved_reference.md") (position 4 17)
    (target (status resolved) (candidate (name "Known") (location (document "memory://snapshot/unresolved_reference.md") (range (start 1 13) (end 1 18)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/unresolved_reference.md") (range (start 1 13) (end 1 18)) (role Declaration)) (location (document "memory://snapshot/unresolved_reference.md") (range (start 4 17) (end 4 22)) (role Reference))))
    (rename (status ready) (name "Known") (range (start 4 17) (end 4 22)) (occurrences 2))
    (visible-members (candidates (member (name "Broken") (qualified-name "Broken") (kind "Package")) (member (name "Known") (qualified-name "Broken::Known") (kind "PartDefinition")) (member (name "known") (qualified-name "Broken::known") (kind "PartUsage")) (member (name "missing") (qualified-name "Broken::missing") (kind "PartUsage")) (member (name "size") (qualified-name "Broken::size") (kind "AttributeUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "known")
          (qualified-name "Broken::known")
          (location (document "memory://snapshot/unresolved_reference.md") (range (start 4 9) (end 4 14)) (role Declaration))
          (declaration (range (start 4 4) (end 4 23)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Known") (target resolved))
          (typing (outcome resolved) (target "Broken::Known"))
          (effective-typing (outcome resolved) (type (qualified-name "Broken::Known") (origin direct) (provenance authored)))
          (outgoing (kind "typing") (peer "Broken::Known") (provenance authored))
        )
      )
      (reference-kind featureTyping)
      (referenced (status resolved)
        (element (kind "PartDefinition")
          (name "Known")
          (qualified-name "Broken::Known")
          (location (document "memory://snapshot/unresolved_reference.md") (range (start 1 13) (end 1 18)) (role Declaration))
          (declaration (range (start 1 4) (end 1 19)))
          (membership (kind owning) (visibility public) (provenance default))
          (incoming (kind "typing") (peer "Broken::known") (provenance authored))
        )
      )
    )
  )
  (probe (document "memory://snapshot/unresolved_reference.md") (position 5 25)
    (target (status unresolved))
    (rename (status unresolved))
    (visible-members (candidates (member (name "Broken") (qualified-name "Broken") (kind "Package")) (member (name "Known") (qualified-name "Broken::Known") (kind "PartDefinition")) (member (name "known") (qualified-name "Broken::known") (kind "PartUsage")) (member (name "missing") (qualified-name "Broken::missing") (kind "PartUsage")) (member (name "size") (qualified-name "Broken::size") (kind "AttributeUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "Expression")
          (qualified-name "Broken::size::")
          (location (document "memory://snapshot/unresolved_reference.md") (range (start 5 21) (end 5 38)) (role Declaration))
          (declaration (range (start 5 21) (end 5 38)))
          (membership (kind owning) (visibility private) (provenance default))
          (evaluation unresolved-operand)
          (relationship (kind "expressionOperand") (provenance authored) (authored "undefinedName") (target unresolved))
          (incoming (kind "typeFeaturing") (peer "Broken::size::::") (provenance implied))
        )
      )
      (reference-kind expressionOperand)
      (referenced (status unresolved))
    )
  )
  (document-symbols (document "memory://snapshot/unresolved_reference.md")
    (status resolved)
    (symbol (kind "Package") (name "Broken") (qualified-name "Broken") (location (document "memory://snapshot/unresolved_reference.md") (range (start 0 8) (end 0 14)) (role Declaration)) (declaration (range (start 0 0) (end 6 1))))
    (symbol (kind "PartDefinition") (name "Known") (qualified-name "Broken::Known") (location (document "memory://snapshot/unresolved_reference.md") (range (start 1 13) (end 1 18)) (role Declaration)) (declaration (range (start 1 4) (end 1 19))))
    (symbol (kind "PartUsage") (name "missing") (qualified-name "Broken::missing") (location (document "memory://snapshot/unresolved_reference.md") (range (start 3 9) (end 3 16)) (role Declaration)) (declaration (range (start 3 4) (end 3 26))))
    (symbol (kind "PartUsage") (name "known") (qualified-name "Broken::known") (location (document "memory://snapshot/unresolved_reference.md") (range (start 4 9) (end 4 14)) (role Declaration)) (declaration (range (start 4 4) (end 4 23))))
    (symbol (kind "AttributeUsage") (name "size") (qualified-name "Broken::size") (location (document "memory://snapshot/unresolved_reference.md") (range (start 5 14) (end 5 18)) (role Declaration)) (declaration (range (start 5 4) (end 5 39))))
    (symbol (kind "Expression") (qualified-name "Broken::size::") (location (document "memory://snapshot/unresolved_reference.md") (range (start 5 21) (end 5 38)) (role Declaration)) (declaration (range (start 5 21) (end 5 38))))
  )
)
~~~
