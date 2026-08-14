# META
~~~ini
description=SysML Example (Simple Tests): AliasTest
type=file
~~~
# SOURCE
~~~sysml
package AliasTest {
	private import ISQSpaceTime::breadth; // import of an alias
	attribute b :> breadth;
	
    part def P1 {
        port porig1;
        alias po1 for porig1;
    }

    part p1 : P1 {
        port po1 :>> po1;
    }

    part p2 : P1 {
        port pdest;
        alias pd1 for pdest;
    }


    connect p1.po1 to p2.pdest;
	connect p1.po1 to p2.pd1;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/alias_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 2 16) (end 2 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 22) (end 19 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 19) (end 20 25))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:7cea3549e21bfc7a29a70f9088bd3189e97e433d93d0a0d957aec1ba8062489a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQSpaceTime::breadth") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "p1::po1")) (memberAccessOperand (reference "p2::pdest"))))
    (declaration (id (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind bare-connect) (ordinal 1)))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "p1::po1")) (memberAccessOperand (reference "p2::pd1"))))
    (declaration (id (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::po1"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "porig1"))))
    (declaration (id (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::porig1"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::b"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "breadth"))))
    (declaration (id (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P1"))))
    (declaration (id (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p1::po1"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "po1"))))
    (declaration (id (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P1"))))
    (declaration (id (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p2::pd1"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "pdest"))))
    (declaration (id (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p2::pdest"))) (kind port) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQSpaceTime::breadth")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "p1::po1")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::po1")))))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind bare-connect) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "p1::po1")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::po1")))))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "p2::pdest")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind bare-connect) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "p2::pd1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::po1"))) (kind aliasBinding) (ordinal 0))
      (authored-target "porig1")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::porig1")))))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::b"))) (kind specialization) (ordinal 0))
      (authored-target "breadth")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p1"))) (kind featureTyping) (ordinal 0))
      (authored-target "P1")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1")))))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p1::po1"))) (kind redefinition) (ordinal 0))
      (authored-target "po1")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p1::po1")))))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p2"))) (kind featureTyping) (ordinal 0))
      (authored-target "P1")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1")))))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p2::pd1"))) (kind aliasBinding) (ordinal 0))
      (authored-target "pdest")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p2::pdest")))))
  )
  (relationships
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind bare-connect) (ordinal 0)))))) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::po1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind bare-connect) (ordinal 1)))))) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::po1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind bare-connect) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::po1"))) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::porig1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::po1"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p1"))) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p1::po1"))) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p1::po1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p1::po1"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p2"))) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p2::pd1"))) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p2::pdest"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p2::pd1"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind bare-connect) (ordinal 0)))))) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::porig1"))) (provenance implied))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind bare-connect) (ordinal 1)))))) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::porig1"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/alias_test.md") (range (start 1 16) (end 1 37)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::breadth")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/alias_test.md") (range (start 19 12) (end 19 18)) (probe (position 19 12))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "p1::po1")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::po1")))))
  )
  (query (document "memory://snapshot/alias_test.md") (range (start 20 9) (end 20 15)) (probe (position 20 9))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind bare-connect) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "p1::po1")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::po1")))))
  )
  (query (document "memory://snapshot/alias_test.md") (range (start 19 22) (end 19 30)) (probe (position 19 22))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 1) (authored-target "p2::pdest")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/alias_test.md") (range (start 20 19) (end 20 25)) (probe (position 20 19))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (path (named (kind package) (name "AliasTest")) (anonymous (kind bare-connect) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 1) (authored-target "p2::pd1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/alias_test.md") (range (start 6 22) (end 6 28)) (probe (position 6 22))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::po1"))) (kind aliasBinding) (ordinal 0) (authored-target "porig1")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1::porig1")))))
  )
  (query (document "memory://snapshot/alias_test.md") (range (start 2 16) (end 2 23)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::b"))) (kind specialization) (ordinal 0) (authored-target "breadth")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/alias_test.md") (range (start 9 14) (end 9 16)) (probe (position 9 14))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p1"))) (kind featureTyping) (ordinal 0) (authored-target "P1")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1")))))
  )
  (query (document "memory://snapshot/alias_test.md") (range (start 10 21) (end 10 24)) (probe (position 10 21))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p1::po1"))) (kind redefinition) (ordinal 0) (authored-target "po1")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p1::po1")))))
  )
  (query (document "memory://snapshot/alias_test.md") (range (start 13 14) (end 13 16)) (probe (position 13 14))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p2"))) (kind featureTyping) (ordinal 0) (authored-target "P1")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::P1")))))
  )
  (query (document "memory://snapshot/alias_test.md") (range (start 15 22) (end 15 27)) (probe (position 15 22))
    (reference (id (source (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p2::pd1"))) (kind aliasBinding) (ordinal 0) (authored-target "pdest")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_test.md") (qualified-name "AliasTest::p2::pdest")))))
  )
)
~~~
