# META
~~~ini
description=A value bound to an inherited member reports its missing redefinition operator
type=file
~~~
# SOURCE
~~~sysml
package Inheritance {
    enum def Status {
        enum approved;
    }
    part def Base {
        attribute mass : Real;
        attribute status : Status;
    }
    part def Explicit :> Base {
        attribute :>> mass = 1200;
    }
    part def Implicit :> Base {
        attribute mass = 1200;
    }
    part def StringWhereTheMemberIsEnumerated :> Base {
        attribute status = "approved";
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/inherited_value_conformance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 25) (end 5 29))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 12 8) (end 12 30))
        (related-information
          (related
            (uri "memory://snapshot/inherited_value_conformance.md")
            (range (start 5 8) (end 5 30))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 15 8) (end 15 38))
        (related-information
          (related
            (uri "memory://snapshot/inherited_value_conformance.md")
            (range (start 6 8) (end 6 34))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "inherited_attribute_value_type_mismatch")
        (source "semantic")
        (range (start 15 25) (end 15 37))
        (related-information
          (related
            (uri "memory://snapshot/inherited_value_conformance.md")
            (range (start 6 8) (end 6 34))
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:dcc7edbafa111aedbd71a061156b2bf93e08ce3ab7107ef7a216e234bb6342c1") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::status"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Status")))))
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Explicit"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (path (named (kind package) (name "Inheritance")) (named (kind part-def) (name "Explicit")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass")))))
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Implicit"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Implicit::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Status"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Status::approved"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::StringWhereTheMemberIsEnumerated"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::StringWhereTheMemberIsEnumerated::status"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::status"))) (kind featureTyping) (ordinal 0))
      (authored-target "Status")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Status")))))
    (reference (id (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Explicit"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base")))))
    (reference (id (source (node (document "memory://snapshot/inherited_value_conformance.md") (path (named (kind package) (name "Inheritance")) (named (kind part-def) (name "Explicit")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::mass")))))
    (reference (id (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Implicit"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base")))))
    (reference (id (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::StringWhereTheMemberIsEnumerated"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::status"))) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Status"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::status"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Explicit"))) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Explicit"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/inherited_value_conformance.md") (path (named (kind package) (name "Inheritance")) (named (kind part-def) (name "Explicit")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::mass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_value_conformance.md") (path (named (kind package) (name "Inheritance")) (named (kind part-def) (name "Explicit")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Implicit"))) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Implicit"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::StringWhereTheMemberIsEnumerated"))) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::StringWhereTheMemberIsEnumerated"))) (kind specialization) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::mass"))) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::status"))) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/inherited_value_conformance.md") (path (named (kind package) (name "Inheritance")) (named (kind part-def) (name "Explicit")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Explicit"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Implicit::mass"))) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::mass"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Implicit::mass"))) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Implicit"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Status::approved"))) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Status"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::StringWhereTheMemberIsEnumerated::status"))) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::status"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::StringWhereTheMemberIsEnumerated::status"))) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::StringWhereTheMemberIsEnumerated"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/inherited_value_conformance.md") (path (named (kind package) (name "Inheritance")) (named (kind part-def) (name "Explicit")) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind integer) (integer 1200)))
    (evaluated (declaration (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Implicit::mass"))) (state literal) (value (kind integer) (integer 1200)))
    (evaluated (declaration (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::StringWhereTheMemberIsEnumerated::status"))) (state literal) (value (kind string) (value "approved")))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base")))
      (subtype (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Explicit")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Implicit")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::StringWhereTheMemberIsEnumerated")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::mass")))
      (featured-by (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base")))
      (subtype (node (document "memory://snapshot/inherited_value_conformance.md") (path (named (kind package) (name "Inheritance")) (named (kind part-def) (name "Explicit")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Implicit::mass")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::status")))
      (featured-by (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base")))
      (type (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Status")) (provenance authored))
      (effective-type (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Status")) (source direct))
      (supertype (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Status")) (scopes any))
      (subtype (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::StringWhereTheMemberIsEnumerated::status")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Explicit")))
      (supertype (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (path (named (kind package) (name "Inheritance")) (named (kind part-def) (name "Explicit")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Explicit")))
      (supertype (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::mass")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Implicit")))
      (supertype (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Implicit::mass")))
      (featured-by (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Implicit")))
      (supertype (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::mass")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Status")))
      (subtype (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::status")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Status::approved")))
      (featured-by (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Status")))
    )
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::StringWhereTheMemberIsEnumerated")))
      (supertype (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::StringWhereTheMemberIsEnumerated::status")))
      (featured-by (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::StringWhereTheMemberIsEnumerated")))
      (effective-type (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Status")) (source inherited) (from (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::status"))))
      (supertype (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::status")) (scopes any feature))
      (supertype (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Status")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/inherited_value_conformance.md") (range (start 5 25) (end 5 29)) (probe (position 5 25))
    (reference (id (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::mass"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/inherited_value_conformance.md") (range (start 6 27) (end 6 33)) (probe (position 6 27))
    (reference (id (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::status"))) (kind featureTyping) (ordinal 0) (authored-target "Status")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Status")))))
    )
  )
  (query (document "memory://snapshot/inherited_value_conformance.md") (range (start 8 25) (end 8 29)) (probe (position 8 25))
    (reference (id (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Explicit"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base")))))
    )
  )
  (query (document "memory://snapshot/inherited_value_conformance.md") (range (start 9 22) (end 9 26)) (probe (position 9 22))
    (reference (id (source (node (document "memory://snapshot/inherited_value_conformance.md") (path (named (kind package) (name "Inheritance")) (named (kind part-def) (name "Explicit")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base::mass")))))
    )
  )
  (query (document "memory://snapshot/inherited_value_conformance.md") (range (start 11 25) (end 11 29)) (probe (position 11 25))
    (reference (id (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Implicit"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base")))))
    )
  )
  (query (document "memory://snapshot/inherited_value_conformance.md") (range (start 14 49) (end 14 53)) (probe (position 14 49))
    (reference (id (source (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::StringWhereTheMemberIsEnumerated"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_value_conformance.md") (qualified-name "Inheritance::Base")))))
    )
  )
)
~~~
