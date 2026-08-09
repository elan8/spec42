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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwMetaclass,Ident,OpenCurly,
Ident,Colon,Ident,Semicolon,
Ident,Colon,Ident,Semicolon,
Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwStruct,Ident,OpenCurly,
At,Ident,OpenCurly,
Ident,Eq,KwTrue,Semicolon,
Ident,Eq,StringValue,Semicolon,
Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
KwComposite,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,StarStar,Semicolon,
KwFilter,Ident,ColonColon,Ident,ColonColon,Ident,KwAnd,
Ident,ColonColon,Ident,ColonColon,Ident,CloseAngle,DecimalValue,Semicolon,
KwStruct,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,StarStar,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,StarStar,OpenSquare,At,Ident,CloseSquare,OpenSquare,Ident,KwAnd,Ident,CloseAngle,DecimalValue,CloseSquare,Semicolon,
KwStruct,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwFilter,OpenParen,Ident,ColonColon,Ident,EqEq,StringValue,KwAnd,KwNot,Ident,ColonColon,Ident,CloseParen,KwOr,
Ident,ColonColon,Ident,Semicolon,
KwStruct,Ident,ColonGt,Ident,Semicolon,
KwFeature,ColonGt,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Filtering'
    (import_decl private 'ScalarValues::*')
    (package_def 'Annotations'
      (metaclass_def 'ApprovalAnnotation'
        (feature_def 'approved' : 'Boolean')
        (feature_def 'approver' : 'String')
        (feature_def 'level' : 'Natural')))
    (package_def 'DesignModel'
      (import_decl private 'Annotations::*')
      (structure_def 'System'
        (metadata_feature typed 'ApprovalAnnotation'
          (feature_def 'approved' value)
          (feature_def 'approver' value)
          (feature_def 'level' value)))
      (feature_def composite 'system' : 'System'))
    (package_def 'UpperLevelApprovals'
      (import_decl private 'DesignModel::**')
      (filter_member
        (binary_expr))
      (structure_def 'Test' :> 'System'))
    (package_def 'UpperLevelApprovals1'
      (import_decl private 'Annotations::**')
      (import_decl private 'DesignModel::**')
      (structure_def 'Test' :> 'System'))
    (import_decl private 'KerML::*')
    (package_def 'Meta'
      (import_decl private 'DesignModel::*')
      (filter_member
        (binary_expr))
      (structure_def 'Test' :> 'System')
      (feature_def :> 'system'))))
~~~
# FORMAT
~~~sysml
package Filtering {
    private import ScalarValues::*;

    package Annotations {
        metaclass ApprovalAnnotation {
            approved: Boolean;
            approver: String;
            level: Natural;
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
        private import DesignModel::**;

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
# EXPECTED
~~~
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Natural'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Natural'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Filtering"))) (name "Filtering") (declared-name "Filtering")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Filtering::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Filtering::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "Filtering::Annotations"))) (name "Annotations") (declared-name "Annotations")
          (contains
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Filtering::Annotations::ApprovalAnnotation"))) (name "ApprovalAnnotation") (declared-name "ApprovalAnnotation"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "Filtering::DesignModel"))) (name "DesignModel") (declared-name "DesignModel")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "Filtering::DesignModel::*"))) (name "*") (declared-name "*"))
            (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Filtering::DesignModel::System"))) (name "System") (declared-name "System"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "Filtering::Meta"))) (name "Meta") (declared-name "Meta")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "Filtering::Meta::*"))) (name "*") (declared-name "*"))
            (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Filtering::Meta::Test"))) (name "Test") (declared-name "Test"))
            (element (kind "filter") (id (node (document "d0") (qualified-name "Filtering::Meta::_filter"))) (name "_filter") (declared-name "_filter") (declared (own-expression (expression (kind "binary") (operator "||") (children (expression (kind "parenthesized") (children (expression (kind "binary") (operator "&&") (children (expression (kind "binary") (operator "==") (children (expression (kind "featureReference") (reference "Element::name")) (expression (kind "stringLiteral") (literal "System")))) (expression (kind "unary") (operator "not") (children (expression (kind "featureReference") (reference "Type::isAbstract")))))))) (expression (kind "featureReference") (reference "Feature::isComposite")))))))
            (element (kind "feature decl") (id (node (document "d0") (qualified-name "Filtering::Meta::feature"))) (name "feature") (declared-name "feature"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals"))) (name "UpperLevelApprovals") (declared-name "UpperLevelApprovals")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals::DesignModel"))) (name "DesignModel") (declared-name "DesignModel"))
            (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals::Test"))) (name "Test") (declared-name "Test"))
            (element (kind "filter") (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals::_filter"))) (name "_filter") (declared-name "_filter") (declared (own-expression (expression (kind "binary") (operator "&&") (children (expression (kind "featureReference") (reference "Annotations::ApprovalAnnotation::approved")) (expression (kind "binary") (operator ">") (children (expression (kind "featureReference") (reference "Annotations::ApprovalAnnotation::level")) (expression (kind "integerLiteral") (literal 1)))))))))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1"))) (name "UpperLevelApprovals1") (declared-name "UpperLevelApprovals1")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1::Annotations"))) (name "Annotations") (declared-name "Annotations"))
            (element (kind "import") (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1::DesignModel"))) (name "DesignModel") (declared-name "DesignModel"))
            (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Filtering::UpperLevelApprovals1::Test"))) (name "Test") (declared-name "Test"))
          )
        )
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
