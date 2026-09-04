# META
~~~ini
description=Member lookup after an as-cast traverses the alias to its member element
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.2.4.3:deriveMembershipMemberElementId
libraries=none
~~~
# SOURCE
~~~sysml
package Demo {
    attribute def Frame {
        attribute mRefs;
    }
    alias FrameAlias for Frame;
    attribute def Holder {
        attribute frame : FrameAlias;
    }
    attribute def Derived :> Holder {
        attribute :>> frame {
            attribute :>> mRefs;
        }
    }
    attribute raw;
    assert constraint castScope { (raw as FrameAlias).mRefs == raw }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/alias_cast_member_scope.md"
    (diagnostics
    )
  )
)
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind type_featuring) (source "Demo::Frame::mRefs") (target "Demo::Frame") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/alias_cast_member_scope.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:a80e7ca05c57e722f03a0cb9ae70f7d42e8020e91224b9ca8d8b2af282dec4f5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Derived"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Holder")))))
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "frame") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "frame")))))
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "mRefs") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mRefs")))))
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame::mRefs"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "Frame")))))
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder::frame"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FrameAlias")))))
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::castScope"))) (kind assert-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "raw")) (memberAccessOperand (reference "raw::mRefs")) (typeCheckTarget (reference "FrameAlias")))))
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::raw"))) (kind attribute) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Derived"))) (kind specialization) (ordinal 0))
      (authored-target "Holder")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder")))))
    (reference (id (source (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "frame")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder::frame")))))
    (reference (id (source (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mRefs")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame::mRefs")))))
    (reference (id (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias"))) (kind aliasBinding) (ordinal 0))
      (authored-target "Frame")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame")))))
    (reference (id (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder::frame"))) (kind featureTyping) (ordinal 0))
      (authored-target "FrameAlias")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias")))))
    (reference (id (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::castScope"))) (kind expressionOperand) (ordinal 0))
      (authored-target "raw")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::raw")))))
    (reference (id (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::castScope"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "raw::mRefs")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame::mRefs")))))
    (reference (id (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::castScope"))) (kind typeCheckTarget) (ordinal 0))
      (authored-target "FrameAlias")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Derived"))) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Derived"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder::frame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame::mRefs"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias"))) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder::frame"))) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder::frame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::castScope"))) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::raw"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::castScope"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::castScope"))) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame::mRefs"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::castScope"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typeCheckTarget) (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::castScope"))) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::castScope"))) (kind typeCheckTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Derived"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame::mRefs"))) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder::frame"))) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder::frame"))) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder"))) (provenance implied))
    (relationship (kind typeCheckTarget) (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::castScope"))) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::castScope"))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Derived")))
      (supertype (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Derived")))
      (effective-type (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame")) (source inherited) (from (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder::frame"))))
      (effective-type (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias")) (source inherited) (from (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder::frame"))))
      (supertype (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame")) (scopes any))
      (supertype (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias")) (scopes any))
      (supertype (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder::frame")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame::mRefs")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame")))
      (subtype (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder::frame")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame::mRefs")))
      (featured-by (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame")))
      (subtype (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias")))
      (subtype (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder::frame")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder")))
      (subtype (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Derived")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder::frame")))
      (featured-by (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder")))
      (type (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame")) (provenance implied))
      (type (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias")) (provenance authored))
      (effective-type (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame")) (source direct))
      (effective-type (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias")) (source direct))
      (supertype (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame")) (scopes any))
      (supertype (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias")) (scopes any))
      (subtype (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::castScope"))) (outcome unsupported))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/alias_cast_member_scope.md") (range (start 8 29) (end 8 35)) (probe (position 8 29))
    (reference (id (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Derived"))) (kind specialization) (ordinal 0) (authored-target "Holder")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder")))))
    )
  )
  (query (document "memory://snapshot/alias_cast_member_scope.md") (range (start 9 22) (end 9 27)) (probe (position 9 22))
    (reference (id (source (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "frame")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder::frame")))))
    )
  )
  (query (document "memory://snapshot/alias_cast_member_scope.md") (range (start 10 26) (end 10 31)) (probe (position 10 26))
    (reference (id (source (node (document "memory://snapshot/alias_cast_member_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Derived")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mRefs")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame::mRefs")))))
    )
  )
  (query (document "memory://snapshot/alias_cast_member_scope.md") (range (start 4 25) (end 4 30)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias"))) (kind aliasBinding) (ordinal 0) (authored-target "Frame")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame")))))
    )
  )
  (query (document "memory://snapshot/alias_cast_member_scope.md") (range (start 6 26) (end 6 36)) (probe (position 6 26))
    (reference (id (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Holder::frame"))) (kind featureTyping) (ordinal 0) (authored-target "FrameAlias")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias")))))
    )
  )
  (query (document "memory://snapshot/alias_cast_member_scope.md") (range (start 14 63) (end 14 66)) (probe (position 14 63))
    (reference (id (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::castScope"))) (kind expressionOperand) (ordinal 0) (authored-target "raw")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::raw")))))
    )
  )
  (query (document "memory://snapshot/alias_cast_member_scope.md") (range (start 14 34) (end 14 59)) (probe (position 14 34))
    (reference (id (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::castScope"))) (kind memberAccessOperand) (ordinal 0) (authored-target "raw::mRefs")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::Frame::mRefs")))))
    )
  )
  (query (document "memory://snapshot/alias_cast_member_scope.md") (range (start 14 42) (end 14 52)) (probe (position 14 42))
    (reference (id (source (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::castScope"))) (kind typeCheckTarget) (ordinal 0) (authored-target "FrameAlias")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_cast_member_scope.md") (qualified-name "Demo::FrameAlias")))))
    )
  )
)
~~~
