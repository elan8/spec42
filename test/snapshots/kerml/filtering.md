# META
~~~ini
description=KerML Simple Tests: Filtering
type=file
~~~
# SOURCE
~~~kerml
package Filtering {
	private import ScalarValues::*;
	
	package Annotations {
		metaclass ApprovalAnnotation {
			approved : Boolean;
			approver : String;
			level : Natural;
		}
	}
	
	package DesignModel {
	    private import Annotations::*;
	    struct System {
	         @ApprovalAnnotation {
	            approved = true;
	            approver = "John Smith";
	            level = 2;
	        }
	    }
		composite feature system : System;
	}

	package UpperLevelApprovals {
	    private import DesignModel::**;
	    filter Annotations::ApprovalAnnotation::approved and 
	           Annotations::ApprovalAnnotation::level > 1;
	    
	    struct Test :> System;
	}
	
	package UpperLevelApprovals1 {
	    private import Annotations::**;
	    private import DesignModel::**[@Structure][approved and level > 1];
	    
	    struct Test :> System;	    
	}
	
 	private import KerML::*;
	package Meta {
		private import DesignModel::*;
		filter (Element::name == "System" and not Type::isAbstract) or 
		       Feature::isComposite;
		
		struct Test :> System; 
		feature :> system;
	}

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/filtering.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 3) (end 5 11))
      )
      (diagnostic
        (severity error)
        (code "recovered_calc_body_element")
        (source "parser")
        (range (start 5 12) (end 6 3))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 5 12) (end 6 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 3) (end 6 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 3) (end 7 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 24 20) (end 24 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 12) (end 25 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 12) (end 26 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 28 20) (end 28 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 32 20) (end 32 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 33 20) (end 33 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 35 20) (end 35 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 38 17) (end 38 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 10) (end 41 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 44) (end 41 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 9) (end 42 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:931e6c53a461d8ab14d6c29c0880e583f34ef4dfe3c2b750be576c4b45db35bf") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "KerML") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations::ApprovalAnnotation"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "approved")) (expressionOperand (reference "approver")) (expressionOperand (reference "level"))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "DesignModel")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Annotations") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::System"))) (kind kerml-structure) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "ApprovalAnnotation"))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "DesignModel")) (named (kind kerml-structure) (name "System")) (anonymous (kind metadata) (ordinal 0)))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "DesignModel")) (named (kind kerml-structure) (name "System")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "approved")))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "DesignModel")) (named (kind kerml-structure) (name "System")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "approver")))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "DesignModel")) (named (kind kerml-structure) (name "System")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "level")))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::system"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "System"))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta"))) (kind package) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "Element::name")) (expressionOperand (reference "Type::isAbstract")) (expressionOperand (reference "Feature::isComposite"))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "Meta")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "DesignModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "Meta")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "system"))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta::Test"))) (kind kerml-structure) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "System"))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::UpperLevelApprovals"))) (kind package) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "Annotations::ApprovalAnnotation::approved")) (expressionOperand (reference "Annotations::ApprovalAnnotation::level"))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::UpperLevelApprovals1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "UpperLevelApprovals1")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Annotations") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "UpperLevelApprovals1")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (filterImport (reference "DesignModel") (import (shape filtered-namespace) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::UpperLevelApprovals1::Test"))) (kind kerml-structure) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "System"))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "UpperLevelApprovals")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "DesignModel") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::UpperLevelApprovals::Test"))) (kind kerml-structure) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "System"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "KerML")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations::ApprovalAnnotation"))) (kind expressionOperand) (ordinal 0))
      (authored-target "approved")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations::ApprovalAnnotation"))) (kind expressionOperand) (ordinal 1))
      (authored-target "approver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations::ApprovalAnnotation"))) (kind expressionOperand) (ordinal 2))
      (authored-target "level")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "DesignModel")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Annotations")
      (outcome (status resolved) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations")))))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::System"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "ApprovalAnnotation")
      (outcome (status resolved) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations::ApprovalAnnotation")))))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::system"))) (kind featureTyping) (ordinal 0))
      (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::System")))))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta"))) (kind expressionOperand) (ordinal 0))
      (authored-target "Element::name")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta"))) (kind expressionOperand) (ordinal 1))
      (authored-target "Type::isAbstract")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta"))) (kind expressionOperand) (ordinal 2))
      (authored-target "Feature::isComposite")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "Meta")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "DesignModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel")))))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "Meta")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind subsetting) (ordinal 0))
      (authored-target "system")
      (outcome (status resolved) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::system")))))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta::Test"))) (kind specialization) (ordinal 0))
      (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::System")))))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::UpperLevelApprovals"))) (kind expressionOperand) (ordinal 0))
      (authored-target "Annotations::ApprovalAnnotation::approved")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::UpperLevelApprovals"))) (kind expressionOperand) (ordinal 1))
      (authored-target "Annotations::ApprovalAnnotation::level")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "UpperLevelApprovals1")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "Annotations")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "UpperLevelApprovals1")) (anonymous (kind import) (ordinal 1)))))) (kind filterImport) (ordinal 0))
      (authored-target "DesignModel")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::UpperLevelApprovals1::Test"))) (kind specialization) (ordinal 0))
      (authored-target "System")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "UpperLevelApprovals")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "DesignModel")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::UpperLevelApprovals::Test"))) (kind specialization) (ordinal 0))
      (authored-target "System")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::System"))) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations::ApprovalAnnotation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::System"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::system"))) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::System"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "Meta")) (anonymous (kind kerml-feature) (ordinal 0)))))) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::system"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "Meta")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind subsetting) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta::Test"))) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::System"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta::Test"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations::ApprovalAnnotation"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations::ApprovalAnnotation"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations::ApprovalAnnotation"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "DesignModel")) (named (kind kerml-structure) (name "System")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "approved")))))) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "DesignModel")) (named (kind kerml-structure) (name "System")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "approver")))))) (value (kind string) (value "John Smith")))
    (evaluated (declaration (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "DesignModel")) (named (kind kerml-structure) (name "System")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "level")))))) (value (kind integer) (integer 2)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/filtering.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 38 17) (end 38 25)) (probe (position 38 17))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "KerML")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 5 3) (end 5 11)) (probe (position 5 3))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations::ApprovalAnnotation"))) (kind expressionOperand) (ordinal 0) (authored-target "approved")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 6 3) (end 6 11)) (probe (position 6 3))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations::ApprovalAnnotation"))) (kind expressionOperand) (ordinal 1) (authored-target "approver")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 7 3) (end 7 8)) (probe (position 7 3))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations::ApprovalAnnotation"))) (kind expressionOperand) (ordinal 2) (authored-target "level")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 12 20) (end 12 34)) (probe (position 12 20))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "DesignModel")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "Annotations")
      (outcome (status resolved) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations")))))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 14 11) (end 14 29)) (probe (position 14 11))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::System"))) (kind metadataAnnotation) (ordinal 0) (authored-target "ApprovalAnnotation")
      (outcome (status resolved) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations::ApprovalAnnotation")))))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 20 29) (end 20 35)) (probe (position 20 29))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::system"))) (kind featureTyping) (ordinal 0) (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::System")))))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 41 10) (end 41 23)) (probe (position 41 10))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta"))) (kind expressionOperand) (ordinal 0) (authored-target "Element::name")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 41 44) (end 41 60)) (probe (position 41 44))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta"))) (kind expressionOperand) (ordinal 1) (authored-target "Type::isAbstract")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 42 9) (end 42 29)) (probe (position 42 9))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta"))) (kind expressionOperand) (ordinal 2) (authored-target "Feature::isComposite")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 40 17) (end 40 31)) (probe (position 40 17))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "Meta")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "DesignModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel")))))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 45 13) (end 45 19)) (probe (position 45 13))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "Meta")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind subsetting) (ordinal 0) (authored-target "system")
      (outcome (status resolved) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::system")))))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 44 17) (end 44 23)) (probe (position 44 17))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta::Test"))) (kind specialization) (ordinal 0) (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel::System")))))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 25 12) (end 25 53)) (probe (position 25 12))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::UpperLevelApprovals"))) (kind expressionOperand) (ordinal 0) (authored-target "Annotations::ApprovalAnnotation::approved")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 26 12) (end 26 50)) (probe (position 26 12))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::UpperLevelApprovals"))) (kind expressionOperand) (ordinal 1) (authored-target "Annotations::ApprovalAnnotation::level")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 32 20) (end 32 35)) (probe (position 32 20))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "UpperLevelApprovals1")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "Annotations")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 33 20) (end 33 71)) (probe (position 33 20))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "UpperLevelApprovals1")) (anonymous (kind import) (ordinal 1)))))) (kind filterImport) (ordinal 0) (authored-target "DesignModel")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 35 20) (end 35 26)) (probe (position 35 20))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::UpperLevelApprovals1::Test"))) (kind specialization) (ordinal 0) (authored-target "System")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 24 20) (end 24 35)) (probe (position 24 20))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (path (named (kind package) (name "Filtering")) (named (kind package) (name "UpperLevelApprovals")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "DesignModel")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 28 20) (end 28 26)) (probe (position 28 20))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::UpperLevelApprovals::Test"))) (kind specialization) (ordinal 0) (authored-target "System")
      (outcome (status unresolved)))
  )
)
~~~
