# META
~~~ini
description=SysML Example (Simple Tests): RootPackageTest
type=file
~~~
# SOURCE
~~~sysml
package P1 {
	part def A;
}

package P2 {
	private import P1::*;
	part a : A;
}

private import P2::*;

package P3 {
	part b subsets a;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/root_package_test.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:715b36879e2e779b906bfe60e084538accfe4fac2c0e01e818b920aeaef52bea") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/root_package_test.md") (path (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "P2") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/root_package_test.md") (qualified-name "P1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/root_package_test.md") (qualified-name "P1::A"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/root_package_test.md") (qualified-name "P2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/root_package_test.md") (path (named (kind package) (name "P2")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "P1") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/root_package_test.md") (qualified-name "P2::a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/root_package_test.md") (qualified-name "P3"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/root_package_test.md") (qualified-name "P3::b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "a"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/root_package_test.md") (path (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "P2")
      (outcome (status resolved) (target (node (document "memory://snapshot/root_package_test.md") (qualified-name "P2")))))
    (reference (id (source (node (document "memory://snapshot/root_package_test.md") (path (named (kind package) (name "P2")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "P1")
      (outcome (status resolved) (target (node (document "memory://snapshot/root_package_test.md") (qualified-name "P1")))))
    (reference (id (source (node (document "memory://snapshot/root_package_test.md") (qualified-name "P2::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/root_package_test.md") (qualified-name "P1::A")))))
    (reference (id (source (node (document "memory://snapshot/root_package_test.md") (qualified-name "P3::b"))) (kind subsetting) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/root_package_test.md") (qualified-name "P2::a")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/root_package_test.md") (qualified-name "P2::a"))) (target (node (document "memory://snapshot/root_package_test.md") (qualified-name "P1::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/root_package_test.md") (qualified-name "P2::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/root_package_test.md") (qualified-name "P3::b"))) (target (node (document "memory://snapshot/root_package_test.md") (qualified-name "P2::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/root_package_test.md") (qualified-name "P3::b"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/root_package_test.md") (qualified-name "P2::a")))
      (supertype (node (document "memory://snapshot/root_package_test.md") (qualified-name "P1::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/root_package_test.md") (qualified-name "P3::b")))
      (supertype (node (document "memory://snapshot/root_package_test.md") (qualified-name "P1::A")) (scopes any))
      (supertype (node (document "memory://snapshot/root_package_test.md") (qualified-name "P2::a")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/root_package_test.md") (range (start 9 15) (end 9 20)) (probe (position 9 15))
    (reference (id (source (node (document "memory://snapshot/root_package_test.md") (path (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "P2")
      (outcome (status resolved) (target (node (document "memory://snapshot/root_package_test.md") (qualified-name "P2")))))
  )
  (query (document "memory://snapshot/root_package_test.md") (range (start 5 16) (end 5 21)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/root_package_test.md") (path (named (kind package) (name "P2")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "P1")
      (outcome (status resolved) (target (node (document "memory://snapshot/root_package_test.md") (qualified-name "P1")))))
  )
  (query (document "memory://snapshot/root_package_test.md") (range (start 6 10) (end 6 11)) (probe (position 6 10))
    (reference (id (source (node (document "memory://snapshot/root_package_test.md") (qualified-name "P2::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/root_package_test.md") (qualified-name "P1::A")))))
  )
  (query (document "memory://snapshot/root_package_test.md") (range (start 12 16) (end 12 17)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/root_package_test.md") (qualified-name "P3::b"))) (kind subsetting) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/root_package_test.md") (qualified-name "P2::a")))))
  )
)
~~~
