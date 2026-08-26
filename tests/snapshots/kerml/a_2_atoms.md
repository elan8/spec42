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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 7 16) (end 7 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 39))
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
        (range (start 11 23) (end 11 40))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:ce6425b889fc153c98df08a93122a3aedca50b5c85670b6ddf82ad7ecd939f6c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text " This package defines a keyword (atom) for classifiers with\n\t * exactly one instance and are disjoint from any others\n\t * marked with this keyword.\n\t "))))
    (declaration (id (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Metaobjects::Metaobject") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::Atom"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)) (facts (short-name "atom")) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Metaobject")))))
    (declaration (id (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata::baseType"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "Atom")) (metaCastTarget (reference "KerML::Classifier")))))
    (declaration (id (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Metaobjects::Metaobject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "Metaobject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "Atom")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::Atom")))))
    (reference (id (source (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind metaCastTarget) (ordinal 0))
      (authored-target "KerML::Classifier")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::Atom"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata::baseType"))) (target (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata::baseType"))) (target (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata::baseType")))
      (featured-by (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata")))
      (supertype (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata::baseType")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/a_2_atoms.md") (range (start 7 16) (end 7 39)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::Metaobject")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/a_2_atoms.md") (range (start 10 43) (end 10 53)) (probe (position 10 43))
    (reference (id (source (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::AtomMetadata"))) (kind specialization) (ordinal 0) (authored-target "Metaobject")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/a_2_atoms.md") (range (start 11 13) (end 11 17)) (probe (position 11 13))
    (reference (id (source (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "Atom")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_2_atoms.md") (qualified-name "Atoms::Atom")))))
    )
  )
  (query (document "memory://snapshot/a_2_atoms.md") (range (start 11 23) (end 11 40)) (probe (position 11 23))
    (reference (id (source (node (document "memory://snapshot/a_2_atoms.md") (path (named (kind package) (name "Atoms")) (named (kind kerml-metaclass) (name "AtomMetadata")) (named (kind default-reference) (name "baseType")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind metaCastTarget) (ordinal 0) (authored-target "KerML::Classifier")
      (outcome (status unresolved)))
    )
  )
)
~~~
