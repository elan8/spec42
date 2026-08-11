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
# FORMAT
~~~sysml
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "5a5b5d9122046ade2aa8f7bc453497fc9c2f6d22975bae72be273f8851dfb250") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Filtering"))) (kind "package") (name "Filtering") (declared-name "Filtering") (range (start (line 0) (character 0)) (end (line 0) (character 1084))))
    (element (id (node (document "d0") (qualified-name "Filtering::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "Filtering"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Filtering::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 38) (character 2)) (end (line 38) (character 26))) (parent (node (document "d0") (qualified-name "Filtering"))) (authored (membership (kind Import) (visibility "private") (import (reference "KerML::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 38) (character 17)) (end (line 38) (character 22))))))
    (element (id (node (document "d0") (qualified-name "Filtering::Annotations"))) (kind "package") (name "Annotations") (declared-name "Annotations") (range (start (line 3) (character 1)) (end (line 3) (character 127))) (parent (node (document "d0") (qualified-name "Filtering"))))
    (element (id (node (document "d0") (qualified-name "Filtering::Annotations::ApprovalAnnotation"))) (kind "kermlDecl") (name "ApprovalAnnotation") (declared-name "ApprovalAnnotation") (range (start (line 4) (character 2)) (end (line 4) (character 101))) (parent (node (document "d0") (qualified-name "Filtering::Annotations"))))
    (element (id (node (document "d0") (qualified-name "Filtering::DesignModel"))) (kind "package") (name "DesignModel") (declared-name "DesignModel") (range (start (line 11) (character 1)) (end (line 11) (character 261))) (parent (node (document "d0") (qualified-name "Filtering"))))
    (element (id (node (document "d0") (qualified-name "Filtering::DesignModel::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 12) (character 5)) (end (line 12) (character 35))) (parent (node (document "d0") (qualified-name "Filtering::DesignModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "Annotations::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 12) (character 20)) (end (line 12) (character 31))))))
    (element (id (node (document "d0") (qualified-name "Filtering::DesignModel::System"))) (kind "classifier decl") (name "System") (declared-name "System") (range (start (line 13) (character 5)) (end (line 13) (character 162))) (parent (node (document "d0") (qualified-name "Filtering::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "Filtering::Meta"))) (kind "package") (name "Meta") (declared-name "Meta") (range (start (line 39) (character 1)) (end (line 39) (character 198))) (parent (node (document "d0") (qualified-name "Filtering"))))
    (element (id (node (document "d0") (qualified-name "Filtering::Meta::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 40) (character 2)) (end (line 40) (character 32))) (parent (node (document "d0") (qualified-name "Filtering::Meta"))) (authored (membership (kind Import) (visibility "private") (import (reference "DesignModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 40) (character 17)) (end (line 40) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Filtering::Meta::Test"))) (kind "classifier decl") (name "Test") (declared-name "Test") (range (start (line 44) (character 2)) (end (line 44) (character 24))) (parent (node (document "d0") (qualified-name "Filtering::Meta"))))
    (element (id (node (document "d0") (qualified-name "Filtering::Meta::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (range (start (line 41) (character 2)) (end (line 41) (character 96))) (parent (node (document "d0") (qualified-name "Filtering::Meta"))))
    (element (id (node (document "d0") (qualified-name "Filtering::Meta::feature"))) (kind "feature decl") (name "feature") (declared-name "feature") (range (start (line 45) (character 2)) (end (line 45) (character 20))) (parent (node (document "d0") (qualified-name "Filtering::Meta"))))
    (element (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals"))) (kind "package") (name "UpperLevelApprovals") (declared-name "UpperLevelApprovals") (range (start (line 23) (character 1)) (end (line 23) (character 219))) (parent (node (document "d0") (qualified-name "Filtering"))))
    (element (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1"))) (kind "package") (name "UpperLevelApprovals1") (declared-name "UpperLevelApprovals1") (range (start (line 31) (character 1)) (end (line 31) (character 183))) (parent (node (document "d0") (qualified-name "Filtering"))))
    (element (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1::Annotations"))) (kind "import") (name "Annotations") (declared-name "Annotations") (range (start (line 32) (character 5)) (end (line 32) (character 36))) (parent (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1"))) (authored (membership (kind Import) (visibility "private") (import (reference "Annotations") (origin Import) (shape Membership) (recursive true)) (import-range (start (line 32) (character 20)) (end (line 32) (character 31))))))
    (element (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1::DesignModel"))) (kind "import") (name "DesignModel") (declared-name "DesignModel") (range (start (line 33) (character 5)) (end (line 33) (character 72))) (parent (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1"))) (authored (membership (kind Import) (visibility "private") (import (reference "DesignModel") (origin Import) (shape FilteredNamespace) (recursive true)) (import-range (start (line 33) (character 20)) (end (line 33) (character 31))))))
    (element (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1::Test"))) (kind "classifier decl") (name "Test") (declared-name "Test") (range (start (line 35) (character 5)) (end (line 35) (character 27))) (parent (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1"))))
    (element (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals::DesignModel"))) (kind "import") (name "DesignModel") (declared-name "DesignModel") (range (start (line 24) (character 5)) (end (line 24) (character 36))) (parent (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals"))) (authored (membership (kind Import) (visibility "private") (import (reference "DesignModel") (origin Import) (shape Membership) (recursive true)) (import-range (start (line 24) (character 20)) (end (line 24) (character 31))))))
    (element (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals::Test"))) (kind "classifier decl") (name "Test") (declared-name "Test") (range (start (line 28) (character 5)) (end (line 28) (character 27))) (parent (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals"))))
    (element (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (range (start (line 25) (character 5)) (end (line 25) (character 114))) (parent (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Filtering::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "KerML::*") (range (start (line 38) (character 17)) (end (line 38) (character 22))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering::DesignModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Annotations::*") (range (start (line 12) (character 20)) (end (line 12) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering::Meta::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "DesignModel::*") (range (start (line 40) (character 17)) (end (line 40) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1::Annotations"))) (kind membershipImport) (ordinal 0)) (authored-target "Annotations") (range (start (line 32) (character 20)) (end (line 32) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1::DesignModel"))) (kind namespaceImport) (ordinal 0)) (authored-target "DesignModel") (range (start (line 33) (character 20)) (end (line 33) (character 31))) (outcome (status unsupported-filtered)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals::DesignModel"))) (kind membershipImport) (ordinal 0)) (authored-target "DesignModel") (range (start (line 24) (character 20)) (end (line 24) (character 31))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Filtering::Meta::_filter")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals::_filter")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
