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
(model
  (namespace
    (package 'Filtering'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (package 'Annotations'
        (metaclass_def 'ApprovalAnnotation'
          (feature_def 'approved' : 'Boolean'[unresolved])
          (feature_def 'approver' : 'String'[unresolved])
          (feature_def 'level' : 'Natural'[unresolved])))
      (package 'DesignModel'
        (namespace_import private -> 'Filtering::Annotations'[package])
        (structure_def 'System'
          (metadata_usage :> 'Filtering::Annotations::ApprovalAnnotation'[metaclass_def]
            (feature_def 'approved' :>> 'Filtering::Annotations::ApprovalAnnotation::approved'[feature_def][implied]
              (feature_value (=)))
            (feature_def 'approver' :>> 'Filtering::Annotations::ApprovalAnnotation::approver'[feature_def][implied]
              (feature_value (=)))
            (feature_def 'level' :>> 'Filtering::Annotations::ApprovalAnnotation::level'[feature_def][implied]
              (feature_value (=)))))
        (feature_def composite 'system' : 'Filtering::DesignModel::System'[structure_def]))
      (package 'UpperLevelApprovals'
        (membership_import private recursive -> 'Filtering::DesignModel'[package])
        (element_filter_membership)
        (structure_def 'Test' :> 'Filtering::DesignModel::System'[structure_def]))
      (package 'UpperLevelApprovals1'
        (membership_import private recursive -> 'Filtering::Annotations'[package])
        (membership_import private recursive -> 'Filtering::DesignModel'[package])
        (structure_def 'Test' :> 'Filtering::DesignModel::System'[structure_def]))
      (namespace_import private -> 'KerML'[unresolved])
      (package 'Meta'
        (namespace_import private -> 'Filtering::DesignModel'[package])
        (element_filter_membership)
        (structure_def 'Test' :> 'Filtering::DesignModel::System'[structure_def])
        (feature_def :> 'Filtering::DesignModel::system'[feature_def])))))
~~~
