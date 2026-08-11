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
  (document "root_package_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 16) (end 12 17))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "10125a54963f4eeedc0481ba6c6b954637caa8be1624b5c15bd15622fe526041") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 9) (character 0)) (end (line 9) (character 21))) (authored (membership (kind Import) (visibility "private") (import (reference "P2::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 9) (character 15)) (end (line 9) (character 17))))))
    (element (id (node (document "d0") (qualified-name "P1"))) (kind "package") (name "P1") (declared-name "P1") (range (start (line 0) (character 0)) (end (line 0) (character 27))))
    (element (id (node (document "d0") (qualified-name "P1::A"))) (kind "part def") (name "A") (declared-name "A") (range (start (line 1) (character 1)) (end (line 1) (character 12))) (parent (node (document "d0") (qualified-name "P1"))))
    (element (id (node (document "d0") (qualified-name "P2"))) (kind "package") (name "P2") (declared-name "P2") (range (start (line 4) (character 0)) (end (line 4) (character 50))))
    (element (id (node (document "d0") (qualified-name "P2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 5) (character 1)) (end (line 5) (character 22))) (parent (node (document "d0") (qualified-name "P2"))) (authored (membership (kind Import) (visibility "private") (import (reference "P1::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 5) (character 16)) (end (line 5) (character 18))))))
    (element (id (node (document "d0") (qualified-name "P2::a"))) (kind "part") (name "a") (declared-name "a") (range (start (line 6) (character 1)) (end (line 6) (character 12))) (parent (node (document "d0") (qualified-name "P2"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range (start (line 6) (character 10)) (end (line 6) (character 11)))))))
    (element (id (node (document "d0") (qualified-name "P3"))) (kind "package") (name "P3") (declared-name "P3") (range (start (line 11) (character 0)) (end (line 11) (character 33))))
    (element (id (node (document "d0") (qualified-name "P3::b"))) (kind "part") (name "b") (declared-name "b") (range (start (line 12) (character 1)) (end (line 12) (character 18))) (parent (node (document "d0") (qualified-name "P3"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "a") (range (start (line 12) (character 16)) (end (line 12) (character 17)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P2::*") (range (start (line 9) (character 15)) (end (line 9) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "P2")))))
    (reference (id (source (node (document "d0") (qualified-name "P2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P1::*") (range (start (line 5) (character 16)) (end (line 5) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "P1")))))
    (reference (id (source (node (document "d0") (qualified-name "P2::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range (start (line 6) (character 10)) (end (line 6) (character 11))) (outcome (status resolved) (target (node (document "d0") (qualified-name "P1::A")))))
    (reference (id (source (node (document "d0") (qualified-name "P3::b"))) (kind subsetting) (ordinal 0)) (authored-target "a") (range (start (line 12) (character 16)) (end (line 12) (character 17))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "P2::a"))) (target (node (document "d0") (qualified-name "P1::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "P2::a"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 6 10) (end 6 11)) (probe (position 6 10))
      (reference
        (source (document "d0") (qualified-name "P2::a"))
        (kind featureTyping) (ordinal 0) (authored-target "A")
        (range (start 6 10) (end 6 11))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "P1::A") (range (start 1 1) (end 1 12)))
        )
      )
    )
    (query (range (start 12 16) (end 12 17)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "P3::b"))
        (kind subsetting) (ordinal 0) (authored-target "a")
        (range (start 12 16) (end 12 17))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 16) (end 5 18)) (probe (position 5 16))
      (reference
        (source (document "d0") (qualified-name "P2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "P1::*")
        (range (start 5 16) (end 5 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "P1") (range (start 0 0) (end 0 27)))
        )
      )
    )
    (query (range (start 9 15) (end 9 17)) (probe (position 9 15))
      (reference
        (source (document "d0") (qualified-name "*"))
        (kind namespaceImport) (ordinal 0) (authored-target "P2::*")
        (range (start 9 15) (end 9 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "P2") (range (start 4 0) (end 4 50)))
        )
      )
    )
  )
)
~~~
