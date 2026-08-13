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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 4 2) (end 8 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 4 2) (end 8 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 13 5) (end 19 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 5) (end 19 6))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 20 2) (end 21 1))
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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 28 5) (end 28 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 28 5) (end 28 27))
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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 35 5) (end 35 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 35 5) (end 35 27))
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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 41 39) (end 41 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 9) (end 42 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 44 2) (end 44 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 44 2) (end 44 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 45 2) (end 45 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 45 2) (end 45 20))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:931e6c53a461d8ab14d6c29c0880e583f34ef4dfe3c2b750be576c4b45db35bf") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "KerML") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Annotations") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta"))) (kind package) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "Element::name")) (expressionOperand (reference "Feature::isComposite"))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "DesignModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::UpperLevelApprovals"))) (kind package) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "Annotations::ApprovalAnnotation::approved")) (expressionOperand (reference "Annotations::ApprovalAnnotation::level"))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::UpperLevelApprovals1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Annotations") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (filterImport (reference "DesignModel") (import (shape filtered-namespace) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "DesignModel") (import (shape membership) (recursive true)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "KerML")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Annotations")
      (outcome (status resolved) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations")))))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta"))) (kind expressionOperand) (ordinal 0))
      (authored-target "Element::name")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta"))) (kind expressionOperand) (ordinal 1))
      (authored-target "Feature::isComposite")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "DesignModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel")))))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::UpperLevelApprovals"))) (kind expressionOperand) (ordinal 0))
      (authored-target "Annotations::ApprovalAnnotation::approved")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::UpperLevelApprovals"))) (kind expressionOperand) (ordinal 1))
      (authored-target "Annotations::ApprovalAnnotation::level")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Annotations")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 1))))) (kind filterImport) (ordinal 0))
      (authored-target "DesignModel")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "DesignModel")
      (outcome (status unsupported)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/filtering.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 38 17) (end 38 25)) (probe (position 38 17))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "KerML")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 12 20) (end 12 34)) (probe (position 12 20))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Annotations")
      (outcome (status resolved) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Annotations")))))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 41 10) (end 41 23)) (probe (position 41 10))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta"))) (kind expressionOperand) (ordinal 0) (authored-target "Element::name")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 42 9) (end 42 29)) (probe (position 42 9))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::Meta"))) (kind expressionOperand) (ordinal 1) (authored-target "Feature::isComposite")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 40 17) (end 40 31)) (probe (position 40 17))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "DesignModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/filtering.md") (qualified-name "Filtering::DesignModel")))))
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
    (reference (id (source (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Annotations")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 33 20) (end 33 71)) (probe (position 33 20))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 1))))) (kind filterImport) (ordinal 0) (authored-target "DesignModel")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/filtering.md") (range (start 24 20) (end 24 35)) (probe (position 24 20))
    (reference (id (source (node (document "memory://snapshot/filtering.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "DesignModel")
      (outcome (status unsupported)))
  )
)
~~~
