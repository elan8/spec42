# META
~~~ini
description=KerML Simple Tests: Imports
type=file
~~~
# SOURCE
~~~kerml
package Imports {

	package P {
		class A;
		class B;
		class C;
	}
	
	package Q {
		class A;
		class D {
			class E;
		}
		package Q1 {
			class D;
			class E;
			private package Q1a {
				class G;
			}
		}
		package Q2 {
			class F;
		}
	}
	
	package R {
		public import Q::*;
	}

	
	package S {
		public import P::*;
		public import Q::**;
		
		class X :> A;
		class Y :> D;
		class Z :> F;
	}
	
	package S1 {
		public import P::*;
		public import R::*;
		
		class X :> A;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/imports.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 16 3) (end 19 2))
      )
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 26 16) (end 26 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 32 16) (end 32 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 35 13) (end 35 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 36 13) (end 36 14))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 43 13) (end 43 14))
        (related-information
          (related
            (uri "memory://snapshot/imports.md")
            (range (start 3 2) (end 3 10))
          )
          (related
            (uri "memory://snapshot/imports.md")
            (range (start 9 2) (end 9 10))
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:9e3430e702d813ef628a81d12b0c321be4801f6c36c62a3fc24b8489031d0fd7") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::P::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::P::B"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::P::C"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::Q"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::Q::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::Q::D"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::Q::D::E"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::Q::Q1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::Q::Q1::D"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::Q::Q1::E"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::Q::Q2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::Q::Q2::F"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::R"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (path (named (kind package) (name "Imports")) (named (kind package) (name "R")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Q") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/imports.md") (path (named (kind package) (name "Imports")) (named (kind package) (name "S1")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "P") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/imports.md") (path (named (kind package) (name "Imports")) (named (kind package) (name "S1")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "R") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S1::X"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A")))))
    (declaration (id (node (document "memory://snapshot/imports.md") (path (named (kind package) (name "Imports")) (named (kind package) (name "S")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "P") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/imports.md") (path (named (kind package) (name "Imports")) (named (kind package) (name "S")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "Q") (import (shape membership) (recursive true))))))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S::X"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A")))))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S::Y"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "D")))))
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S::Z"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "F")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/imports.md") (path (named (kind package) (name "Imports")) (named (kind package) (name "R")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/imports.md") (qualified-name "Imports::Q")))))
    (reference (id (source (node (document "memory://snapshot/imports.md") (path (named (kind package) (name "Imports")) (named (kind package) (name "S1")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/imports.md") (qualified-name "Imports::P")))))
    (reference (id (source (node (document "memory://snapshot/imports.md") (path (named (kind package) (name "Imports")) (named (kind package) (name "S1")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/imports.md") (qualified-name "Imports::R")))))
    (reference (id (source (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S1::X"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/imports.md") (qualified-name "Imports::P::A")) (node (document "memory://snapshot/imports.md") (qualified-name "Imports::Q::A")))))
    (reference (id (source (node (document "memory://snapshot/imports.md") (path (named (kind package) (name "Imports")) (named (kind package) (name "S")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/imports.md") (qualified-name "Imports::P")))))
    (reference (id (source (node (document "memory://snapshot/imports.md") (path (named (kind package) (name "Imports")) (named (kind package) (name "S")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Q")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S::X"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/imports.md") (qualified-name "Imports::P::A")))))
    (reference (id (source (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S::Y"))) (kind specialization) (ordinal 0))
      (authored-target "D")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S::Z"))) (kind specialization) (ordinal 0))
      (authored-target "F")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S::X"))) (target (node (document "memory://snapshot/imports.md") (qualified-name "Imports::P::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S::X"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::P::A")))
      (subtype (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S::X")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S::X")))
      (supertype (node (document "memory://snapshot/imports.md") (qualified-name "Imports::P::A")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/imports.md") (range (start 26 16) (end 26 20)) (probe (position 26 16))
    (reference (id (source (node (document "memory://snapshot/imports.md") (path (named (kind package) (name "Imports")) (named (kind package) (name "R")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/imports.md") (qualified-name "Imports::Q")))))
    )
  )
  (query (document "memory://snapshot/imports.md") (range (start 40 16) (end 40 20)) (probe (position 40 16))
    (reference (id (source (node (document "memory://snapshot/imports.md") (path (named (kind package) (name "Imports")) (named (kind package) (name "S1")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/imports.md") (qualified-name "Imports::P")))))
    )
  )
  (query (document "memory://snapshot/imports.md") (range (start 41 16) (end 41 20)) (probe (position 41 16))
    (reference (id (source (node (document "memory://snapshot/imports.md") (path (named (kind package) (name "Imports")) (named (kind package) (name "S1")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/imports.md") (qualified-name "Imports::R")))))
    )
  )
  (query (document "memory://snapshot/imports.md") (range (start 43 13) (end 43 14)) (probe (position 43 13))
    (reference (id (source (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S1::X"))) (kind specialization) (ordinal 0) (authored-target "A")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/imports.md") (qualified-name "Imports::P::A")) (node (document "memory://snapshot/imports.md") (qualified-name "Imports::Q::A")))))
    )
  )
  (query (document "memory://snapshot/imports.md") (range (start 31 16) (end 31 20)) (probe (position 31 16))
    (reference (id (source (node (document "memory://snapshot/imports.md") (path (named (kind package) (name "Imports")) (named (kind package) (name "S")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/imports.md") (qualified-name "Imports::P")))))
    )
  )
  (query (document "memory://snapshot/imports.md") (range (start 32 16) (end 32 21)) (probe (position 32 16))
    (reference (id (source (node (document "memory://snapshot/imports.md") (path (named (kind package) (name "Imports")) (named (kind package) (name "S")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Q")
      (outcome (status unsupported)))
    )
  )
  (query (document "memory://snapshot/imports.md") (range (start 34 13) (end 34 14)) (probe (position 34 13))
    (reference (id (source (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S::X"))) (kind specialization) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/imports.md") (qualified-name "Imports::P::A")))))
    )
  )
  (query (document "memory://snapshot/imports.md") (range (start 35 13) (end 35 14)) (probe (position 35 13))
    (reference (id (source (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S::Y"))) (kind specialization) (ordinal 0) (authored-target "D")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/imports.md") (range (start 36 13) (end 36 14)) (probe (position 36 13))
    (reference (id (source (node (document "memory://snapshot/imports.md") (qualified-name "Imports::S::Z"))) (kind specialization) (ordinal 0) (authored-target "F")
      (outcome (status unresolved)))
    )
  )
)
~~~
