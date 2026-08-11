# META
~~~ini
description=SysML Training 41 (Language Extension): Semantic Metadata Example
type=file
~~~
# SOURCE
~~~sysml
library package 'Semantic Metadata Example' {
	private import 'Model Library Example'::*;
	private import Metaobjects::SemanticMetadata;

	metadata def situation :> SemanticMetadata {
		:>> baseType = situations meta SysML::Usage;
	}
	
	metadata def cause :> SemanticMetadata {
		:>> baseType = causes meta SysML::Usage;
	}
	
	metadata def failure :> SemanticMetadata {
		:>> baseType = failures meta SysML::Usage;
	}
	
	metadata def causation :> SemanticMetadata {
		:>> baseType = causations meta SysML::Usage;
	}
	
	metadata def scenario :> SemanticMetadata {
		:>> baseType = scenarios meta SysML::Usage;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "41_semantic_metadata_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 45))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
library package 'Semantic Metadata Example' {
    private import 'Model Library Example'::*;
    private import Metaobjects::SemanticMetadata;

    metadata def situation :> SemanticMetadata {
        :>> baseType = situations meta SysML::Usage;
    }

    metadata def cause :> SemanticMetadata {
        :>> baseType = causes meta SysML::Usage;
    }

    metadata def failure :> SemanticMetadata {
        :>> baseType = failures meta SysML::Usage;
    }

    metadata def causation :> SemanticMetadata {
        :>> baseType = causations meta SysML::Usage;
    }

    metadata def scenario :> SemanticMetadata {
        :>> baseType = scenarios meta SysML::Usage;
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "8e2fefb99f75b8ec1e53364f8bc46320a8fb3975369696147360522293fff450") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Semantic Metadata Example"))) (kind "package") (name "Semantic Metadata Example") (declared-name "Semantic Metadata Example") (range (start (line 0) (character 0)) (end (line 0) (character 615))))
    (element (id (node (document "d0") (qualified-name "Semantic Metadata Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 43))) (parent (node (document "d0") (qualified-name "Semantic Metadata Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Model Library Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 39))))))
    (element (id (node (document "d0") (qualified-name "Semantic Metadata Example::SemanticMetadata"))) (kind "import") (name "SemanticMetadata") (declared-name "SemanticMetadata") (range (start (line 2) (character 1)) (end (line 2) (character 46))) (parent (node (document "d0") (qualified-name "Semantic Metadata Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::SemanticMetadata") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 45))))))
    (element (id (node (document "d0") (qualified-name "Semantic Metadata Example::causation"))) (kind "metadata def") (name "causation") (declared-name "causation") (range (start (line 16) (character 1)) (end (line 16) (character 95))) (parent (node (document "d0") (qualified-name "Semantic Metadata Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 16) (character 27)) (end (line 16) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "Semantic Metadata Example::causation::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 17) (character 2)) (end (line 17) (character 46))) (parent (node (document "d0") (qualified-name "Semantic Metadata Example::causation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 17) (character 2)) (end (line 17) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "Semantic Metadata Example::cause"))) (kind "metadata def") (name "cause") (declared-name "cause") (range (start (line 8) (character 1)) (end (line 8) (character 87))) (parent (node (document "d0") (qualified-name "Semantic Metadata Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 8) (character 23)) (end (line 8) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "Semantic Metadata Example::cause::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 9) (character 2)) (end (line 9) (character 42))) (parent (node (document "d0") (qualified-name "Semantic Metadata Example::cause"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 9) (character 2)) (end (line 9) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "Semantic Metadata Example::failure"))) (kind "metadata def") (name "failure") (declared-name "failure") (range (start (line 12) (character 1)) (end (line 12) (character 91))) (parent (node (document "d0") (qualified-name "Semantic Metadata Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 12) (character 25)) (end (line 12) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "Semantic Metadata Example::failure::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 13) (character 2)) (end (line 13) (character 44))) (parent (node (document "d0") (qualified-name "Semantic Metadata Example::failure"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 13) (character 2)) (end (line 13) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "Semantic Metadata Example::scenario"))) (kind "metadata def") (name "scenario") (declared-name "scenario") (range (start (line 20) (character 1)) (end (line 20) (character 93))) (parent (node (document "d0") (qualified-name "Semantic Metadata Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 20) (character 26)) (end (line 20) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "Semantic Metadata Example::scenario::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 21) (character 2)) (end (line 21) (character 45))) (parent (node (document "d0") (qualified-name "Semantic Metadata Example::scenario"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 21) (character 2)) (end (line 21) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "Semantic Metadata Example::situation"))) (kind "metadata def") (name "situation") (declared-name "situation") (range (start (line 4) (character 1)) (end (line 4) (character 95))) (parent (node (document "d0") (qualified-name "Semantic Metadata Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 4) (character 27)) (end (line 4) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "Semantic Metadata Example::situation::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 5) (character 2)) (end (line 5) (character 46))) (parent (node (document "d0") (qualified-name "Semantic Metadata Example::situation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 5) (character 2)) (end (line 5) (character 14)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Semantic Metadata Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Model Library Example::*") (range (start (line 1) (character 16)) (end (line 1) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Semantic Metadata Example::SemanticMetadata"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::SemanticMetadata") (range (start (line 2) (character 16)) (end (line 2) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Semantic Metadata Example::causation"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 16) (character 27)) (end (line 16) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Semantic Metadata Example::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "Semantic Metadata Example::causation::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 17) (character 2)) (end (line 17) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Semantic Metadata Example::causation::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "Semantic Metadata Example::cause"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 8) (character 23)) (end (line 8) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Semantic Metadata Example::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "Semantic Metadata Example::cause::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 9) (character 2)) (end (line 9) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Semantic Metadata Example::cause::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "Semantic Metadata Example::failure"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 12) (character 25)) (end (line 12) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Semantic Metadata Example::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "Semantic Metadata Example::failure::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 13) (character 2)) (end (line 13) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Semantic Metadata Example::failure::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "Semantic Metadata Example::scenario"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 20) (character 26)) (end (line 20) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Semantic Metadata Example::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "Semantic Metadata Example::scenario::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 21) (character 2)) (end (line 21) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Semantic Metadata Example::scenario::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "Semantic Metadata Example::situation"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 4) (character 27)) (end (line 4) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Semantic Metadata Example::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "Semantic Metadata Example::situation::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 5) (character 2)) (end (line 5) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Semantic Metadata Example::situation::baseType")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Semantic Metadata Example::causation"))) (target (node (document "d0") (qualified-name "Semantic Metadata Example::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Semantic Metadata Example::causation"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Semantic Metadata Example::causation::baseType"))) (target (node (document "d0") (qualified-name "Semantic Metadata Example::causation::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Semantic Metadata Example::causation::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Semantic Metadata Example::cause"))) (target (node (document "d0") (qualified-name "Semantic Metadata Example::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Semantic Metadata Example::cause"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Semantic Metadata Example::cause::baseType"))) (target (node (document "d0") (qualified-name "Semantic Metadata Example::cause::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Semantic Metadata Example::cause::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Semantic Metadata Example::failure"))) (target (node (document "d0") (qualified-name "Semantic Metadata Example::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Semantic Metadata Example::failure"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Semantic Metadata Example::failure::baseType"))) (target (node (document "d0") (qualified-name "Semantic Metadata Example::failure::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Semantic Metadata Example::failure::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Semantic Metadata Example::scenario"))) (target (node (document "d0") (qualified-name "Semantic Metadata Example::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Semantic Metadata Example::scenario"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Semantic Metadata Example::scenario::baseType"))) (target (node (document "d0") (qualified-name "Semantic Metadata Example::scenario::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Semantic Metadata Example::scenario::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Semantic Metadata Example::situation"))) (target (node (document "d0") (qualified-name "Semantic Metadata Example::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Semantic Metadata Example::situation"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Semantic Metadata Example::situation::baseType"))) (target (node (document "d0") (qualified-name "Semantic Metadata Example::situation::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Semantic Metadata Example::situation::baseType"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
