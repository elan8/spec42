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
  (document "filtering.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 20) (end 12 31))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 20 2) (end 20 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 24 20) (end 24 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 32 20) (end 32 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 33 20) (end 33 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 38 17) (end 38 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 40 17) (end 40 28))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "5a5b5d9122046ade2aa8f7bc453497fc9c2f6d22975bae72be273f8851dfb250") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Filtering"))) (kind "package") (name "Filtering") (declared-name "Filtering"))
    (element (id (node (document "d0") (qualified-name "Filtering::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Filtering"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Filtering::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Filtering"))) (authored (membership (kind Import) (visibility "private") (import (reference "KerML::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Filtering::Annotations"))) (kind "package") (name "Annotations") (declared-name "Annotations") (parent (node (document "d0") (qualified-name "Filtering"))))
    (element (id (node (document "d0") (qualified-name "Filtering::Annotations::ApprovalAnnotation"))) (kind "kermlDecl") (name "ApprovalAnnotation") (declared-name "ApprovalAnnotation") (parent (node (document "d0") (qualified-name "Filtering::Annotations"))))
    (element (id (node (document "d0") (qualified-name "Filtering::DesignModel"))) (kind "package") (name "DesignModel") (declared-name "DesignModel") (parent (node (document "d0") (qualified-name "Filtering"))))
    (element (id (node (document "d0") (qualified-name "Filtering::DesignModel::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Filtering::DesignModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "Annotations::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Filtering::DesignModel::System"))) (kind "classifier decl") (name "System") (declared-name "System") (parent (node (document "d0") (qualified-name "Filtering::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "Filtering::Meta"))) (kind "package") (name "Meta") (declared-name "Meta") (parent (node (document "d0") (qualified-name "Filtering"))))
    (element (id (node (document "d0") (qualified-name "Filtering::Meta::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Filtering::Meta"))) (authored (membership (kind Import) (visibility "private") (import (reference "DesignModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Filtering::Meta::Test"))) (kind "classifier decl") (name "Test") (declared-name "Test") (parent (node (document "d0") (qualified-name "Filtering::Meta"))))
    (element (id (node (document "d0") (qualified-name "Filtering::Meta::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (parent (node (document "d0") (qualified-name "Filtering::Meta"))))
    (element (id (node (document "d0") (qualified-name "Filtering::Meta::feature"))) (kind "feature decl") (name "feature") (declared-name "feature") (parent (node (document "d0") (qualified-name "Filtering::Meta"))))
    (element (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals"))) (kind "package") (name "UpperLevelApprovals") (declared-name "UpperLevelApprovals") (parent (node (document "d0") (qualified-name "Filtering"))))
    (element (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1"))) (kind "package") (name "UpperLevelApprovals1") (declared-name "UpperLevelApprovals1") (parent (node (document "d0") (qualified-name "Filtering"))))
    (element (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1::Annotations"))) (kind "import") (name "Annotations") (declared-name "Annotations") (parent (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1"))) (authored (membership (kind Import) (visibility "private") (import (reference "Annotations") (origin Import) (shape Membership) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1::DesignModel"))) (kind "import") (name "DesignModel") (declared-name "DesignModel") (parent (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1"))) (authored (membership (kind Import) (visibility "private") (import (reference "DesignModel") (origin Import) (shape FilteredNamespace) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1::Test"))) (kind "classifier decl") (name "Test") (declared-name "Test") (parent (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1"))))
    (element (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals::DesignModel"))) (kind "import") (name "DesignModel") (declared-name "DesignModel") (parent (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals"))) (authored (membership (kind Import) (visibility "private") (import (reference "DesignModel") (origin Import) (shape Membership) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals::Test"))) (kind "classifier decl") (name "Test") (declared-name "Test") (parent (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals"))))
    (element (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (parent (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Filtering::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "KerML::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering::DesignModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Annotations::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering::Meta::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "DesignModel::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1::Annotations"))) (kind membershipImport) (ordinal 0)) (authored-target "Annotations") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive true) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1::DesignModel"))) (kind namespaceImport) (ordinal 0)) (authored-target "DesignModel") (outcome (status unsupported-filtered)) (import (origin import) (shape filtered-namespace) (recursive true) (conformance not-checked-unsupported-filtered)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals::DesignModel"))) (kind membershipImport) (ordinal 0)) (authored-target "DesignModel") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive true) (conformance not-checked-unresolved)))
  )
  (relationships
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Filtering::Meta::_filter")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals::_filter")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 38 17) (end 38 22)) (probe (position 38 17))
      (reference
        (source (document "d0") (qualified-name "Filtering::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "KerML::*")
        (range (start 38 17) (end 38 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 20) (end 12 31)) (probe (position 12 20))
      (reference
        (source (document "d0") (qualified-name "Filtering::DesignModel::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Annotations::*")
        (range (start 12 20) (end 12 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 24 20) (end 24 31)) (probe (position 24 20))
      (reference
        (source (document "d0") (qualified-name "Filtering::UpperLevelApprovals::DesignModel"))
        (kind membershipImport) (ordinal 0) (authored-target "DesignModel")
        (range (start 24 20) (end 24 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 32 20) (end 32 31)) (probe (position 32 20))
      (reference
        (source (document "d0") (qualified-name "Filtering::UpperLevelApprovals1::Annotations"))
        (kind membershipImport) (ordinal 0) (authored-target "Annotations")
        (range (start 32 20) (end 32 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 20) (end 33 31)) (probe (position 33 20))
      (reference
        (source (document "d0") (qualified-name "Filtering::UpperLevelApprovals1::DesignModel"))
        (kind namespaceImport) (ordinal 0) (authored-target "DesignModel")
        (range (start 33 20) (end 33 31))
        (outcome (status unsupported-filtered))
      )
    )
    (query (range (start 40 17) (end 40 28)) (probe (position 40 17))
      (reference
        (source (document "d0") (qualified-name "Filtering::Meta::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "DesignModel::*")
        (range (start 40 17) (end 40 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Filtering::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
