# META
~~~ini
description=SysML Example (Simple Tests): ImportTest
type=file
~~~
# SOURCE
~~~sysml
package ImportTest {
    package Pkg1 {
    	private import Pkg2::Pkg21::Pkg211::P211;
    	private import Pkg2::Pkg21::*;
    	private import Pkg211::*::**;
        part p11 : Pkg211::P211;
        part def P12;
    }

    package Pkg2 {
        private import Pkg1::*;
        package Pkg21 {
        	package Pkg211 {
        		part def P211 :> P12;
        	}
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/import_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 4 20) (end 4 33))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:43d411905026d0649007b0f3bb99071fd0a539112675a5b70e456f269bcc7b69") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_test.md") (path (named (kind package) (name "ImportTest")) (named (kind package) (name "Pkg1")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Pkg2::Pkg21::Pkg211::P211") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/import_test.md") (path (named (kind package) (name "ImportTest")) (named (kind package) (name "Pkg1")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Pkg2::Pkg21") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/import_test.md") (path (named (kind package) (name "ImportTest")) (named (kind package) (name "Pkg1")) (anonymous (kind import) (ordinal 2)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Pkg211") (import (shape namespace) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg1::P12"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg1::p11"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Pkg211::P211"))))
    (declaration (id (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_test.md") (path (named (kind package) (name "ImportTest")) (named (kind package) (name "Pkg2")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Pkg1") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg2::Pkg21"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "P12"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/import_test.md") (path (named (kind package) (name "ImportTest")) (named (kind package) (name "Pkg1")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Pkg2::Pkg21")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg2::Pkg21")))))
    (reference (id (source (node (document "memory://snapshot/import_test.md") (path (named (kind package) (name "ImportTest")) (named (kind package) (name "Pkg1")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Pkg211")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/import_test.md") (path (named (kind package) (name "ImportTest")) (named (kind package) (name "Pkg1")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "Pkg2::Pkg21::Pkg211::P211")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211")))))
    (reference (id (source (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg1::p11"))) (kind featureTyping) (ordinal 0))
      (authored-target "Pkg211::P211")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211")))))
    (reference (id (source (node (document "memory://snapshot/import_test.md") (path (named (kind package) (name "ImportTest")) (named (kind package) (name "Pkg2")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Pkg1")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg1")))))
    (reference (id (source (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211"))) (kind specialization) (ordinal 0))
      (authored-target "P12")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg1::P12")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg1::p11"))) (target (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg1::p11"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211"))) (target (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg1::P12"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/import_test.md") (range (start 3 20) (end 3 34)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/import_test.md") (path (named (kind package) (name "ImportTest")) (named (kind package) (name "Pkg1")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "Pkg2::Pkg21")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg2::Pkg21")))))
  )
  (query (document "memory://snapshot/import_test.md") (range (start 4 20) (end 4 33)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/import_test.md") (path (named (kind package) (name "ImportTest")) (named (kind package) (name "Pkg1")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0) (authored-target "Pkg211")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/import_test.md") (range (start 2 20) (end 2 45)) (probe (position 2 20))
    (reference (id (source (node (document "memory://snapshot/import_test.md") (path (named (kind package) (name "ImportTest")) (named (kind package) (name "Pkg1")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "Pkg2::Pkg21::Pkg211::P211")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211")))))
  )
  (query (document "memory://snapshot/import_test.md") (range (start 5 19) (end 5 31)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg1::p11"))) (kind featureTyping) (ordinal 0) (authored-target "Pkg211::P211")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211")))))
  )
  (query (document "memory://snapshot/import_test.md") (range (start 10 23) (end 10 30)) (probe (position 10 23))
    (reference (id (source (node (document "memory://snapshot/import_test.md") (path (named (kind package) (name "ImportTest")) (named (kind package) (name "Pkg2")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "Pkg1")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg1")))))
  )
  (query (document "memory://snapshot/import_test.md") (range (start 13 27) (end 13 30)) (probe (position 13 27))
    (reference (id (source (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211"))) (kind specialization) (ordinal 0) (authored-target "P12")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_test.md") (qualified-name "ImportTest::Pkg1::P12")))))
  )
)
~~~
