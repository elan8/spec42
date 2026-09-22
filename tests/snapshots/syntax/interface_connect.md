# META
~~~ini
description=Interface definition with end members and interface usage connect parse with and without a name
type=file
~~~
# SOURCE
~~~sysml
package Interfaces {
    port def Power;
    part def A { port p : Power; }
    part def B { port q : Power; }
    interface def PowerLink {
        end src : Power;
        end dst : Power;
    }
    interface def Bare;
    interface def Braced { }
    part def System {
        part a : A;
        part b : B;
        interface i : PowerLink connect a.p to b.q;
        interface connect a.p to b.q;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/interface_connect.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:597396060252a6b77233263bbe5920ad22c05dd956691807d617b83e69fc0f0b"))
  (declarations
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Power")))))
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Power")))))
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Bare"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Braced"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::dst"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Power")))))
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::src"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Power")))))
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (path (named (kind package) (name "Interfaces")) (named (kind part-def) (name "System")) (anonymous (kind interface) (ordinal 0))))) (kind interface) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "a::p")) (connectorEnd (reference "b::q")))))
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i"))) (kind interface) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerLink")) (connectorEnd (reference "a::p")) (connectorEnd (reference "b::q")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")))))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q"))) (kind featureTyping) (ordinal 0))
      (authored-target "Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")))))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::dst"))) (kind featureTyping) (ordinal 0))
      (authored-target "Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")))))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::src"))) (kind featureTyping) (ordinal 0))
      (authored-target "Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")))))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (path (named (kind package) (name "Interfaces")) (named (kind part-def) (name "System")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "a::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p")))))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (path (named (kind package) (name "Interfaces")) (named (kind part-def) (name "System")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "b::q")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q")))))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A")))))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B")))))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink")))))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i"))) (kind connectorEnd) (ordinal 0))
      (authored-target "a::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p")))))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i"))) (kind connectorEnd) (ordinal 1))
      (authored-target "b::q")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p"))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q"))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::dst"))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::dst"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::src"))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::src"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/interface_connect.md") (path (named (kind package) (name "Interfaces")) (named (kind part-def) (name "System")) (anonymous (kind interface) (ordinal 0))))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interface_connect.md") (path (named (kind package) (name "Interfaces")) (named (kind part-def) (name "System")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/interface_connect.md") (path (named (kind package) (name "Interfaces")) (named (kind part-def) (name "System")) (anonymous (kind interface) (ordinal 0))))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interface_connect.md") (path (named (kind package) (name "Interfaces")) (named (kind part-def) (name "System")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::a"))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::b"))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i"))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i"))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i"))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i"))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p"))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q"))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::dst"))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::src"))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/interface_connect.md") (path (named (kind package) (name "Interfaces")) (named (kind part-def) (name "System")) (anonymous (kind interface) (ordinal 0))))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::a"))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::b"))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i"))) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A")))
      (subtype (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::a")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p")))
      (featured-by (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A")))
      (type (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")) (provenance authored))
      (effective-type (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")) (source direct))
      (supertype (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B")))
      (subtype (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::b")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q")))
      (featured-by (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B")))
      (type (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")) (provenance authored))
      (effective-type (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")) (source direct))
      (supertype (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")))
      (subtype (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p")) (scopes any))
      (subtype (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q")) (scopes any))
      (subtype (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::dst")) (scopes any))
      (subtype (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::src")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink")))
      (positional-ends (authored 2) (effective 2))
      (subtype (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::dst")))
      (featured-by (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink")))
      (type (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")) (provenance authored))
      (effective-type (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")) (source direct))
      (supertype (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::src")))
      (featured-by (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink")))
      (type (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")) (provenance authored))
      (effective-type (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")) (source direct))
      (supertype (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (path (named (kind package) (name "Interfaces")) (named (kind part-def) (name "System")) (anonymous (kind interface) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System")))
    )
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::a")))
      (featured-by (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System")))
      (type (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A")) (source direct))
      (supertype (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::b")))
      (featured-by (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System")))
      (type (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B")) (source direct))
      (supertype (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i")))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System")))
      (type (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink")) (provenance authored))
      (effective-type (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink")) (source direct))
      (supertype (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink")) (scopes any))
    )
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Bare"))) (kind interface))
  (connector (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Braced"))) (kind interface))
  (connector (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink"))) (kind interface) (end (name (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::src"))) unconnected) (end (name (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::dst"))) unconnected))
  (connector (id (node (document "memory://snapshot/interface_connect.md") (path (named (kind package) (name "Interfaces")) (named (kind part-def) (name "System")) (anonymous (kind interface) (ordinal 0))))) (kind interface) (end bare (feature-chain (root (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::a"))) (terminal (resolved (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p")))) (path (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::a")) (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p"))) "a::p")) (end bare (feature-chain (root (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::b"))) (terminal (resolved (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q")))) (path (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::b")) (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q"))) "b::q")))
  (connector (id (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i"))) (kind interface) (type (resolved (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink")))) (end bare (feature-chain (root (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::a"))) (terminal (resolved (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p")))) (path (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::a")) (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p"))) "a::p")) (end bare (feature-chain (root (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::b"))) (terminal (resolved (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q")))) (path (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::b")) (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q"))) "b::q")))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/interface_connect.md") (range (start 2 26) (end 2 31)) (probe (position 2 26))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p"))) (kind featureTyping) (ordinal 0) (authored-target "Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")))))
    )
  )
  (query (document "memory://snapshot/interface_connect.md") (range (start 3 26) (end 3 31)) (probe (position 3 26))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q"))) (kind featureTyping) (ordinal 0) (authored-target "Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")))))
    )
  )
  (query (document "memory://snapshot/interface_connect.md") (range (start 6 18) (end 6 23)) (probe (position 6 18))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::dst"))) (kind featureTyping) (ordinal 0) (authored-target "Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")))))
    )
  )
  (query (document "memory://snapshot/interface_connect.md") (range (start 5 18) (end 5 23)) (probe (position 5 18))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink::src"))) (kind featureTyping) (ordinal 0) (authored-target "Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::Power")))))
    )
  )
  (query (document "memory://snapshot/interface_connect.md") (range (start 14 26) (end 14 29)) (probe (position 14 26))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (path (named (kind package) (name "Interfaces")) (named (kind part-def) (name "System")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "a::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p")))))
    )
  )
  (query (document "memory://snapshot/interface_connect.md") (range (start 14 33) (end 14 36)) (probe (position 14 33))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (path (named (kind package) (name "Interfaces")) (named (kind part-def) (name "System")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "b::q")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q")))))
    )
  )
  (query (document "memory://snapshot/interface_connect.md") (range (start 11 17) (end 11 18)) (probe (position 11 17))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A")))))
    )
  )
  (query (document "memory://snapshot/interface_connect.md") (range (start 12 17) (end 12 18)) (probe (position 12 17))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B")))))
    )
  )
  (query (document "memory://snapshot/interface_connect.md") (range (start 13 22) (end 13 31)) (probe (position 13 22))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i"))) (kind featureTyping) (ordinal 0) (authored-target "PowerLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::PowerLink")))))
    )
  )
  (query (document "memory://snapshot/interface_connect.md") (range (start 13 40) (end 13 43)) (probe (position 13 40))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i"))) (kind connectorEnd) (ordinal 0) (authored-target "a::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::A::p")))))
    )
  )
  (query (document "memory://snapshot/interface_connect.md") (range (start 13 47) (end 13 50)) (probe (position 13 47))
    (reference (id (source (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::System::i"))) (kind connectorEnd) (ordinal 1) (authored-target "b::q")
      (outcome (status resolved) (target (node (document "memory://snapshot/interface_connect.md") (qualified-name "Interfaces::B::q")))))
    )
  )
)
~~~
