# META
~~~ini
description=KerML KerML Spec Annex A: A-2-Atoms
type=file
~~~
# SOURCE
~~~kerml
package Atoms {
	doc
	/* This package defines a keyword (atom) for classifiers with
	 * exactly one instance and are disjoint from any others
	 * marked with this keyword.
	 */

	private import Metaobjects::Metaobject;
	
	classifier Atom;
	metaclass <atom> AtomMetadata specializes Metaobject {
		baseType = Atom meta KerML::Classifier;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/a_2_atoms.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 9 1) (end 9 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 10 43) (end 10 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 13) (end 11 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 23) (end 11 40))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:ce6425b889fc153c98df08a93122a3aedca50b5c85670b6ddf82ad7ecd939f6c") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text " This package defines a keyword (atom) for classifiers with\n\t * exactly one instance and are disjoint from any others\n\t * marked with this keyword.\n\t "))))
    (declaration (id (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Metaobjects::Metaobject") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)) (facts (short-name "atom")) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Metaobject"))))
    (declaration (id (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata::baseType"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "Atom")) (metaCastTarget (reference "KerML::Classifier"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "Metaobjects::Metaobject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "Metaobject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata::baseType"))) (kind expressionOperand) (ordinal 0))
      (authored-target "Atom")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata::baseType"))) (kind metaCastTarget) (ordinal 0))
      (authored-target "KerML::Classifier")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata::baseType"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/a_2_atoms.md") (range (start 7 16) (end 7 39)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::Metaobject")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_2_atoms.md") (range (start 10 43) (end 10 53)) (probe (position 10 43))
    (reference (id (source (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata"))) (kind specialization) (ordinal 0) (authored-target "Metaobject")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_2_atoms.md") (range (start 11 13) (end 11 17)) (probe (position 11 13))
    (reference (id (source (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata::baseType"))) (kind expressionOperand) (ordinal 0) (authored-target "Atom")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_2_atoms.md") (range (start 11 23) (end 11 40)) (probe (position 11 23))
    (reference (id (source (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata::baseType"))) (kind metaCastTarget) (ordinal 0) (authored-target "KerML::Classifier")
      (outcome (status unresolved)))
  )
)
~~~
